use super::{
    error::{self, RustResult},
    utils::convert_c_string,
    value::{DataType, SemanticType},
};
use greptime_proto::v1::ColumnSchema as ProtoColumnSchema;

#[repr(C)]
pub struct ColumnSchema {
    pub name: *const libc::c_char,
    pub data_type: DataType,
    pub semantic_type: SemanticType,
}

impl TryFrom<&ColumnSchema> for ProtoColumnSchema {
    type Error = error::RustError;

    fn try_from(val: &ColumnSchema) -> RustResult<Self> {
        let name = unsafe { convert_c_string(val.name) }?;
        let datatype = val.data_type as i32;
        let semantic_type = val.semantic_type as i32;

        Ok(ProtoColumnSchema {
            column_name: name,
            datatype,
            semantic_type,
            datatype_extension: None,
            options: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use greptime_proto::v1::{ColumnDataType as ProtoDataType, SemanticType as ProtoSemanticType};

    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_convert_column_schema() {
        let name = CString::new("hello").unwrap();
        let name_ptr = name.as_ptr();
        let column_schema = ColumnSchema {
            name: name_ptr,
            data_type: DataType::I8,
            semantic_type: SemanticType::Field,
        };

        let result = ProtoColumnSchema::try_from(&column_schema);

        assert!(result.is_ok());
        let proto_column_schema = result.unwrap();
        assert_eq!(proto_column_schema.column_name, "hello");
        assert_eq!(proto_column_schema.datatype, ProtoDataType::Int8 as i32);
        assert_eq!(
            proto_column_schema.semantic_type,
            ProtoSemanticType::Field as i32
        );
    }
}
