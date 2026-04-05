use crate::gff4::header::{read_raw_header, real_version, Header};
use crate::gff4::schema::{resolve_header, BaseType, FieldDef, ResolvedHeader, ValueType};
use crate::gff4::value::{FieldValue, GffStruct, Value};
use crate::gff4::writer;
use std::fs;
use std::io;
use std::path::Path;

const TYPE_ID_ECSTRING: u16 = 14;
const NULLPTR: u32 = 0xFFFF_FFFF;

#[derive(Debug, Clone)]
pub struct GffFile {
    pub header: ResolvedHeader,
    pub root: GffStruct,
}

impl GffFile {
    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        Reader::from_file(path)?.read()
    }

    pub fn from_bytes(bytes: Vec<u8>) -> io::Result<Self> {
        Reader::from_bytes(bytes)?.read()
    }

    pub fn root(&self) -> &GffStruct {
        &self.root
    }

    pub fn root_mut(&mut self) -> &mut GffStruct {
        &mut self.root
    }

    pub fn to_bytes(&self) -> io::Result<Vec<u8>> {
        writer::write_to_bytes(self)
    }

    pub fn write_to_path(&self, path: impl AsRef<Path>) -> io::Result<()> {
        writer::write_to_path(self, path)
    }
}

#[derive(Debug, Clone, Copy)]
struct GenericRef {
    type_id: u16,
    is_list: bool,
    is_struct: bool,
    is_reference: bool,
    address: u32,
}

pub struct Reader {
    bytes: Vec<u8>,
    header: ResolvedHeader,
    big_endian: bool,
    use_cstring: bool,
    string_cache: Vec<String>,
    _raw_header: Header,
}

impl Reader {
    pub fn from_bytes(bytes: Vec<u8>) -> io::Result<Self> {
        let mut cursor = std::io::Cursor::new(&bytes);
        let raw_header = read_raw_header(&mut cursor)?;
        let header = resolve_header(&raw_header)?;

        let big_endian = &header.platform != b"PC  ";
        let effective_version = real_version(&header.version, &header.platform);
        let use_cstring = &effective_version >= b"V4.1";

        let string_cache = if use_cstring {
            Self::read_string_table(&bytes, &header)?
        } else {
            Vec::new()
        };

        Ok(Self {
            bytes,
            header,
            big_endian,
            use_cstring,
            string_cache,
            _raw_header: raw_header,
        })
    }

    pub fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let bytes = fs::read(path)?;
        Self::from_bytes(bytes)
    }

    pub fn header(&self) -> &ResolvedHeader {
        &self.header
    }

    pub fn read(self) -> io::Result<GffFile> {
        if self.header.structs.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "header contains no structs",
            ));
        }

        let root = self.read_struct(0, self.header.data_offset as usize)?;
        Ok(GffFile {
            header: self.header,
            root,
        })
    }

    fn read_string_table(bytes: &[u8], header: &ResolvedHeader) -> io::Result<Vec<String>> {
        let start = header.string_offset as usize;
        let end = header.data_offset as usize;

        if start > end || end > bytes.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid string table bounds",
            ));
        }

        let raw = &bytes[start..end];
        let mut strings = Vec::new();

        for part in raw.split(|b| *b == 0) {
            if strings.len() >= header.string_count as usize {
                break;
            }

            let s = String::from_utf8(part.to_vec()).map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "invalid utf-8 in string table")
            })?;
            strings.push(s);
        }

        if strings.len() != header.string_count as usize {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "string table count mismatch: expected {}, got {}",
                    header.string_count,
                    strings.len()
                ),
            ));
        }

        Ok(strings)
    }

    fn read_struct(&self, struct_index: usize, base_offset: usize) -> io::Result<GffStruct> {
        let struct_def = self
            .header
            .structs
            .get(struct_index)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "invalid struct index"))?;

        let mut fields = Vec::with_capacity(struct_def.fields.len());

        for field in &struct_def.fields {
            let value = self.read_field(field, base_offset)?;
            fields.push(FieldValue {
                label: field.label,
                value,
            });
        }

        Ok(GffStruct {
            struct_index,
            fields,
        })
    }

    fn read_field(&self, field: &FieldDef, struct_base: usize) -> io::Result<Value> {
        let field_offset = struct_base
            .checked_add(field.offset as usize)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "field offset overflow"))?;

        if field.is_list {
            return self.read_list(&field.base, field.is_reference, field_offset);
        }

        if field.is_reference {
            return self.read_reference(&field.base, field_offset);
        }

        match field.base {
            BaseType::Primitive(ty) => self.read_primitive_value(ty, field_offset),
            BaseType::Struct(struct_index) => {
                let child = self.read_struct(struct_index, field_offset)?;
                Ok(Value::Struct(Box::new(child)))
            }
            BaseType::Generic => Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "non-reference generic field is invalid",
            )),
        }
    }

    fn read_list(
        &self,
        base: &BaseType,
        indirect: bool,
        field_offset: usize,
    ) -> io::Result<Value> {
        let address = self.read_u32(field_offset)?;
        if address == NULLPTR {
            return Ok(Value::List(Vec::new()));
        }

        let list_base = self.data_abs(address)?;
        let length = self.read_u32(list_base)? as usize;
        let elem_base = list_base
            .checked_add(4)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "list offset overflow"))?;

        if indirect {
            self.read_indirect_list(base, length, elem_base)
        } else {
            self.read_inline_list(base, length, elem_base)
        }
    }

    fn read_indirect_list(
        &self,
        base: &BaseType,
        length: usize,
        elem_base: usize,
    ) -> io::Result<Value> {
        match base {
            BaseType::Generic => {
                let mut items = Vec::with_capacity(length);

                for i in 0..length {
                    let off = elem_base.checked_add(i * 8).ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "list element overflow")
                    })?;
                    items.push(self.read_generic_list_element(off)?);
                }

                Ok(Value::List(items))
            }
            BaseType::Struct(struct_index) => {
                let mut items = Vec::with_capacity(length);

                for i in 0..length {
                    let ref_off = elem_base.checked_add(i * 4).ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "list element overflow")
                    })?;
                    let addr = self.read_u32(ref_off)?;

                    if addr == NULLPTR {
                        items.push(Value::Null);
                    } else {
                        let abs = self.data_abs(addr)?;
                        let s = self.read_struct(*struct_index, abs)?;
                        items.push(Value::Struct(Box::new(s)));
                    }
                }

                Ok(Value::List(items))
            }
            BaseType::Primitive(ty) => {
                let mut items = Vec::with_capacity(length);

                for i in 0..length {
                    let ref_off = elem_base.checked_add(i * 4).ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "list element overflow")
                    })?;
                    let addr = self.read_u32(ref_off)?;

                    if addr == NULLPTR {
                        items.push(Value::Null);
                    } else if *ty == ValueType::ECString {
                        if self.use_cstring {
                            let s = self
                                .string_cache
                                .get(addr as usize)
                                .ok_or_else(|| {
                                    io::Error::new(
                                        io::ErrorKind::InvalidData,
                                        "invalid string index",
                                    )
                                })?
                                .clone();
                            items.push(Value::ECString(s));
                        } else {
                            let abs = self.data_abs(addr)?;
                            let s = self.read_utf16_string(abs)?;
                            items.push(Value::ECString(s));
                        }
                    } else {
                        let abs = self.data_abs(addr)?;
                        items.push(self.read_primitive_value(*ty, abs)?);
                    }
                }

                Ok(Value::List(items))
            }
        }
    }

    fn read_inline_list(
        &self,
        base: &BaseType,
        length: usize,
        elem_base: usize,
    ) -> io::Result<Value> {
        match base {
            BaseType::Generic => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "inline generic list is invalid",
            )),
            BaseType::Struct(struct_index) => {
                let struct_def = self.header.structs.get(*struct_index).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "invalid struct index")
                })?;

                let elem_size = struct_def.size as usize;
                let mut items = Vec::with_capacity(length);

                for i in 0..length {
                    let off = elem_base.checked_add(i * elem_size).ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "list element overflow")
                    })?;
                    let s = self.read_struct(*struct_index, off)?;
                    items.push(Value::Struct(Box::new(s)));
                }

                Ok(Value::List(items))
            }
            BaseType::Primitive(ValueType::UInt8) => {
                let end = elem_base.checked_add(length).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "binary length overflow")
                })?;
                let bytes = self.slice(elem_base, end)?.to_vec();
                Ok(Value::Binary(bytes))
            }
            BaseType::Primitive(ty) => {
                let elem_size = ty.size();
                let mut items = Vec::with_capacity(length);

                for i in 0..length {
                    let off = elem_base.checked_add(i * elem_size).ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "list element overflow")
                    })?;
                    items.push(self.read_primitive_value(*ty, off)?);
                }

                Ok(Value::List(items))
            }
        }
    }

    fn read_reference(&self, base: &BaseType, offset: usize) -> io::Result<Value> {
        match base {
            BaseType::Generic => self.read_generic_reference(offset),
            BaseType::Struct(struct_index) => {
                let address = self.read_u32(offset)?;
                if address == NULLPTR {
                    return Ok(Value::Null);
                }

                let abs = self.data_abs(address)?;
                let value = self.read_struct(*struct_index, abs)?;
                Ok(Value::Struct(Box::new(value)))
            }
            BaseType::Primitive(ty) => {
                if *ty == ValueType::ECString {
                    let address = self.read_u32(offset)?;
                    if address == NULLPTR {
                        return Ok(Value::Null);
                    }

                    if self.use_cstring {
                        let s = self
                            .string_cache
                            .get(address as usize)
                            .ok_or_else(|| {
                                io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    "invalid string index",
                                )
                            })?
                            .clone();
                        return Ok(Value::ECString(s));
                    }

                    let abs = self.data_abs(address)?;
                    let s = self.read_utf16_string(abs)?;
                    Ok(Value::ECString(s))
                } else {
                    let address = self.read_u32(offset)?;
                    if address == NULLPTR {
                        return Ok(Value::Null);
                    }

                    let abs = self.data_abs(address)?;
                    self.read_primitive_value(*ty, abs)
                }
            }
        }
    }

    fn read_generic_reference(&self, offset: usize) -> io::Result<Value> {
        let g = self.read_generic(offset)?;

        if g.address == NULLPTR {
            return Ok(Value::Null);
        }

        if g.is_list || g.is_reference {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "generic list/reference values not implemented",
            ));
        }

        if g.is_struct {
            let struct_index = g.type_id as usize;
            if struct_index >= self.header.structs.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("generic references invalid struct index {}", struct_index),
                ));
            }

            let abs = self.data_abs(g.address)?;
            let value = self.read_struct(struct_index, abs)?;
            Ok(Value::Struct(Box::new(value)))
        } else if g.type_id == TYPE_ID_ECSTRING {
            let s = if self.use_cstring {
                self.string_cache
                    .get(g.address as usize)
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "invalid string index")
                    })?
                    .clone()
            } else {
                let abs = self.data_abs(g.address)?;
                self.read_utf16_string(abs)?
            };
            Ok(Value::ECString(s))
        } else {
            let abs = self.data_abs(g.address)?;
            let ty = ValueType::from_type_id(g.type_id).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unknown generic type id {}", g.type_id),
                )
            })?;
            self.read_primitive_value(ty, abs)
        }
    }

    fn read_generic_list_element(&self, offset: usize) -> io::Result<Value> {
        let g = self.read_generic(offset)?;

        if g.is_list || g.is_reference {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "generic list element cannot itself be list/reference",
            ));
        }

        if g.address == NULLPTR {
            return Ok(Value::Null);
        }

        if g.is_struct {
            let struct_index = g.type_id as usize;
            if struct_index >= self.header.structs.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("generic references invalid struct index {}", struct_index),
                ));
            }

            let abs = self.data_abs(g.address)?;
            let s = self.read_struct(struct_index, abs)?;
            Ok(Value::Struct(Box::new(s)))
        } else if g.type_id == TYPE_ID_ECSTRING {
            let s = if self.use_cstring {
                self.string_cache
                    .get(g.address as usize)
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "invalid string index")
                    })?
                    .clone()
            } else {
                let abs = self.data_abs(g.address)?;
                self.read_utf16_string(abs)?
            };
            Ok(Value::ECString(s))
        } else {
            let abs = self.data_abs(g.address)?;
            let ty = ValueType::from_type_id(g.type_id).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unknown generic type id {}", g.type_id),
                )
            })?;
            self.read_primitive_value(ty, abs)
        }
    }

    fn read_generic(&self, offset: usize) -> io::Result<GenericRef> {
        let type_and_flags = self.read_u32(offset)?;
        let address = self.read_u32(offset + 4)?;

        let flags = (type_and_flags >> 16) as u16;
        let type_id = (type_and_flags & 0xFFFF) as u16;

        if flags & 0x1FFF != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown generic flag bits: {:04x}", flags & 0x1FFF),
            ));
        }

        Ok(GenericRef {
            type_id,
            is_list: flags & 0x8000 != 0,
            is_struct: flags & 0x4000 != 0,
            is_reference: flags & 0x2000 != 0,
            address,
        })
    }

    fn read_primitive_value(&self, ty: ValueType, offset: usize) -> io::Result<Value> {
        match ty {
            ValueType::UInt8 => Ok(Value::UInt8(self.read_u8(offset)?)),
            ValueType::Int8 => Ok(Value::Int8(self.read_i8(offset)?)),
            ValueType::UInt16 => Ok(Value::UInt16(self.read_u16(offset)?)),
            ValueType::Int16 => Ok(Value::Int16(self.read_i16(offset)?)),
            ValueType::UInt32 => Ok(Value::UInt32(self.read_u32(offset)?)),
            ValueType::Int32 => Ok(Value::Int32(self.read_i32(offset)?)),
            ValueType::UInt64 => Ok(Value::UInt64(self.read_u64(offset)?)),
            ValueType::Int64 => Ok(Value::Int64(self.read_i64(offset)?)),
            ValueType::Float32 => Ok(Value::Float32(self.read_f32(offset)?)),
            ValueType::Float64 => Ok(Value::Float64(self.read_f64(offset)?)),
            ValueType::Vector3f => Ok(Value::Vector3f([
                self.read_f32(offset)?,
                self.read_f32(offset + 4)?,
                self.read_f32(offset + 8)?,
            ])),
            ValueType::Vector4f => Ok(Value::Vector4f([
                self.read_f32(offset)?,
                self.read_f32(offset + 4)?,
                self.read_f32(offset + 8)?,
                self.read_f32(offset + 12)?,
            ])),
            ValueType::Quaternionf => Ok(Value::Quaternionf([
                self.read_f32(offset)?,
                self.read_f32(offset + 4)?,
                self.read_f32(offset + 8)?,
                self.read_f32(offset + 12)?,
            ])),
            ValueType::Color4f => Ok(Value::Color4f([
                self.read_f32(offset)?,
                self.read_f32(offset + 4)?,
                self.read_f32(offset + 8)?,
                self.read_f32(offset + 12)?,
            ])),
            ValueType::Matrix4x4f => {
                let mut arr = [0.0f32; 16];
                for (i, item) in arr.iter_mut().enumerate() {
                    *item = self.read_f32(offset + i * 4)?;
                }
                Ok(Value::Matrix4x4f(arr))
            }
            ValueType::ECString => self.read_ecstring(offset),
            ValueType::TlkString => self.read_tlkstring(offset),
        }
    }

    fn read_ecstring(&self, offset: usize) -> io::Result<Value> {
        let address = self.read_u32(offset)?;
        if address == NULLPTR {
            return Ok(Value::Null);
        }

        if self.use_cstring {
            let s = self
                .string_cache
                .get(address as usize)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "invalid string index")
                })?
                .clone();
            Ok(Value::ECString(s))
        } else {
            let abs = self.data_abs(address)?;
            let s = self.read_utf16_string(abs)?;
            Ok(Value::ECString(s))
        }
    }

    fn read_tlkstring(&self, offset: usize) -> io::Result<Value> {
        let label = self.read_u32(offset)?;
        let address = self.read_u32(offset + 4)?;

        if address == NULLPTR {
            return Ok(Value::TlkString {
                label,
                text: None,
                raw_zero: false,
            });
        }

        if self.use_cstring {
            let s = self
                .string_cache
                .get(address as usize)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "invalid string index")
                })?
                .clone();

            Ok(Value::TlkString {
                label,
                text: Some(s),
                raw_zero: false,
            })
        } else if address == 0 {
            Ok(Value::TlkString {
                label,
                text: None,
                raw_zero: true,
            })
        } else {
            let abs = self.data_abs(address)?;
            let s = self.read_utf16_string(abs)?;
            Ok(Value::TlkString {
                label,
                text: Some(s),
                raw_zero: false,
            })
        }
    }

    fn read_utf16_string(&self, offset: usize) -> io::Result<String> {
        let len = self.read_u32(offset)? as usize;
        let data_start = offset + 4;
        let data_end = data_start
            .checked_add(len * 2)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "utf16 length overflow"))?;

        let bytes = self.slice(data_start, data_end)?;
        let mut words = Vec::with_capacity(len);

        for chunk in bytes.chunks_exact(2) {
            let word = u16::from_le_bytes([chunk[0], chunk[1]]);
            words.push(word);
        }

        String::from_utf16(&words)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid utf-16 string"))
    }

    fn data_abs(&self, relative: u32) -> io::Result<usize> {
        let abs = self
            .header
            .data_offset
            .checked_add(relative)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "data offset overflow"))?;
        Ok(abs as usize)
    }

    fn slice(&self, start: usize, end: usize) -> io::Result<&[u8]> {
        self.bytes
            .get(start..end)
            .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "out of bounds read"))
    }

    fn read_u8(&self, offset: usize) -> io::Result<u8> {
        Ok(*self
            .bytes
            .get(offset)
            .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "u8 out of bounds"))?)
    }

    fn read_i8(&self, offset: usize) -> io::Result<i8> {
        Ok(self.read_u8(offset)? as i8)
    }

    fn read_u16(&self, offset: usize) -> io::Result<u16> {
        let b = self.slice(offset, offset + 2)?;
        Ok(if self.big_endian {
            u16::from_be_bytes([b[0], b[1]])
        } else {
            u16::from_le_bytes([b[0], b[1]])
        })
    }

    fn read_i16(&self, offset: usize) -> io::Result<i16> {
        Ok(self.read_u16(offset)? as i16)
    }

    fn read_u32(&self, offset: usize) -> io::Result<u32> {
        let b = self.slice(offset, offset + 4)?;
        Ok(if self.big_endian {
            u32::from_be_bytes([b[0], b[1], b[2], b[3]])
        } else {
            u32::from_le_bytes([b[0], b[1], b[2], b[3]])
        })
    }

    fn read_i32(&self, offset: usize) -> io::Result<i32> {
        Ok(self.read_u32(offset)? as i32)
    }

    fn read_u64(&self, offset: usize) -> io::Result<u64> {
        let b = self.slice(offset, offset + 8)?;
        Ok(if self.big_endian {
            u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
        } else {
            u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
        })
    }

    fn read_i64(&self, offset: usize) -> io::Result<i64> {
        Ok(self.read_u64(offset)? as i64)
    }

    fn read_f32(&self, offset: usize) -> io::Result<f32> {
        Ok(f32::from_bits(self.read_u32(offset)?))
    }

    fn read_f64(&self, offset: usize) -> io::Result<f64> {
        Ok(f64::from_bits(self.read_u64(offset)?))
    }
}
