use std::error::Error as StdError;
use std::fmt;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct StorableStoreError(pub String);

impl fmt::Display for StorableStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for StorableStoreError {}

#[derive(Debug)]
pub struct StorableRestoreError(pub String);

impl fmt::Display for StorableRestoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for StorableRestoreError {}

pub trait Storable<W, R>
where
    W: Write,
    R: Read,
    Self: Sized,
{
    fn restore(reader: R) -> Result<Self, StorableRestoreError>;
    fn store(&self, writer: W) -> Result<(), StorableStoreError>;
}
