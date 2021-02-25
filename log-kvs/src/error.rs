use failure::Fail;
use std::io;


/// Error Type of the kvs
#[derive(Debug,Fail)]
pub enum KvsError{
    #[fail(display = "{}",_0)]
    IO(#[cause] io::Error),

    #[fail(display = "{}",_0)]
    SerDe(#[cause] serde_json::Error),

    #[fail(display = "Key Not Found")]
    KeyNotFound,
    #[fail(display = "Unexpected Command Type")]
    UnexpectedCommandType
}

impl From<io::Error> for KvsError{
    fn from(err: io::Error) -> Self {
        KvsError::IO(err)
    }
}

impl From<serde_json::Error> for KvsError{
    fn from(err: serde_json::Error) -> Self {
        KvsError::SerDe(err)
    }
}

pub type KvResult<T> =Result<T,KvsError>;