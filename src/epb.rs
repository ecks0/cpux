use {
  crate::{
    pseudofs,
    pseudofs::{read_u64, write_u64},
    sysfs::cpu_epb,
  },
  log::{debug, info, trace},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {

  #[error(transparent)] CpuxPseudofs(#[from] crate::pseudofs::Error),
  #[error(transparent)] CpuxSysfs(#[from] crate::sysfs::Error),
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

pub fn get(cpu_id: u64) -> Result<u64> {
  let res = read_u64(&cpu_epb(cpu_id))?;
  debug!("epb get cpu{} {}", cpu_id, res);
  Ok(res)
}

pub fn set(cpu_id: u64, val: u64) -> Result<()> {
  info!("epb set cpu{} {}", cpu_id, val);
  write_u64(&cpu_epb(cpu_id), val)?;
  Ok(())
}
