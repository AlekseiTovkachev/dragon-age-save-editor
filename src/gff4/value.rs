use crate::gff4::fields::{field_id_by_name, field_name_by_id};
use crate::gff4::schema::ValueType;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,

    UInt8(u8),
    Int8(i8),
    UInt16(u16),
    Int16(i16),
    UInt32(u32),
    Int32(i32),
    UInt64(u64),
    Int64(i64),

    Float32(f32),
    Float64(f64),

    Vector3f([f32; 3]),
    Vector4f([f32; 4]),
    Quaternionf([f32; 4]),
    Color4f([f32; 4]),
    Matrix4x4f([f32; 16]),

    ECString(String),
    TlkString {
        label: u32,
        text: Option<String>,
        raw_zero: bool,
    },

    Struct(Box<GffStruct>),
    List(Vec<Value>),

    Binary(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldValue {
    pub label: u32,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GffStruct {
    pub struct_index: usize,
    pub fields: Vec<FieldValue>,
}

impl FieldValue {
    pub fn field_name(&self) -> Option<&'static str> {
        field_name_by_id(self.label)
    }
}

impl GffStruct {
    pub fn get(&self, label: u32) -> Option<&Value> {
        self.fields
            .iter()
            .find(|f| f.label == label)
            .map(|f| &f.value)
    }

    pub fn get_mut(&mut self, label: u32) -> Option<&mut Value> {
        self.fields
            .iter_mut()
            .find(|f| f.label == label)
            .map(|f| &mut f.value)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Value> {
        let id = field_id_by_name(name)?;
        self.get(id)
    }

    pub fn get_mut_by_name(&mut self, name: &str) -> Option<&mut Value> {
        let id = field_id_by_name(name)?;
        self.get_mut(id)
    }

    pub fn get_struct(&self, label: u32) -> Option<&GffStruct> {
        self.get(label)?.as_struct()
    }

    pub fn get_struct_mut(&mut self, label: u32) -> Option<&mut GffStruct> {
        self.get_mut(label)?.as_struct_mut()
    }

    pub fn get_struct_by_name(&self, name: &str) -> Option<&GffStruct> {
        self.get_by_name(name)?.as_struct()
    }

    pub fn get_struct_mut_by_name(&mut self, name: &str) -> Option<&mut GffStruct> {
        self.get_mut_by_name(name)?.as_struct_mut()
    }

    pub fn get_list(&self, label: u32) -> Option<&[Value]> {
        self.get(label)?.as_list()
    }

    pub fn get_list_mut(&mut self, label: u32) -> Option<&mut Vec<Value>> {
        self.get_mut(label)?.as_list_mut()
    }

    pub fn get_list_by_name(&self, name: &str) -> Option<&[Value]> {
        self.get_by_name(name)?.as_list()
    }

    pub fn get_list_mut_by_name(&mut self, name: &str) -> Option<&mut Vec<Value>> {
        self.get_mut_by_name(name)?.as_list_mut()
    }

    pub fn dump(&self, indent: usize) {
        let pad = " ".repeat(indent);

        for field in &self.fields {
            let name = field.field_name().unwrap_or("<unknown>");
            println!(
                "{}{} ({}) = {}",
                pad,
                name,
                field.label,
                field.value.type_name()
            );

            match &field.value {
                Value::Struct(s) => s.dump(indent + 2),
                Value::List(items) => {
                    for (i, item) in items.iter().enumerate() {
                        println!("{}  [{}] {}", pad, i, item.type_name());
                        if let Value::Struct(s) = item {
                            s.dump(indent + 4);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "Null",
            Value::UInt8(_) => "UInt8",
            Value::Int8(_) => "Int8",
            Value::UInt16(_) => "UInt16",
            Value::Int16(_) => "Int16",
            Value::UInt32(_) => "UInt32",
            Value::Int32(_) => "Int32",
            Value::UInt64(_) => "UInt64",
            Value::Int64(_) => "Int64",
            Value::Float32(_) => "Float32",
            Value::Float64(_) => "Float64",
            Value::Vector3f(_) => "Vector3f",
            Value::Vector4f(_) => "Vector4f",
            Value::Quaternionf(_) => "Quaternionf",
            Value::Color4f(_) => "Color4f",
            Value::Matrix4x4f(_) => "Matrix4x4f",
            Value::ECString(_) => "ECString",
            Value::TlkString { .. } => "TlkString",
            Value::Struct(_) => "Struct",
            Value::List(_) => "List",
            Value::Binary(_) => "Binary",
        }
    }

    pub fn matches_primitive_type(&self, ty: ValueType) -> bool {
        matches!(
            (self, ty),
            (Value::UInt8(_), ValueType::UInt8)
                | (Value::Int8(_), ValueType::Int8)
                | (Value::UInt16(_), ValueType::UInt16)
                | (Value::Int16(_), ValueType::Int16)
                | (Value::UInt32(_), ValueType::UInt32)
                | (Value::Int32(_), ValueType::Int32)
                | (Value::UInt64(_), ValueType::UInt64)
                | (Value::Int64(_), ValueType::Int64)
                | (Value::Float32(_), ValueType::Float32)
                | (Value::Float64(_), ValueType::Float64)
                | (Value::Vector3f(_), ValueType::Vector3f)
                | (Value::Vector4f(_), ValueType::Vector4f)
                | (Value::Quaternionf(_), ValueType::Quaternionf)
                | (Value::ECString(_), ValueType::ECString)
                | (Value::Color4f(_), ValueType::Color4f)
                | (Value::Matrix4x4f(_), ValueType::Matrix4x4f)
                | (Value::TlkString { .. }, ValueType::TlkString)
        )
    }

    pub fn as_struct(&self) -> Option<&GffStruct> {
        match self {
            Value::Struct(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_struct_mut(&mut self) -> Option<&mut GffStruct> {
        match self {
            Value::Struct(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&[Value]> {
        match self {
            Value::List(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_list_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::List(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_u8(&self) -> Option<u8> {
        match self {
            Value::UInt8(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_i8(&self) -> Option<i8> {
        match self {
            Value::Int8(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_u16(&self) -> Option<u16> {
        match self {
            Value::UInt16(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_i16(&self) -> Option<i16> {
        match self {
            Value::Int16(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Value::UInt32(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Value::Int32(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::UInt64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Int64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Value::Float32(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Float64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_ecstring(&self) -> Option<&str> {
        match self {
            Value::ECString(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_binary(&self) -> Option<&[u8]> {
        match self {
            Value::Binary(b) => Some(b),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
}
