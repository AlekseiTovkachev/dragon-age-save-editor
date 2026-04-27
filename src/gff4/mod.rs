pub mod fields;
pub mod header;
pub mod reader;
pub mod schema;
pub mod value;
pub mod writer;

pub use reader::{GffFile, Reader};
pub use value::{FieldValue, GffStruct, Value};

#[cfg(test)]
mod tests {
    use super::GffFile;
    use crate::test_support::{da2_save_path, dao_save_path};
    use std::fs;
    use std::io;

    #[test]
    fn rejects_empty_or_truncated_binary_inputs() {
        let empty = GffFile::from_bytes(Vec::new()).unwrap_err();
        assert_eq!(empty.kind(), io::ErrorKind::UnexpectedEof);

        let bytes = fs::read(dao_save_path()).unwrap();
        for len in [1, 8, 24, 64, bytes.len() / 2] {
            assert!(
                GffFile::from_bytes(bytes[..len].to_vec()).is_err(),
                "expected truncated input of {len} bytes to fail"
            );
        }
    }

    #[test]
    fn roundtrips_representative_dao_and_da2_files_through_writer() {
        for path in [dao_save_path(), da2_save_path()] {
            let original = GffFile::from_path(&path).unwrap();
            let bytes = original.to_bytes().unwrap();
            let reread = GffFile::from_bytes(bytes).unwrap();

            assert_eq!(reread.header.version, original.header.version);
            assert_eq!(reread.header.platform, original.header.platform);
            assert_eq!(reread.header.file_type, original.header.file_type);
            assert_eq!(reread.header.structs.len(), original.header.structs.len());
            assert_eq!(reread.root.struct_index, original.root.struct_index);
            assert_eq!(reread.root.fields.len(), original.root.fields.len());
        }
    }
}
