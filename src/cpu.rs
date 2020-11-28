use {
  crate::{
    pseudofs,
    pseudofs::{
      read_bool,
      read_str,
      write_bool,
    },
    sysfs::{cpu, cpu_present, cpu_online},
  },
  log::{debug, info, trace},
  std::path::PathBuf,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {

  #[error("Error parsing value `{1}` in file {0}")]
  Parse(PathBuf, String),

  #[error(transparent)] CpuxPseudofs(#[from] crate::pseudofs::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

fn allow_missing_if_cpu_exists<T>(cpu_id: u64, result: Result<T>) -> Result<Option<T>> {
  match result {
    Ok(val) => Ok(Some(val)),
    Err(Error::CpuxPseudofs(err)) => {
      if let pseudofs::Error::IoNotFound(_, _) = err {
        if ! exists(cpu_id) { return Err(Error::CpuxPseudofs(err)); }
      }
      Ok(pseudofs::allow_missing_files(Err(err))? )
    },
    Err(err) => Err(err),
  }
}

pub fn exists(cpu_id: u64) -> bool {
  cpu(cpu_id).is_dir()
}

pub fn ids() -> Result<Vec<u64>> {
  let path = cpu_present();
  let val = read_str(&path)?;
  debug!(r#"cpu get_ids "{}""#, val);
  let val = crate::cli::parse_indices(val.trim_end()).ok_or(Error::Parse(path, val))?;
  Ok(val)
}

pub fn try_online(cpu_id: u64) -> Result<bool> {
  let res = read_bool(&cpu_online(cpu_id))?;
  debug!("cpu get_online cpu{} {}", cpu_id, res);
  Ok(res)
}

pub fn online(cpu_id: u64) -> Result<Option<bool>> {
  Ok(allow_missing_if_cpu_exists(cpu_id, try_online(cpu_id))?)
}

pub fn try_set_online(cpu_id: u64, val: bool) -> Result<()> {
  info!("cpu set_online cpu{} {}", cpu_id, val);
  write_bool(&cpu_online(cpu_id), val)?;
  Ok(())
}

pub fn set_online(cpu_id: u64, val: bool) -> Result<Option<()>> {
  Ok(allow_missing_if_cpu_exists(cpu_id, try_set_online(cpu_id, val))?)
}
