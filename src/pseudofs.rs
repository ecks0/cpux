use {
  log::{debug, trace},
  std::path::{Path, PathBuf}
};

#[derive(thiserror::Error, Debug)]
pub enum Error {

  #[error("{0}: {1}")]
  Io(PathBuf, std::io::Error),

  #[error("Not found: {0}: {1}")]
  IoNotFound(PathBuf, std::io::Error),

  #[error("Permission denied: {0}: {1}")]
  IoNoPermission(PathBuf, std::io::Error),

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
        std::io::ErrorKind::NotFound => Err(Error::IoNotFound(path.to_path_buf(), err)),
        std::io::ErrorKind::PermissionDenied => Err(Error::IoNoPermission(path.to_path_buf(), err)),
        std::io::ErrorKind::Other => 
          match err.raw_os_error() {
            Some(6)  |  // ENXIO "No such device or address",
            Some(16)    // EBUSY "Resource busy"
              => Err(Error::IoNotFound(path.to_path_buf(), err)),
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
      match err {
        Error::IoNotFound(path, _) => {
          debug!("pseudofs NotFound {}", path.display());
          Ok(None)
        },
        Error::IoNoPermission(ref path, _) => {
          if path.is_file() || path.is_dir() { Err(err) }
          else {
            debug!("pseudofs NoPermission {}", path.display());
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

pub fn read_bool(path: &Path) -> Result<bool> {
  trace!("pseudofs read_bool {}", path.display());
  let val = read_to_string(path)?;
  let val = val.trim_end();
  match val {
    "0" => Ok(false),
    "1" => Ok(true),
    _ => Err(Error::ParseBool(path.display().to_string(), val.to_string())),
  }
}

pub fn read_str(path: &Path) -> Result<String> {
  trace!("pseudofs read_str {}", path.display());
  Ok(read_to_string(path)?.trim_end().to_string())
}

pub fn read_str_list(path: &Path) -> Result<Vec<String>> {
  trace!("pseudofs read_str_list {}", path.display());
  Ok(read_to_string(path)?
    .trim_end()
    .split(' ')
    .map(String::from)
    .collect())
}

pub fn read_u64(path: &Path) -> Result<u64> {
  trace!("pseudofs read_u64 {}", path.display());
  let val = read_to_string(path)?;
  let val = val.trim_end();
  match val.parse::<u64>() {
    Ok(val) => Ok(val),
    Err(_) => Err(Error::ParseU64(path.display().to_string(), val.to_string())),
  }
}

pub fn write_bool(path: &Path, val: bool) -> Result<()> {
  trace!("pseudofs write_bool {} {}", path.display(), val);
  write(path, if val { "1" } else { "0" })?;
  Ok(())
}

pub fn write_str(path: &Path, val: &str) -> Result<()> {
  trace!("pseudofs write_str {} {}", path.display(), val.replace('\n', "\\n"));
  write(path, val)?;
  Ok(())
}

pub fn write_u64(path: &Path, val: u64) -> Result<()> {
  trace!("pseudofs write_u64 {} {}", path.display(), val);
  write(path, &val.to_string())?;
  Ok(())
}
