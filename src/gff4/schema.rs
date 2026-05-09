use crate::gff4::header::{Header, RawFieldDef};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    UInt8,
    Int8,
    UInt16,
    Int16,
    UInt32,
    Int32,
    UInt64,
    Int64,
    Float32,
    Float64,
    Vector3f,
    Vector4f,
    Quaternionf,
    ECString,
    Color4f,
    Matrix4x4f,
    TlkString,
}

impl ValueType {
    pub fn from_type_id(id: u16) -> Option<Self> {
        match id {
            0 => Some(Self::UInt8),
            1 => Some(Self::Int8),
            2 => Some(Self::UInt16),
            3 => Some(Self::Int16),
            4 => Some(Self::UInt32),
            5 => Some(Self::Int32),
            6 => Some(Self::UInt64),
            7 => Some(Self::Int64),
            8 => Some(Self::Float32),
            9 => Some(Self::Float64),
            10 => Some(Self::Vector3f),
            12 => Some(Self::Vector4f),
            13 => Some(Self::Quaternionf),
            14 => Some(Self::ECString),
            15 => Some(Self::Color4f),
            16 => Some(Self::Matrix4x4f),
            17 => Some(Self::TlkString),
            _ => None,
        }
    }

    pub fn size(self) -> usize {
        match self {
            ValueType::UInt8 => 1,
            ValueType::Int8 => 1,
            ValueType::UInt16 => 2,
            ValueType::Int16 => 2,
            ValueType::UInt32 => 4,
            ValueType::Int32 => 4,
            ValueType::UInt64 => 8,
            ValueType::Int64 => 8,
            ValueType::Float32 => 4,
            ValueType::Float64 => 8,
            ValueType::Vector3f => 12,
            ValueType::Vector4f => 16,
            ValueType::Quaternionf => 16,
            ValueType::ECString => 4,
            ValueType::Color4f => 16,
            ValueType::Matrix4x4f => 64,
            ValueType::TlkString => 8,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseType {
    Primitive(ValueType),
    Struct(usize),
    Generic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDef {
    pub label: u32,
    pub base: BaseType,
    pub is_list: bool,
    pub is_reference: bool,
    pub offset: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructDef {
    pub fourcc: [u8; 4],
    pub size: u32,
    pub fields: Vec<FieldDef>,
}

#[derive(Debug, Clone)]
pub struct ResolvedHeader {
    pub version: [u8; 4],
    pub platform: [u8; 4],
    pub file_type: [u8; 4],
    pub file_version: [u8; 4],
    pub string_count: u32,
    pub string_offset: u32,
    pub data_offset: u32,
    pub structs: Vec<StructDef>,
}

fn resolve_field(field: &RawFieldDef, struct_count: usize) -> io::Result<FieldDef> {
    let base = if field.is_struct {
        let idx = field.type_id as usize;
        if idx >= struct_count {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("struct field references invalid struct index {idx}"),
            ));
        }
        BaseType::Struct(idx)
    } else if field.type_id == 0xFFFF {
        if !field.is_reference {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "generic field must be a reference",
            ));
        }
        BaseType::Generic
    } else {
        let ty = ValueType::from_type_id(field.type_id).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown primitive type id {}", field.type_id),
            )
        })?;
        BaseType::Primitive(ty)
    };

    Ok(FieldDef {
        label: field.label,
        base,
        is_list: field.is_list,
        is_reference: field.is_reference,
        offset: field.offset,
    })
}

pub fn resolve_header(raw: &Header) -> io::Result<ResolvedHeader> {
    let struct_count = raw.structs.len();
    let mut structs = Vec::with_capacity(struct_count);

    for raw_struct in &raw.structs {
        let mut fields = Vec::with_capacity(raw_struct.fields.len());

        for raw_field in &raw_struct.fields {
            fields.push(resolve_field(raw_field, struct_count)?);
        }

        structs.push(StructDef {
            fourcc: raw_struct.type_code,
            size: raw_struct.size,
            fields,
        });
    }

    Ok(ResolvedHeader {
        version: raw.version,
        platform: raw.platform,
        file_type: raw.file_type,
        file_version: raw.file_version,
        string_count: raw.string_count,
        string_offset: raw.string_offset,
        data_offset: raw.data_offset,
        structs,
    })
}
