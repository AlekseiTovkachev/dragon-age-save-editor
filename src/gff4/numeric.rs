use super::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumericWriteError {
    Unsupported { actual: &'static str },
    Range { detail: String },
}

pub fn to_u32_compatible(value: &Value) -> Option<u32> {
    match value {
        Value::UInt8(v) => Some(*v as u32),
        Value::UInt16(v) => Some(*v as u32),
        Value::UInt32(v) => Some(*v),
        Value::Int8(v) if *v >= 0 => Some(*v as u32),
        Value::Int16(v) if *v >= 0 => Some(*v as u32),
        Value::Int32(v) if *v >= 0 => Some(*v as u32),
        Value::Float32(v) if v.is_finite() && *v >= 0.0 => Some(*v as u32),
        Value::Float64(v) if v.is_finite() && *v >= 0.0 => Some(*v as u32),
        _ => None,
    }
}

pub fn to_u16_compatible(value: &Value) -> Option<u16> {
    to_u32_compatible(value).and_then(|value| u16::try_from(value).ok())
}

pub fn to_i32_compatible(value: &Value) -> Option<i32> {
    match value {
        Value::UInt8(v) => Some(*v as i32),
        Value::UInt16(v) => Some(*v as i32),
        Value::UInt32(v) => i32::try_from(*v).ok(),
        Value::Int8(v) => Some(*v as i32),
        Value::Int16(v) => Some(*v as i32),
        Value::Int32(v) => Some(*v),
        Value::Float32(v) if v.is_finite() => Some(*v as i32),
        Value::Float64(v) if v.is_finite() => Some(*v as i32),
        _ => None,
    }
}

pub fn to_f32_compatible(value: &Value) -> Option<f32> {
    match value {
        Value::Float32(v) => Some(*v),
        Value::Float64(v) => Some(*v as f32),
        Value::UInt8(v) => Some(*v as f32),
        Value::UInt16(v) => Some(*v as f32),
        Value::UInt32(v) => Some(*v as f32),
        Value::Int8(v) => Some(*v as f32),
        Value::Int16(v) => Some(*v as f32),
        Value::Int32(v) => Some(*v as f32),
        _ => None,
    }
}

pub fn to_da2_property_power(value: &Value) -> Option<f32> {
    match value {
        Value::UInt32(v) => Some(f32::from_bits(*v)),
        Value::Int32(v) => Some(f32::from_bits(*v as u32)),
        Value::UInt64(v) => u32::try_from(*v).ok().map(f32::from_bits),
        Value::Int64(v) if *v >= 0 => u32::try_from(*v).ok().map(f32::from_bits),
        _ => to_f32_compatible(value),
    }
}

pub fn set_u32_compatible(value: &mut Value, new_value: u32) -> Result<(), NumericWriteError> {
    match value {
        Value::UInt8(existing) => {
            *existing = u8::try_from(new_value).map_err(|_| range_error(new_value, "u8"))?;
            Ok(())
        }
        Value::Int8(existing) => {
            *existing = i8::try_from(new_value).map_err(|_| range_error(new_value, "i8"))?;
            Ok(())
        }
        Value::UInt16(existing) => {
            *existing = u16::try_from(new_value).map_err(|_| range_error(new_value, "u16"))?;
            Ok(())
        }
        Value::Int16(existing) => {
            *existing = i16::try_from(new_value).map_err(|_| range_error(new_value, "i16"))?;
            Ok(())
        }
        Value::UInt32(existing) => {
            *existing = new_value;
            Ok(())
        }
        Value::Int32(existing) => {
            *existing = i32::try_from(new_value).map_err(|_| range_error(new_value, "i32"))?;
            Ok(())
        }
        Value::Float32(existing) => {
            *existing = new_value as f32;
            Ok(())
        }
        Value::Float64(existing) => {
            *existing = new_value as f64;
            Ok(())
        }
        other => Err(NumericWriteError::Unsupported {
            actual: other.type_name(),
        }),
    }
}

pub fn set_i32_compatible(value: &mut Value, new_value: i32) -> Result<(), NumericWriteError> {
    match value {
        Value::UInt8(existing) => {
            *existing = u8::try_from(new_value).map_err(|_| range_error(new_value, "u8"))?;
            Ok(())
        }
        Value::Int8(existing) => {
            *existing = i8::try_from(new_value).map_err(|_| range_error(new_value, "i8"))?;
            Ok(())
        }
        Value::UInt16(existing) => {
            *existing = u16::try_from(new_value).map_err(|_| range_error(new_value, "u16"))?;
            Ok(())
        }
        Value::Int16(existing) => {
            *existing = i16::try_from(new_value).map_err(|_| range_error(new_value, "i16"))?;
            Ok(())
        }
        Value::UInt32(existing) => {
            *existing = u32::try_from(new_value).map_err(|_| range_error(new_value, "u32"))?;
            Ok(())
        }
        Value::Int32(existing) => {
            *existing = new_value;
            Ok(())
        }
        Value::Float32(existing) => {
            *existing = new_value as f32;
            Ok(())
        }
        Value::Float64(existing) => {
            *existing = new_value as f64;
            Ok(())
        }
        other => Err(NumericWriteError::Unsupported {
            actual: other.type_name(),
        }),
    }
}

pub fn set_f32_compatible(value: &mut Value, new_value: f32) -> Result<(), NumericWriteError> {
    match value {
        Value::Float32(existing) => {
            *existing = new_value;
            Ok(())
        }
        Value::Float64(existing) => {
            *existing = new_value as f64;
            Ok(())
        }
        Value::UInt8(existing) => {
            if new_value.is_finite() && new_value >= 0.0 && new_value <= u8::MAX as f32 {
                *existing = new_value as u8;
                Ok(())
            } else {
                Err(range_error(new_value, "u8"))
            }
        }
        Value::Int8(existing) => {
            if new_value.is_finite() && new_value >= i8::MIN as f32 && new_value <= i8::MAX as f32 {
                *existing = new_value as i8;
                Ok(())
            } else {
                Err(range_error(new_value, "i8"))
            }
        }
        Value::UInt16(existing) => {
            if new_value.is_finite() && new_value >= 0.0 && new_value <= u16::MAX as f32 {
                *existing = new_value as u16;
                Ok(())
            } else {
                Err(range_error(new_value, "u16"))
            }
        }
        Value::Int16(existing) => {
            if new_value.is_finite() && new_value >= i16::MIN as f32 && new_value <= i16::MAX as f32
            {
                *existing = new_value as i16;
                Ok(())
            } else {
                Err(range_error(new_value, "i16"))
            }
        }
        Value::UInt32(existing) => {
            if new_value.is_finite() && new_value >= 0.0 && new_value <= u32::MAX as f32 {
                *existing = new_value as u32;
                Ok(())
            } else {
                Err(range_error(new_value, "u32"))
            }
        }
        Value::Int32(existing) => {
            if new_value.is_finite() && new_value >= i32::MIN as f32 && new_value <= i32::MAX as f32
            {
                *existing = new_value as i32;
                Ok(())
            } else {
                Err(range_error(new_value, "i32"))
            }
        }
        other => Err(NumericWriteError::Unsupported {
            actual: other.type_name(),
        }),
    }
}

pub fn set_da2_property_power(value: &mut Value, new_value: f32) -> Result<(), NumericWriteError> {
    match value {
        Value::UInt32(existing) => {
            *existing = new_value.to_bits();
            Ok(())
        }
        Value::Int32(existing) => {
            *existing = i32::from_ne_bytes(new_value.to_bits().to_ne_bytes());
            Ok(())
        }
        Value::UInt64(existing) => {
            *existing = new_value.to_bits() as u64;
            Ok(())
        }
        Value::Int64(existing) => {
            *existing = new_value.to_bits() as i64;
            Ok(())
        }
        _ => set_f32_compatible(value, new_value),
    }
}

fn range_error<T: std::fmt::Display>(value: T, target: &str) -> NumericWriteError {
    NumericWriteError::Range {
        detail: format!("{value} does not fit into {target}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        NumericWriteError, set_da2_property_power, set_f32_compatible, set_i32_compatible,
        set_u32_compatible, to_da2_property_power, to_f32_compatible, to_i32_compatible,
        to_u16_compatible, to_u32_compatible,
    };
    use crate::gff4::Value;

    #[test]
    fn compatible_unsigned_numbers_accept_non_negative_values() {
        assert_eq!(to_u32_compatible(&Value::UInt8(7)), Some(7));
        assert_eq!(to_u32_compatible(&Value::Int16(8)), Some(8));
        assert_eq!(to_u32_compatible(&Value::Float32(9.8)), Some(9));
    }

    #[test]
    fn compatible_unsigned_numbers_reject_negative_and_non_finite_values() {
        assert_eq!(to_u32_compatible(&Value::Int32(-1)), None);
        assert_eq!(to_u32_compatible(&Value::Float32(f32::NAN)), None);
        assert_eq!(to_u32_compatible(&Value::Float64(f64::INFINITY)), None);
    }

    #[test]
    fn compatible_narrow_unsigned_rejects_overflow() {
        assert_eq!(
            to_u16_compatible(&Value::UInt32(u16::MAX as u32)),
            Some(u16::MAX)
        );
        assert_eq!(to_u16_compatible(&Value::UInt32(u16::MAX as u32 + 1)), None);
    }

    #[test]
    fn compatible_signed_numbers_accept_integral_and_finite_float_values() {
        assert_eq!(to_i32_compatible(&Value::UInt16(12)), Some(12));
        assert_eq!(to_i32_compatible(&Value::Int8(-12)), Some(-12));
        assert_eq!(to_i32_compatible(&Value::Float64(-12.9)), Some(-12));
        assert_eq!(to_i32_compatible(&Value::UInt32(i32::MAX as u32 + 1)), None);
    }

    #[test]
    fn compatible_float_numbers_accept_numeric_shapes() {
        assert_eq!(to_f32_compatible(&Value::UInt32(12)), Some(12.0));
        assert_eq!(to_f32_compatible(&Value::Int32(-12)), Some(-12.0));
        assert_eq!(to_f32_compatible(&Value::Float64(12.5)), Some(12.5));
    }

    #[test]
    fn unsigned_writes_preserve_existing_value_shape_and_reject_overflow() {
        let mut value = Value::UInt16(0);
        set_u32_compatible(&mut value, 12).unwrap();
        assert_eq!(value, Value::UInt16(12));

        let mut value = Value::Int8(0);
        assert!(matches!(
            set_u32_compatible(&mut value, i8::MAX as u32 + 1),
            Err(NumericWriteError::Range { .. })
        ));
    }

    #[test]
    fn signed_writes_preserve_existing_value_shape_and_reject_unsigned_negative() {
        let mut value = Value::Int16(0);
        set_i32_compatible(&mut value, -12).unwrap();
        assert_eq!(value, Value::Int16(-12));

        let mut value = Value::UInt8(0);
        assert!(matches!(
            set_i32_compatible(&mut value, -1),
            Err(NumericWriteError::Range { .. })
        ));
    }

    #[test]
    fn float_writes_reject_non_finite_values_for_integral_shapes() {
        let mut value = Value::UInt32(0);
        assert!(matches!(
            set_f32_compatible(&mut value, f32::NAN),
            Err(NumericWriteError::Range { .. })
        ));
    }

    #[test]
    fn da2_property_power_decodes_and_writes_integer_float_bits() {
        assert_eq!(
            to_da2_property_power(&Value::UInt32(1.25f32.to_bits())),
            Some(1.25)
        );

        let mut value = Value::UInt32(0);
        set_da2_property_power(&mut value, 1.25).unwrap();
        assert_eq!(value, Value::UInt32(1.25f32.to_bits()));
    }

    #[test]
    fn da2_property_power_falls_back_to_float_shapes() {
        let mut value = Value::Float32(0.0);
        set_da2_property_power(&mut value, 1.25).unwrap();
        assert_eq!(value, Value::Float32(1.25));
        assert_eq!(to_da2_property_power(&value), Some(1.25));
    }
}
