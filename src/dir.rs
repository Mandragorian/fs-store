use std::io;
use std::fmt;

use std::io::{BufReader, BufWriter};

use std::hash::Hash;
use std::borrow::Borrow;

use std::path::Path;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fs::{read_dir, File, OpenOptions};

use crate::storable::*;

#[derive(Debug)]
pub enum Error {
    OSError(String),
    IOError(io::Error),
    NotFound(String),
    StoreError(String, String),
    RestoreError(String, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::OSError(s) => write!(f, "{}", format!("{}", s)),
            Error::IOError(e) => e.fmt(f),
            Error::NotFound(filename) => write!(f, "{}: Not Found", filename),
            Error::StoreError(filename, s) => write!(f, "{}: {}", filename, s),
            Error::RestoreError(filename, s) => write!(f, "{}: {}", filename, s),
        }
    }
}

impl StdError for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        match e {
            _ => Error::IOError(e),
        }
    }
}

type BufReadFile = BufReader<File>;
type BufWriteFile = BufWriter<File>;

#[derive(Debug, Eq, PartialEq)]
pub struct DirStorage<T>
where
    T: Storable<BufWriteFile, BufReadFile>,
{
    storage: HashMap<String, T>,
}

impl<T> Default for DirStorage<T>
where
    T: Storable<BufWriteFile, BufReadFile>,
{
    fn default() -> DirStorage<T> {
        let storage = HashMap::new();
        DirStorage::new(storage)
    }
}

impl<T> DirStorage<T>
where
    T: Storable<BufWriteFile, BufReadFile>,
{
    pub fn new(storage: HashMap<String, T>) -> DirStorage<T> {
        DirStorage {
            storage,
        }
    }

    pub fn restore(path_str: &str) -> Result<DirStorage<T>, Error> {
        let mut storage: HashMap<String, T> = HashMap::new();
        let path = Path::new(path_str);
        if path.is_dir() {
            for entry in read_dir(path)? {
                let entry = entry?;
                let file_path = entry.path();
                if entry.path().is_dir() {
                    continue;
                }
                let file = File::open(file_path)?;
                let reader = BufReader::new(file);
                let object = Storable::<BufWriteFile, BufReadFile>::restore(reader).map_err(|e| {
                    Error::RestoreError(entry.path().display().to_string(), e.0)
                })?;
                storage.insert(entry.file_name().into_string().unwrap().into(), object);
            }
        }
        let dirstor: DirStorage<T> = DirStorage { storage };
        Ok(dirstor)
    }

    pub fn store<D>(&self, dir_path_str: D) -> Result<(), Error>
    where
        D: AsRef<str>,
    {
        for path_str in self.storage.keys() {
            self.store_single(dir_path_str.as_ref(), path_str.as_str())?;
        }
        Ok(())
    }

    pub fn store_single<S, F>(&self,  dir_path_string: F, filename: S) -> Result<(), Error>
    where
        S: AsRef<str>,
        F: AsRef<str>,
    {
        let dir_path = Path::new(dir_path_string.as_ref());
        let storable = self.storage.get(filename.as_ref()).ok_or(Error::NotFound(String::from(filename.as_ref())))?;
        let new_path_buf = dir_path.join(filename.as_ref());
        let new_path = new_path_buf.as_path();
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(new_path)
            .or(Err(Error::OSError(
                "could not open/create new agenda file".to_string(),
            )))?;
        let writer = BufWriter::new(file);
        storable
            .store(writer)
            .map_err(|e| Error::RestoreError(new_path.display().to_string(), e.0))
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&T>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.storage.get(k)
    }

    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut T>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.storage.get_mut(k)
    }

    pub fn insert<S>(&mut self, k: S, v: T) -> Option<T>
    where
        S: Into<String>
    {
        self.storage.insert(k.into(), v)
    }

    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.storage.contains_key(k)
    }
}
