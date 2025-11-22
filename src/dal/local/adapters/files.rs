use std::fs;
use std::io::{Read, Write};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use serde::{Serialize, de::DeserializeOwned};

/// Simple error type for the repository.
#[derive(Debug)]
pub enum RepoError {
    Io(std::io::Error),
    Serde(serde_json::Error),
}

impl From<std::io::Error> for RepoError {
    fn from(e: std::io::Error) -> Self {
        RepoError::Io(e)
    }
}

impl From<serde_json::Error> for RepoError {
    fn from(e: serde_json::Error) -> Self {
        RepoError::Serde(e)
    }
}

/// Generic CRUD trait.
pub trait CrudAdapter<T, Id> {
    fn create(&self, id: &Id, value: &T) -> Result<(), RepoError>;
    fn read(&self, id: &Id) -> Result<Option<T>, RepoError>;
    fn update(&self, id: &Id, value: &T) -> Result<(), RepoError>;
    fn delete(&self, id: &Id) -> Result<(), RepoError>;
    fn list(&self) -> Result<Vec<T>, RepoError>;
}

/// File-based adapter with:
/// - base_path: directory to store data (dynamic path)
/// - extension: file extension (dynamic extension)
pub struct FileAdapter<T> {
    base_path: PathBuf,
    extension: String,
    _marker: PhantomData<T>,
}

impl<T> FileAdapter<T> {
    /// `base_path` = directory to store files
    /// `extension` = "json", "data", ".json", etc. (we normalize it)
    pub fn new<P: AsRef<Path>, S: AsRef<str>>(
        base_path: P,
        extension: S,
    ) -> Result<Self, std::io::Error> {
        let base_path = base_path.as_ref().to_path_buf();
        fs::create_dir_all(&base_path)?; // ensure directory exists

        let mut ext = extension.as_ref().to_string();
        if !ext.starts_with('.') {
            ext.insert(0, '.'); // ensure it starts with "."
        }

        Ok(Self {
            base_path,
            extension: ext,
            _marker: PhantomData,
        })
    }

    fn file_path_for_id<Id: std::fmt::Display>(&self, id: &Id) -> PathBuf {
        let mut path = self.base_path.clone();
        let file_name = format!("{id}{}", self.extension);
        path.push(file_name);
        path
    }
}

impl<T, Id> CrudAdapter<T, Id> for FileAdapter<T>
where
    T: Serialize + DeserializeOwned,
    Id: std::fmt::Display,
{
    fn create(&self, id: &Id, value: &T) -> Result<(), RepoError> {
        let path = self.file_path_for_id(id);

        if path.exists() {
            // if you want create-or-update, just drop this check
            return Err(RepoError::Io(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "file already exists",
            )));
        }

        let json = serde_json::to_vec_pretty(value)?;
        let mut file = fs::File::create(path)?;
        file.write_all(&json)?;
        Ok(())
    }

    fn read(&self, id: &Id) -> Result<Option<T>, RepoError> {
        let path = self.file_path_for_id(id);

        if !path.exists() {
            return Ok(None);
        }

        let mut file = fs::File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let value = serde_json::from_slice(&buf)?;
        Ok(Some(value))
    }

    fn update(&self, id: &Id, value: &T) -> Result<(), RepoError> {
        let path = self.file_path_for_id(id);

        if !path.exists() {
            return Err(RepoError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "file does not exist",
            )));
        }

        let json = serde_json::to_vec_pretty(value)?;
        let mut file = fs::File::create(path)?;
        file.write_all(&json)?;
        Ok(())
    }

    fn delete(&self, id: &Id) -> Result<(), RepoError> {
        let path = self.file_path_for_id(id);

        if path.exists() {
            fs::remove_file(path)?;
        }

        Ok(())
    }

    fn list(&self) -> Result<Vec<T>, RepoError> {
        let mut result = Vec::new();

        for entry in fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == &self.extension[1..] {
                        let mut file = fs::File::open(&path)?;
                        let mut buf = Vec::new();
                        file.read_to_end(&mut buf)?;
                        let value = serde_json::from_slice(&buf)?;
                        result.push(value);
                    }
                }
            }
        }

        Ok(result)
    }
}
