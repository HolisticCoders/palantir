use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ResourceError {
    Io(io::Error),
    FailedToGetExePath,
}

impl From<io::Error> for ResourceError {
    fn from(other: io::Error) -> Self {
        ResourceError::Io(other)
    }
}

impl std::fmt::Display for ResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ResourceError::FailedToGetExePath => {
                write!(f, "Could not get application executable path.")
            }
            ResourceError::Io(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for ResourceError {}

pub struct Resources {
    root_path: PathBuf,
}

impl Resources {
    pub fn root_path(&self) -> PathBuf {
        self.root_path.clone()
    }

    pub fn from_relative_exe_path(rel_path: &Path) -> Result<Resources, ResourceError> {
        let exe_file_name =
            std::env::current_exe().map_err(|_| ResourceError::FailedToGetExePath)?;
        let exe_path = exe_file_name
            .parent()
            .ok_or(ResourceError::FailedToGetExePath)?;
        Ok(Resources {
            root_path: exe_path.join(rel_path),
        })
    }

    pub fn resource_name_to_path(&self, location: &str) -> PathBuf {
        let mut path: PathBuf = self.root_path.clone();

        for part in location.split("/") {
            path = path.join(part);
        }

        path
    }
}
