use greptime_proto::v1::{self, value::ValueData};

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Value {
    BoolValue(libc::c_char),
    I8Value(libc::c_char),
    I16Value(libc::c_short),
    I32Value(libc::c_int),
    I64Value(libc::c_long),
    U8Value(libc::c_uchar),
    U16Value(libc::c_ushort),
    U32Value(libc::c_uint),
    U64Value(libc::c_ulong),
    F32Value(libc::c_float),
    F64Value(libc::c_double),
    StringValue(*const libc::c_char),
    BinaryValue(*const libc::c_void, libc::size_t),
    TimestampValueSecond(libc::c_long),
    TimestampValueMillisecond(libc::c_long),
    TimestampValueMicrosecond(libc::c_long),
    TimestampValueNanosecond(libc::c_long),

    BoolValueNull,
    I8ValueNull,
    I16ValueNull,
    I32ValueNull,
    I64ValueNull,
    U8ValueNull,
    U16ValueNull,
    U32ValueNull,
    U64ValueNull,
    F32ValueNull,
    F64ValueNull,
    StringValueNull,
    BinaryValueNull,
    TimestampValueSecondNull,
    TimestampValueMillisecondNull,
    TimestampValueMicrosecondNull,
    TimestampValueNanosecondNull,
}

impl From<Value> for v1::Value {
    fn from(val: Value) -> Self {
        let value_data = match val {
            Value::BoolValue(val) => Some(ValueData::BoolValue(val != 0)),
            Value::I8Value(val) => Some(ValueData::I8Value(val.into())),
            Value::I16Value(val) => Some(ValueData::I16Value(val.into())),
            Value::I32Value(val) => Some(ValueData::I32Value(val)),
            Value::I64Value(val) => Some(ValueData::I64Value(val)),
            Value::U8Value(val) => Some(ValueData::U8Value(val.into())),
            Value::U16Value(val) => Some(ValueData::U16Value(val.into())),
            Value::U32Value(val) => Some(ValueData::U32Value(val)),
            Value::U64Value(val) => Some(ValueData::U64Value(val)),
            Value::F32Value(val) => Some(ValueData::F32Value(val)),
            Value::F64Value(val) => Some(ValueData::F64Value(val)),
            Value::StringValue(_) => todo!(),
            Value::BinaryValue(_, _) => todo!(),
            Value::TimestampValueSecond(val) => Some(ValueData::TimestampSecondValue(val)),
            Value::TimestampValueMillisecond(val) => {
                Some(ValueData::TimestampMillisecondValue(val))
            }
            Value::TimestampValueMicrosecond(val) => {
                Some(ValueData::TimestampMicrosecondValue(val))
            }
            Value::TimestampValueNanosecond(val) => Some(ValueData::TimestampNanosecondValue(val)),
            Value::BoolValueNull => None,
            Value::I8ValueNull => None,
            Value::I16ValueNull => None,
            Value::I32ValueNull => None,
            Value::I64ValueNull => None,
            Value::U8ValueNull => None,
            Value::U16ValueNull => None,
            Value::U32ValueNull => None,
            Value::U64ValueNull => None,
            Value::F32ValueNull => None,
            Value::F64ValueNull => None,
            Value::StringValueNull => None,
            Value::BinaryValueNull => None,
            Value::TimestampValueSecondNull => None,
            Value::TimestampValueMillisecondNull => None,
            Value::TimestampValueMicrosecondNull => None,
            Value::TimestampValueNanosecondNull => None,
        };
        v1::Value { value_data }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DataType {
    Boolean = 0,
    I8 = 1,
    I16 = 2,
    I32 = 3,
    I64 = 4,
    U8 = 5,
    U16 = 6,
    U32 = 7,
    U64 = 8,
    F32 = 9,
    F64 = 10,
    Binary = 11,
    String = 12,
    TimestampSecond = 15,
    TimestampMillisecond = 16,
    TimestampMicrosecond = 17,
    TimestampNanosecond = 18,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SemanticType {
    Tag = 0,
    Field,
    Timestamp,
}

#[cfg(test)]
mod tests {
    use super::DataType;
    use super::SemanticType;
    use greptime_proto::v1::{ColumnDataType as ProtoDataType, SemanticType as ProtoSemanticType};

    #[test]
    fn test_data_type() {
        assert_eq!(DataType::Boolean as i32, ProtoDataType::Boolean as i32);
        assert_eq!(DataType::I8 as i32, ProtoDataType::Int8 as i32);
        assert_eq!(DataType::I16 as i32, ProtoDataType::Int16 as i32);
        assert_eq!(DataType::I32 as i32, ProtoDataType::Int32 as i32);
        assert_eq!(DataType::I64 as i32, ProtoDataType::Int64 as i32);
        assert_eq!(DataType::U8 as i32, ProtoDataType::Uint8 as i32);
        assert_eq!(DataType::U16 as i32, ProtoDataType::Uint16 as i32);
        assert_eq!(DataType::U32 as i32, ProtoDataType::Uint32 as i32);
        assert_eq!(DataType::U64 as i32, ProtoDataType::Uint64 as i32);
        assert_eq!(DataType::F32 as i32, ProtoDataType::Float32 as i32);
        assert_eq!(DataType::F64 as i32, ProtoDataType::Float64 as i32);
        assert_eq!(DataType::Binary as i32, ProtoDataType::Binary as i32);
        assert_eq!(DataType::String as i32, ProtoDataType::String as i32);
        assert_eq!(
            DataType::TimestampSecond as i32,
            ProtoDataType::TimestampSecond as i32
        );
        assert_eq!(
            DataType::TimestampMillisecond as i32,
            ProtoDataType::TimestampMillisecond as i32
        );
        assert_eq!(
            DataType::TimestampMicrosecond as i32,
            ProtoDataType::TimestampMicrosecond as i32
        );
        assert_eq!(
            DataType::TimestampNanosecond as i32,
            ProtoDataType::TimestampNanosecond as i32
        );
    }

    #[test]
    fn test_semantic_type() {
        assert_eq!(SemanticType::Tag as i32, ProtoSemanticType::Tag as i32);
        assert_eq!(SemanticType::Field as i32, ProtoSemanticType::Field as i32);
        assert_eq!(
            SemanticType::Timestamp as i32,
            ProtoSemanticType::Timestamp as i32
        );
    }
}
