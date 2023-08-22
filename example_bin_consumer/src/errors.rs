use protobuf_json_mapping::PrintError;
use thiserror::Error;

pub type ExampleResult<T> = std::result::Result<T, ExampleError>;

#[derive(Error, Debug)]
pub enum ExampleError {
    #[error("I/O Error")]
    IOError(#[from] std::io::Error),
    #[error("Print Proto To Json Error")]
    PrintProtoJsonError(#[from] PrintError),
    #[error("Deserialize Json To Proto Error")]
    ParseProtoJsonError(#[from] protobuf_json_mapping::ParseError),
    #[error("Protobuf error")]
    ProtobufError(#[from] protobuf::Error),
}
