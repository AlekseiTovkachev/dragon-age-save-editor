pub mod fields;
pub mod header;
pub mod reader;
pub mod schema;
pub mod value;
pub mod writer;

pub use reader::{GffFile, Reader};
pub use value::{FieldValue, GffStruct, Value};
