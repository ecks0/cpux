use {
  log::{debug, trace},
  std::path::{Path, PathBuf},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {

  #[error("{0}: {1}")]
  Io(PathBuf, std::io::Error),

  #[error("{0}: {1}")]
  NotFound(PathBuf, std::io::Error),

  #[error("{0}: {1}")]
  NoPermission(PathBuf, std::io::Error),

  #[error("{0}: value could not be parsed as bool: `{1}`")]
  ParseBool(String, String),

  #[error("{0}: value could not be parsed as u64: `{1}")]
  ParseU64(String, String),
}

pub type Result<T> = std::result::Result<T, Error>;

fn handle_io_error<T>(path: &Path, result: std::io::Result<T>) -> Result<T> {
  match result {
    Ok(val) => Ok(val),
    Err(err) => {
      match err.kind() {
        std::io::ErrorKind::NotFound => Err(Error::NotFound(path.to_path_buf(), err)),
        std::io::ErrorKind::PermissionDenied => Err(Error::NoPermission(path.to_path_buf(), err)),
        std::io::ErrorKind::Other => 
          match err.raw_os_error() {
            Some(6)  | // ENXIO "No such device or address",
            Some(16)   // EBUSY "Resource busy"
              => Err(Error::NotFound(path.to_path_buf(), err)),
            _ => Err(Error::Io(path.to_path_buf(), err))
          },
        _ => Err(Error::Io(path.to_path_buf(), err))
      }
    },
  }
}

pub(crate) fn allow_missing_files<T>(result: Result<T>) -> Result<Option<T>> {
  match result {
    Ok(val) => Ok(Some(val)),
    Err(err) =>
      match &err {
        Error::NotFound(path, err_io) => {
          debug!("pseudofs NotFound {} {}", path.display(), err_io);
          Ok(None)
        },
        Error::NoPermission(ref path, err_io) => {
          if path.is_file() || path.is_dir() { Err(err) }
          else {
            debug!("pseudofs NoPermission {} {}", path.display(), err_io);
            Ok(None)
          }
        }
        _ => Err(err),
      },
  }
}

fn read_to_string(path: &Path) -> Result<String> {
  handle_io_error(path, std::fs::read_to_string(path))
}

fn write(path: &Path, data: &str) -> Result<()> {
  handle_io_error(path, std::fs::write(path, data))
}

pub trait Read {
  type Item;
  
  fn read(path: &Path) -> Result<Self::Item>;
}

pub trait Write {
  type Item;

  fn write(&self, path: &Path) -> Result<()>;
}

impl Read for bool {
  type Item = bool;

  fn read(path: &Path) -> Result<Self::Item> {
    trace!("pseudofs read_bool {}", path.display());
    let val = read_to_string(path)?;
    let val = val.trim_end();
    match val {
      "0" => Ok(false),
      "1" => Ok(true),
      _ => Err(Error::ParseBool(path.display().to_string(), val.to_string())),
    }
  }
}

impl Write for bool {
  type Item = bool;

  fn write(&self, path: &Path) -> Result<()> {
    trace!("pseudofs write_bool {} {}", path.display(), self);
    write(path, if self.eq(&true) { "1" } else { "0" })?;
    Ok(())
  }
}

impl Read for u64 {
  type Item = u64;

  fn read(path: &Path) -> Result<Self::Item> {
    trace!("pseudofs read_u64 {}", path.display());
    let val = read_to_string(path)?;
    let val = val.trim_end();
    match val.parse::<u64>() {
      Ok(val) => Ok(val),
      Err(_) => Err(Error::ParseU64(path.display().to_string(), val.to_string())),
    }
  }
}

impl Write for u64 {
  type Item = u64;

  fn write(&self, path: &Path) -> Result<()> {
    trace!("pseudofs write_u64 {} {}", path.display(), self);
    write(path, &self.to_string())?;
    Ok(())
  }
}

impl Read for String {
  type Item = String;

  fn read(path: &Path) -> Result<Self::Item> {
    trace!("pseudofs read_str {}", path.display());
    Ok(read_to_string(path)?.trim_end().to_string())
  }
}

impl<'a> Write for &'a str {
  type Item = &'a str;

  fn write(&self, path: &Path) -> Result<()> {
    trace!("pseudofs write_str {} {}", path.display(), self.replace('\n', "\\n"));
    write(path, self)?;
    Ok(())
  }
}

impl Read for Vec<String> {
  type Item = Vec<String>;

  fn read(path: &Path) -> Result<Self::Item> {
    trace!("pseudofs read_str_list {}", path.display());
    Ok(read_to_string(path)?
      .trim_end()
      .split(' ')
      .map(String::from)
      .collect())
    }
}
