use failure::Fail;
use std::io;


#[derive(Fail,Debug)]
pub enum KvsError{
    /// IO error
    #[fail(display = "IO error: {}", _0)]
    Io(#[cause] io::Error),
    /// parse error
    #[fail(display = "the command don't have enough args")]
    ParseLackArgs,
}

pub type KvsResult<T> =std::result::Result<T,KvsError>;