use {
  crate::{
    pseudofs,
    pseudofs::{read_bool, write_bool},
    sysfs::cpu_online,
  },
  log::{debug, info, trace},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {

  #[error(transparent)] CpuxPseudofs(#[from] crate::pseudofs::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn or<T>(result: Result<T>, default: T) -> Result<T> {
  match &result {
    Err(Error::CpuxPseudofs(pseudofs::Error::IoNotFound(path, _))) => {
      trace!(
        "{}::or() FileNotFound, using default value for {} path: {}",
        module_path!(), if path.is_file() { "existing" } else { "non-existing" }, path.display());
      Ok(default)
    },
    Err(Error::CpuxPseudofs(pseudofs::Error::IoNoPermission(path, _))) => {
      if path.is_file() { result }
      else {
        trace!(
          "{}::or() FilePermissionDenied, using default value for non-existing path: {}",
          module_path!(), path.display());
        Ok(default)
      }
    },
    _ => result,
  }
}

pub fn ok(result: Result<()>) -> Result<()> {
  or(result, ())
}

pub fn get(cpu_id: u64) -> Result<bool> {
  let res = read_bool(&cpu_online(cpu_id))?;
  debug!("online get cpu{} {}", cpu_id, res);
  Ok(res)
}

pub fn set(cpu_id: u64, val: bool) -> Result<()> {
  info!("online set cpu{} {}", cpu_id, val);
  write_bool(&cpu_online(cpu_id), val)?;
  Ok(())
}
