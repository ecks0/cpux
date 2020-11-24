use {
  crate::{
    pseudofs,
    pseudofs::{
      read_str,
      read_str_list,
      read_u64,
      write_str,
      write_u64,
    },
    sysfs::{
      cpu_freq_cur_khz,
      cpu_freq_ep_pref,
      cpu_freq_ep_prefs,
      cpu_freq_governor,
      cpu_freq_governors,
      cpu_freq_max_khz,
      cpu_freq_max_khz_limit,
      cpu_freq_min_khz,
      cpu_freq_min_khz_limit,
    }
  },
  log::{debug, info, trace},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {

  #[error(transparent)] CpuxPseudofs(#[from] crate::pseudofs::Error),
  #[error(transparent)] CpuxSysfs(#[from] crate::sysfs::Error),
}

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

pub type Result<T> = std::result::Result<T, Error>;

pub fn ep_pref(cpu_id: u64) -> Result<String> {
  let res = read_str(&cpu_freq_ep_pref(cpu_id))?;
  debug!(r#"freq ep_pref cpu{} "{}""#, cpu_id, res);
  Ok(res)
}

pub fn set_ep_pref(cpu_id: u64, val: &str) -> Result<()> {
  info!(r#"freq set_ep_pref cpu{} "{}""#, cpu_id, val);
  write_str(&cpu_freq_ep_pref(cpu_id), val)?;
  Ok(())
}

pub fn ep_prefs(cpu_id: u64) -> Result<Vec<String>> {
  let res = read_str_list(&cpu_freq_ep_prefs(cpu_id))?;
  debug!(r#"freq ep_prefs cpu{} "{}""#, cpu_id, res.join(","));
  Ok(res)
}

pub fn governor(cpu_id: u64) -> Result<String> {
  let res = read_str(&cpu_freq_governor(cpu_id))?;
  debug!(r#"freq governor cpu{} "{}""#, cpu_id, res);
  Ok(res)
}

pub fn set_governor(cpu_id: u64, val: &str) -> Result<()> {
  info!(r#"freq set_governor cpu{} "{}""#, cpu_id, val);
  write_str(&cpu_freq_governor(cpu_id), val)?;
  Ok(())
}

pub fn governors(cpu_id: u64) -> Result<Vec<String>> {
  let res = read_str_list(&cpu_freq_governors(cpu_id))?;
  debug!(r#"freq governors cpu{} "{}""#, cpu_id, res.join(","));
  Ok(res)
}

pub fn cur_khz(cpu_id: u64) -> Result<u64> {
  let res = read_u64(&cpu_freq_cur_khz(cpu_id))?;
  debug!("freq max_khz cpu{} {}", cpu_id, res);
  Ok(res)
}

pub fn max_khz(cpu_id: u64) -> Result<u64> {
  let res = read_u64(&cpu_freq_max_khz(cpu_id))?;
  debug!("freq max_khz cpu{} {}", cpu_id, res);
  Ok(res)
}

pub fn max_khz_limit(cpu_id: u64) -> Result<u64> {
  let res = read_u64(&cpu_freq_max_khz_limit(cpu_id))?;
  debug!("freq max_khz_limit cpu{} {}", cpu_id, res);
  Ok(res)
}

pub fn set_max_khz(cpu_id: u64, val: u64) -> Result<()> {
  info!("freq set_max_khz cpu{} {}", cpu_id, val);
  write_u64(&cpu_freq_max_khz(cpu_id), val)?;
  Ok(())
}

pub fn min_khz(cpu_id: u64) -> Result<u64> {
  let res = read_u64(&cpu_freq_min_khz(cpu_id))?;
  debug!("freq min_khz cpu{} {}", cpu_id, res);
  Ok(res)
}

pub fn min_khz_limit(cpu_id: u64) -> Result<u64> {
  let res = read_u64(&cpu_freq_min_khz_limit(cpu_id))?;
  debug!("freq min_khz_limit cpu{} {}", cpu_id, res);
  Ok(res)
}

pub fn set_min_khz(cpu_id: u64, val: u64) -> Result<()> {
  info!("freq set_min_khz cpu{} {}", cpu_id, val);
  write_u64(&cpu_freq_min_khz(cpu_id), val)?;
  Ok(())
}
