use std::io;
use std::io::Error;
use failure::Fail;
use std::sync::{PoisonError, RwLockReadGuard, LockResult};
use std::fs::File;


#[derive(Fail, Debug)]
pub enum KvsError{
    /// the IO error
    #[fail(display = "IO error : {}",_0)]
    IO(#[cause] io::Error),

    /// the serde error
    #[fail(display = "serde error : {}", _0)]
    Serde(#[cause] serde_json::Error),

    /// the option is null error
    #[fail(display = "the option is None")]
    None,

}

impl From<serde_json::Error> for KvsError{
    fn from(e: serde_json::Error) -> Self {
        KvsError::Serde(e)
    }
}

impl From<io::Error> for KvsError{
    fn from(e: Error) -> Self {
        KvsError::IO(e)
    }
}

pub type KvsResult<T> = std::result::Result<T,KvsError>;