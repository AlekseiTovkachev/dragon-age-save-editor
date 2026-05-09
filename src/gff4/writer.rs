use crate::gff4::GffFile;
use crate::gff4::header::real_version;
use crate::gff4::schema::{BaseType, ResolvedHeader, StructDef, ValueType};
use crate::gff4::value::{GffStruct, Value};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

const NULLPTR: u32 = 0xFFFF_FFFF;

pub fn write_to_bytes(file: &GffFile) -> io::Result<Vec<u8>> {
    Writer::new(file)?.write()
}

pub fn write_to_path(file: &GffFile, path: impl AsRef<Path>) -> io::Result<()> {
    let bytes = write_to_bytes(file)?;
    fs::write(path, bytes)
}

struct Writer<'a> {
    file: &'a GffFile,
    header: &'a ResolvedHeader,
    big_endian: bool,
    use_cstring: bool,
    string_section: Vec<u8>,
    string_cache: HashMap<String, u32>,
    data_section: Vec<u8>,
}

impl<'a> Writer<'a> {
    fn new(file: &'a GffFile) -> io::Result<Self> {
        if file.root.struct_index >= file.header.structs.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "root struct index is out of bounds",
            ));
        }

        let effective_version = real_version(&file.header.version, &file.header.platform);
        let use_cstring = &effective_version >= b"V4.1";
        let big_endian = &file.header.platform != b"PC  ";

        let mut writer = Self {
            file,
            header: &file.header,
            big_endian,
            use_cstring,
            string_section: Vec::new(),
            string_cache: HashMap::new(),
            data_section: Vec::new(),
        };

        if use_cstring {
            writer.string_cache.insert(String::new(), 0);
            writer.string_section.push(0);
        }

        Ok(writer)
    }

    fn write(mut self) -> io::Result<Vec<u8>> {
        let root_size = self.struct_def(self.file.root.struct_index)?.size as usize;
        let root_offset = self.allocate(root_size, 4);
        if root_offset != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "root structure did not allocate at offset 0",
            ));
        }

        self.write_struct(&self.file.root, 0)?;

        let struct_count = self.header.structs.len() as u32;
        let field_count: usize = self.header.structs.iter().map(|s| s.fields.len()).sum();
        let struct_offset = 12u32 + if self.use_cstring { 24 } else { 16 };
        let field_offset = struct_offset + struct_count * 16;

        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"GFF ");
        bytes.extend_from_slice(&self.header.version);
        bytes.extend_from_slice(&self.header.platform);

        if self.use_cstring {
            let string_offset = field_offset + field_count as u32 * 12;
            let mut data_offset = string_offset + self.string_section.len() as u32;
            if data_offset % 16 != 0 {
                data_offset += 16 - data_offset % 16;
            }

            bytes.extend_from_slice(&self.header.file_type);
            bytes.extend_from_slice(&self.header.file_version);
            bytes.extend_from_slice(&self.pack_u32(struct_count));
            bytes.extend_from_slice(&self.pack_u32(self.string_cache.len() as u32));
            bytes.extend_from_slice(&self.pack_u32(string_offset));
            bytes.extend_from_slice(&self.pack_u32(data_offset));

            self.write_structure_table(&mut bytes, field_offset)?;
            self.write_field_table(&mut bytes)?;
            bytes.extend_from_slice(&self.string_section);
            while (bytes.len() as u32) < data_offset {
                bytes.push(0xFF);
            }
        } else {
            let mut data_offset = field_offset + field_count as u32 * 12;
            if data_offset % 16 != 0 {
                data_offset += 16 - data_offset % 16;
            }

            bytes.extend_from_slice(&self.header.file_type);
            bytes.extend_from_slice(&self.header.file_version);
            bytes.extend_from_slice(&self.pack_u32(struct_count));
            bytes.extend_from_slice(&self.pack_u32(data_offset));

            self.write_structure_table(&mut bytes, field_offset)?;
            self.write_field_table(&mut bytes)?;
            while (bytes.len() as u32) < data_offset {
                bytes.push(0xFF);
            }
        }

        bytes.extend_from_slice(&self.data_section);
        Ok(bytes)
    }

    fn write_structure_table(&self, bytes: &mut Vec<u8>, mut field_offset: u32) -> io::Result<()> {
        for struct_def in &self.header.structs {
            bytes.extend_from_slice(&struct_def.fourcc);
            bytes.extend_from_slice(&self.pack_u32(struct_def.fields.len() as u32));
            bytes.extend_from_slice(&self.pack_u32(field_offset));
            bytes.extend_from_slice(&self.pack_u32(struct_def.size));
            field_offset += struct_def.fields.len() as u32 * 12;
        }
        Ok(())
    }

    fn write_field_table(&self, bytes: &mut Vec<u8>) -> io::Result<()> {
        for struct_def in &self.header.structs {
            for field in &struct_def.fields {
                bytes.extend_from_slice(&self.pack_u32(field.label));
                let type_id = field_type_id(field)?;
                let flags = pack_flags(
                    field.is_list,
                    matches!(field.base, BaseType::Struct(_)),
                    field.is_reference,
                );
                let type_and_flags = ((flags as u32) << 16) | type_id as u32;
                bytes.extend_from_slice(&self.pack_u32(type_and_flags));
                bytes.extend_from_slice(&self.pack_u32(field.offset));
            }
        }
        Ok(())
    }

    fn write_struct(&mut self, structure: &GffStruct, offset: usize) -> io::Result<()> {
        let struct_def = self.struct_def(structure.struct_index)?.clone();
        for field in &struct_def.fields {
            let value = structure
                .fields
                .iter()
                .find(|item| item.label == field.label)
                .map(|item| &item.value)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("missing value for field {}", field.label),
                    )
                })?;
            let field_offset = offset.checked_add(field.offset as usize).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "field offset overflow")
            })?;
            self.write_field(field, value, field_offset)?;
        }
        Ok(())
    }

    fn write_field(
        &mut self,
        field: &crate::gff4::schema::FieldDef,
        value: &Value,
        offset: usize,
    ) -> io::Result<()> {
        if field.is_list {
            return self.write_list(&field.base, field.is_reference, value, offset);
        }

        if field.is_reference {
            return self.write_reference(&field.base, value, offset);
        }

        match &field.base {
            BaseType::Primitive(ty) => self.write_primitive_value(*ty, value, offset),
            BaseType::Struct(_) => {
                let child = value
                    .as_struct()
                    .ok_or_else(|| type_mismatch("Struct", value, "inline struct field"))?;
                self.write_struct(child, offset)
            }
            BaseType::Generic => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "generic field cannot be inline",
            )),
        }
    }

    fn write_list(
        &mut self,
        base: &BaseType,
        indirect: bool,
        value: &Value,
        offset: usize,
    ) -> io::Result<()> {
        let is_empty = matches!(value, Value::List(items) if items.is_empty())
            || matches!(value, Value::Binary(bytes) if bytes.is_empty());
        if is_empty {
            self.write_u32_at(offset, NULLPTR);
            return Ok(());
        }

        let list_offset = self.align_end(4);
        self.write_u32_at(offset, list_offset as u32);
        self.data_section
            .extend_from_slice(&self.pack_u32(list_length(value)? as u32));

        match (base, indirect, value) {
            (BaseType::Generic, true, Value::List(items)) => {
                let elem_offset = self.allocate(items.len() * 8, 1);
                for (index, item) in items.iter().enumerate() {
                    self.write_generic(item, elem_offset + index * 8)?;
                }
                Ok(())
            }
            (BaseType::Struct(_), true, Value::List(items))
            | (BaseType::Primitive(_), true, Value::List(items)) => {
                let elem_offset = self.allocate(items.len() * 4, 1);
                for (index, item) in items.iter().enumerate() {
                    self.write_reference(base, item, elem_offset + index * 4)?;
                }
                Ok(())
            }
            (BaseType::Struct(_), false, Value::List(items)) => {
                let BaseType::Struct(struct_index) = base else {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "inline struct list requires struct base type",
                    ));
                };
                let elem_size = self.struct_def(*struct_index)?.size as usize;
                let elem_offset = self.allocate(items.len() * elem_size, 1);
                for (index, item) in items.iter().enumerate() {
                    let child = item
                        .as_struct()
                        .ok_or_else(|| type_mismatch("Struct", item, "inline struct list"))?;
                    self.write_struct(child, elem_offset + index * elem_size)?;
                }
                Ok(())
            }
            (BaseType::Primitive(ValueType::UInt8), false, Value::Binary(bytes)) => {
                let elem_offset = self.allocate(bytes.len(), 1);
                self.data_section[elem_offset..elem_offset + bytes.len()].copy_from_slice(bytes);
                Ok(())
            }
            (BaseType::Primitive(ty), false, Value::List(items)) => {
                let elem_size = ty.size();
                let elem_offset = self.allocate(items.len() * elem_size, 1);
                for (index, item) in items.iter().enumerate() {
                    self.write_primitive_value(*ty, item, elem_offset + index * elem_size)?;
                }
                Ok(())
            }
            _ => Err(type_mismatch(
                "List/Binary compatible with schema",
                value,
                "list field",
            )),
        }
    }

    fn write_reference(&mut self, base: &BaseType, value: &Value, offset: usize) -> io::Result<()> {
        match base {
            BaseType::Generic => self.write_generic(value, offset),
            BaseType::Struct(_) => {
                if value.is_null() {
                    self.write_u32_at(offset, NULLPTR);
                    return Ok(());
                }
                let child = value
                    .as_struct()
                    .ok_or_else(|| type_mismatch("Struct", value, "reference struct field"))?;
                let struct_size = self.struct_def(child.struct_index)?.size as usize;
                let address = self.allocate(struct_size, 4);
                self.write_struct(child, address)?;
                self.write_u32_at(offset, address as u32);
                Ok(())
            }
            BaseType::Primitive(ty) => {
                if value.is_null() {
                    self.write_u32_at(offset, NULLPTR);
                    return Ok(());
                }
                if *ty == ValueType::ECString {
                    let address = self.cache_ecstring(value)?;
                    self.write_u32_at(offset, address);
                    Ok(())
                } else {
                    let address = self.allocate(ty.size(), 4);
                    self.write_primitive_value(*ty, value, address)?;
                    self.write_u32_at(offset, address as u32);
                    Ok(())
                }
            }
        }
    }

    fn write_generic(&mut self, value: &Value, offset: usize) -> io::Result<()> {
        if value.is_null() {
            self.write_u32_at(offset, NULLPTR);
            self.write_u32_at(offset + 4, NULLPTR);
            return Ok(());
        }

        match value {
            Value::Struct(child) => {
                let size = self.struct_def(child.struct_index)?.size as usize;
                let address = self.allocate(size, 4);
                self.write_struct(child, address)?;
                let flags = pack_flags(false, true, false);
                let type_and_flags = ((flags as u32) << 16) | child.struct_index as u32;
                self.write_u32_at(offset, type_and_flags);
                self.write_u32_at(offset + 4, address as u32);
                Ok(())
            }
            _ => {
                let ty = value_type_for_generic(value)?;
                let address = if ty == ValueType::ECString {
                    self.cache_ecstring(value)?
                } else {
                    let address = self.allocate(ty.size(), 4);
                    self.write_primitive_value(ty, value, address)?;
                    address as u32
                };
                self.write_u32_at(offset, ty.type_id() as u32);
                self.write_u32_at(offset + 4, address);
                Ok(())
            }
        }
    }

    fn write_primitive_value(
        &mut self,
        ty: ValueType,
        value: &Value,
        offset: usize,
    ) -> io::Result<()> {
        match ty {
            ValueType::UInt8 => self.write_u8_at(offset, expect_u8(value, "UInt8 field")?),
            ValueType::Int8 => self.write_i8_at(offset, expect_i8(value, "Int8 field")?),
            ValueType::UInt16 => self.write_u16_at(offset, expect_u16(value, "UInt16 field")?),
            ValueType::Int16 => self.write_i16_at(offset, expect_i16(value, "Int16 field")?),
            ValueType::UInt32 => {
                self.write_u32_at(offset, expect_u32(value, "UInt32 field")?);
                Ok(())
            }
            ValueType::Int32 => self.write_i32_at(offset, expect_i32(value, "Int32 field")?),
            ValueType::UInt64 => self.write_u64_at(offset, expect_u64(value, "UInt64 field")?),
            ValueType::Int64 => self.write_i64_at(offset, expect_i64(value, "Int64 field")?),
            ValueType::Float32 => self.write_f32_at(offset, expect_f32(value, "Float32 field")?),
            ValueType::Float64 => self.write_f64_at(offset, expect_f64(value, "Float64 field")?),
            ValueType::Vector3f => {
                let Value::Vector3f(values) = value else {
                    return Err(type_mismatch("Vector3f", value, "Vector3f field"));
                };
                for (index, item) in values.iter().enumerate() {
                    self.write_f32_at(offset + index * 4, *item)?;
                }
                Ok(())
            }
            ValueType::Vector4f => {
                let Value::Vector4f(values) = value else {
                    return Err(type_mismatch("Vector4f", value, "Vector4f field"));
                };
                for (index, item) in values.iter().enumerate() {
                    self.write_f32_at(offset + index * 4, *item)?;
                }
                Ok(())
            }
            ValueType::Quaternionf => {
                let Value::Quaternionf(values) = value else {
                    return Err(type_mismatch("Quaternionf", value, "Quaternionf field"));
                };
                for (index, item) in values.iter().enumerate() {
                    self.write_f32_at(offset + index * 4, *item)?;
                }
                Ok(())
            }
            ValueType::Color4f => {
                let Value::Color4f(values) = value else {
                    return Err(type_mismatch("Color4f", value, "Color4f field"));
                };
                for (index, item) in values.iter().enumerate() {
                    self.write_f32_at(offset + index * 4, *item)?;
                }
                Ok(())
            }
            ValueType::Matrix4x4f => {
                let Value::Matrix4x4f(values) = value else {
                    return Err(type_mismatch("Matrix4x4f", value, "Matrix4x4f field"));
                };
                for (index, item) in values.iter().enumerate() {
                    self.write_f32_at(offset + index * 4, *item)?;
                }
                Ok(())
            }
            ValueType::ECString => {
                let address = self.cache_ecstring(value)?;
                self.write_u32_at(offset, address);
                Ok(())
            }
            ValueType::TlkString => {
                let Value::TlkString {
                    label,
                    text,
                    raw_zero,
                } = value
                else {
                    return Err(type_mismatch("TlkString", value, "TlkString field"));
                };
                self.write_u32_at(offset, *label);
                let address = if text.is_none() && *raw_zero {
                    0
                } else if let Some(text) = text {
                    self.cache_string(text)?
                } else {
                    NULLPTR
                };
                self.write_u32_at(offset + 4, address);
                Ok(())
            }
        }
    }

    fn cache_ecstring(&mut self, value: &Value) -> io::Result<u32> {
        match value {
            Value::Null => Ok(NULLPTR),
            Value::ECString(text) => self.cache_string(text),
            other => Err(type_mismatch("ECString/Null", other, "ECString field")),
        }
    }

    fn cache_string(&mut self, text: &str) -> io::Result<u32> {
        let text = text.to_string();
        if self.use_cstring {
            if let Some(index) = self.string_cache.get(&text) {
                return Ok(*index);
            }

            let index = self.string_cache.len() as u32;
            self.string_cache.insert(text.clone(), index);
            self.string_section.extend_from_slice(text.as_bytes());
            self.string_section.push(0);
            Ok(index)
        } else {
            if let Some(offset) = self.string_cache.get(&text) {
                return Ok(*offset);
            }

            let offset = self.align_end(4);
            self.data_section
                .extend_from_slice(&self.pack_u32(text.encode_utf16().count() as u32));
            for word in text.encode_utf16() {
                self.data_section.extend_from_slice(&word.to_le_bytes());
            }
            self.string_cache.insert(text, offset as u32);
            Ok(offset as u32)
        }
    }

    fn struct_def(&self, index: usize) -> io::Result<&StructDef> {
        self.header.structs.get(index).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid struct index {index}"),
            )
        })
    }

    fn allocate(&mut self, size: usize, align: usize) -> usize {
        let offset = self.align_end(align);
        self.data_section.resize(offset + size, 0xFF);
        offset
    }

    fn align_end(&mut self, align: usize) -> usize {
        let mut offset = self.data_section.len();
        if align > 1 && offset % align != 0 {
            let padding = align - offset % align;
            self.data_section.resize(offset + padding, 0xFF);
            offset += padding;
        }
        offset
    }

    fn ensure_len(&mut self, len: usize) {
        if self.data_section.len() < len {
            self.data_section.resize(len, 0xFF);
        }
    }

    fn write_u8_at(&mut self, offset: usize, value: u8) -> io::Result<()> {
        self.ensure_len(offset + 1);
        self.data_section[offset] = value;
        Ok(())
    }

    fn write_i8_at(&mut self, offset: usize, value: i8) -> io::Result<()> {
        self.write_u8_at(offset, value as u8)
    }

    fn write_u16_at(&mut self, offset: usize, value: u16) -> io::Result<()> {
        let bytes = if self.big_endian {
            value.to_be_bytes()
        } else {
            value.to_le_bytes()
        };
        self.write_bytes_at(offset, &bytes);
        Ok(())
    }

    fn write_i16_at(&mut self, offset: usize, value: i16) -> io::Result<()> {
        self.write_u16_at(offset, value as u16)
    }

    fn write_u32_at(&mut self, offset: usize, value: u32) {
        let bytes = self.pack_u32(value);
        self.write_bytes_at(offset, &bytes);
    }

    fn write_i32_at(&mut self, offset: usize, value: i32) -> io::Result<()> {
        self.write_u32_at(offset, value as u32);
        Ok(())
    }

    fn write_u64_at(&mut self, offset: usize, value: u64) -> io::Result<()> {
        let bytes = if self.big_endian {
            value.to_be_bytes()
        } else {
            value.to_le_bytes()
        };
        self.write_bytes_at(offset, &bytes);
        Ok(())
    }

    fn write_i64_at(&mut self, offset: usize, value: i64) -> io::Result<()> {
        self.write_u64_at(offset, value as u64)
    }

    fn write_f32_at(&mut self, offset: usize, value: f32) -> io::Result<()> {
        self.write_u32_at(offset, value.to_bits());
        Ok(())
    }

    fn write_f64_at(&mut self, offset: usize, value: f64) -> io::Result<()> {
        self.write_u64_at(offset, value.to_bits())
    }

    fn write_bytes_at(&mut self, offset: usize, bytes: &[u8]) {
        self.ensure_len(offset + bytes.len());
        self.data_section[offset..offset + bytes.len()].copy_from_slice(bytes);
    }

    fn pack_u32(&self, value: u32) -> [u8; 4] {
        if self.big_endian {
            value.to_be_bytes()
        } else {
            value.to_le_bytes()
        }
    }
}

fn list_length(value: &Value) -> io::Result<usize> {
    match value {
        Value::List(items) => Ok(items.len()),
        Value::Binary(bytes) => Ok(bytes.len()),
        other => Err(type_mismatch("List/Binary", other, "list length")),
    }
}

fn pack_flags(is_list: bool, is_struct: bool, is_reference: bool) -> u16 {
    (if is_list { 0x8000 } else { 0 })
        | (if is_struct { 0x4000 } else { 0 })
        | (if is_reference { 0x2000 } else { 0 })
}

fn field_type_id(field: &crate::gff4::schema::FieldDef) -> io::Result<u16> {
    match field.base {
        BaseType::Primitive(ty) => Ok(ty.type_id()),
        BaseType::Struct(index) => u16::try_from(index).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("struct index {index} does not fit into u16"),
            )
        }),
        BaseType::Generic => Ok(0xFFFF),
    }
}

fn value_type_for_generic(value: &Value) -> io::Result<ValueType> {
    match value {
        Value::UInt8(_) => Ok(ValueType::UInt8),
        Value::Int8(_) => Ok(ValueType::Int8),
        Value::UInt16(_) => Ok(ValueType::UInt16),
        Value::Int16(_) => Ok(ValueType::Int16),
        Value::UInt32(_) => Ok(ValueType::UInt32),
        Value::Int32(_) => Ok(ValueType::Int32),
        Value::UInt64(_) => Ok(ValueType::UInt64),
        Value::Int64(_) => Ok(ValueType::Int64),
        Value::Float32(_) => Ok(ValueType::Float32),
        Value::Float64(_) => Ok(ValueType::Float64),
        Value::Vector3f(_) => Ok(ValueType::Vector3f),
        Value::Vector4f(_) => Ok(ValueType::Vector4f),
        Value::Quaternionf(_) => Ok(ValueType::Quaternionf),
        Value::ECString(_) => Ok(ValueType::ECString),
        Value::Color4f(_) => Ok(ValueType::Color4f),
        Value::Matrix4x4f(_) => Ok(ValueType::Matrix4x4f),
        Value::TlkString { .. } => Ok(ValueType::TlkString),
        other => Err(type_mismatch(
            "generic primitive/struct",
            other,
            "generic value",
        )),
    }
}

fn type_mismatch(expected: &'static str, actual: &Value, context: &'static str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!(
            "type mismatch for {context}: expected {expected}, found {}",
            actual.type_name()
        ),
    )
}

fn expect_u8(value: &Value, context: &'static str) -> io::Result<u8> {
    value
        .as_u8()
        .ok_or_else(|| type_mismatch("UInt8", value, context))
}
fn expect_i8(value: &Value, context: &'static str) -> io::Result<i8> {
    value
        .as_i8()
        .ok_or_else(|| type_mismatch("Int8", value, context))
}
fn expect_u16(value: &Value, context: &'static str) -> io::Result<u16> {
    value
        .as_u16()
        .ok_or_else(|| type_mismatch("UInt16", value, context))
}
fn expect_i16(value: &Value, context: &'static str) -> io::Result<i16> {
    value
        .as_i16()
        .ok_or_else(|| type_mismatch("Int16", value, context))
}
fn expect_u32(value: &Value, context: &'static str) -> io::Result<u32> {
    value
        .as_u32()
        .ok_or_else(|| type_mismatch("UInt32", value, context))
}
fn expect_i32(value: &Value, context: &'static str) -> io::Result<i32> {
    value
        .as_i32()
        .ok_or_else(|| type_mismatch("Int32", value, context))
}
fn expect_u64(value: &Value, context: &'static str) -> io::Result<u64> {
    value
        .as_u64()
        .ok_or_else(|| type_mismatch("UInt64", value, context))
}
fn expect_i64(value: &Value, context: &'static str) -> io::Result<i64> {
    value
        .as_i64()
        .ok_or_else(|| type_mismatch("Int64", value, context))
}
fn expect_f32(value: &Value, context: &'static str) -> io::Result<f32> {
    value
        .as_f32()
        .ok_or_else(|| type_mismatch("Float32", value, context))
}
fn expect_f64(value: &Value, context: &'static str) -> io::Result<f64> {
    value
        .as_f64()
        .ok_or_else(|| type_mismatch("Float64", value, context))
}

impl ValueType {
    fn type_id(self) -> u16 {
        match self {
            ValueType::UInt8 => 0,
            ValueType::Int8 => 1,
            ValueType::UInt16 => 2,
            ValueType::Int16 => 3,
            ValueType::UInt32 => 4,
            ValueType::Int32 => 5,
            ValueType::UInt64 => 6,
            ValueType::Int64 => 7,
            ValueType::Float32 => 8,
            ValueType::Float64 => 9,
            ValueType::Vector3f => 10,
            ValueType::Vector4f => 12,
            ValueType::Quaternionf => 13,
            ValueType::ECString => 14,
            ValueType::Color4f => 15,
            ValueType::Matrix4x4f => 16,
            ValueType::TlkString => 17,
        }
    }
}
