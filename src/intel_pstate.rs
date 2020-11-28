use {
  crate::{
    cpu,
    pseudofs,
    pseudofs::{
      read_str,
      read_u64,
      read_str_list,
      write_str,
      write_u64,
    },
    sysfs,
  },
  log::{debug, info},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {

  #[error(transparent)] CpuxPseudofs(#[from] crate::pseudofs::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

fn allow_missing_if_cpu_exists<T>(cpu_id: u64, result: Result<T>) -> Result<Option<T>> {
  match result {
    Ok(val) => Ok(Some(val)),
    Err(Error::CpuxPseudofs(err)) => {
      if let pseudofs::Error::IoNotFound(_, _) = err {
        if ! cpu::exists(cpu_id) { return Err(Error::CpuxPseudofs(err)); }
      }
      Ok(pseudofs::allow_missing_files(Err(err))? )
    },
  }
}

pub fn try_epb(cpu_id: u64) -> Result<u64> {
  let res = read_u64(&sysfs::intel_pstate_epb(cpu_id))?;
  debug!("intel_pstate get_epb cpu{} {}", cpu_id, res);
  Ok(res)
}

pub fn epb(cpu_id: u64) -> Result<Option<u64>> {
  allow_missing_if_cpu_exists(cpu_id, try_epb(cpu_id))
}

pub fn try_set_epb(cpu_id: u64, val: u64) -> Result<()> {
  info!("intel_pstate set_epb cpu{} {}", cpu_id, val);
  write_u64(&sysfs::intel_pstate_epb(cpu_id), val)?;
  Ok(())
}

pub fn set_epb(cpu_id: u64, val: u64) -> Result<Option<()>> {
  allow_missing_if_cpu_exists(cpu_id, try_set_epb(cpu_id, val))
}

pub fn try_epp(cpu_id: u64) -> Result<String> {
  let res = read_str(&sysfs::intel_pstate_epp(cpu_id))?;
  debug!(r#"intel_pstate get_epp cpu{} "{}""#, cpu_id, res);
  Ok(res)
}

pub fn epp(cpu_id: u64) -> Result<Option<String>> {
  allow_missing_if_cpu_exists(cpu_id, try_epp(cpu_id))
}

pub fn try_set_epp(cpu_id: u64, val: &str) -> Result<()> {
  info!(r#"intel_pstate set_epp cpu{} "{}""#, cpu_id, val);
  write_str(&sysfs::intel_pstate_epp(cpu_id), val)?;
  Ok(())
}

pub fn set_epp(cpu_id: u64, val: &str) -> Result<Option<()>> {
  allow_missing_if_cpu_exists(cpu_id, try_set_epp(cpu_id, val))
}

pub fn try_epps(cpu_id: u64) -> Result<Vec<String>> {
  let res = read_str_list(&sysfs::intel_pstate_epps(cpu_id))?;
  debug!(r#"intel_pstate get_epps cpu{} "{}""#, cpu_id, res.join(","));
  Ok(res)
}

pub fn epps(cpu_id: u64) -> Result<Option<Vec<String>>> {
  allow_missing_if_cpu_exists(cpu_id, try_epps(cpu_id))
}

pub fn try_status() -> Result<String> {
  let res = read_str(&sysfs::intel_pstate_status())?;
  debug!(r#"intel_pstate get_status "{}""#, res);
  Ok(res)
}

pub fn status() -> Result<Option<String>> {
  match try_status() {
    Ok(val) => Ok(Some(val)),
    Err(Error::CpuxPseudofs(err)) => Ok(pseudofs::allow_missing_files(Err(err))?),
  }
}

pub fn try_set_status(val: &str) -> Result<()> {
  info!(r#"intel_pstate set_status "{}""#, val);
  write_str(&sysfs::intel_pstate_status(), val)?;
  Ok(())
}

pub fn set_status(val: &str) -> Result<Option<()>> {
  match try_set_status(val) {
    Ok(()) => Ok(Some(())),
    Err(Error::CpuxPseudofs(err)) => Ok(pseudofs::allow_missing_files(Err(err))?),
  }
}
