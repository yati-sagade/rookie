use std::result;

#[derive(Debug)]
pub enum Error {
    OutOfRange,
    InvalidSquare,
}

pub type Result<T> = result::Result<T, Error>;
