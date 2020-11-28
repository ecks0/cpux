use {
  crate::{
    cpu,
    pseudofs,
    pseudofs::{
      read_str,
      read_str_list,
      read_u64,
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

pub fn available() -> bool {
  sysfs::cpu_cpufreq().is_dir()
}

pub fn try_governor(cpu_id: u64) -> Result<String> {
  let res = read_str(&sysfs::cpufreq_governor(cpu_id))?;
  debug!(r#"cpufreq get_governor cpu{} "{}""#, cpu_id, res);
  Ok(res)
}

pub fn governor(cpu_id: u64) -> Result<Option<String>> {
  allow_missing_if_cpu_exists(cpu_id, try_governor(cpu_id))
}

pub fn try_set_governor(cpu_id: u64, val: &str) -> Result<()> {
  info!(r#"cpufreq set_governor cpu{} "{}""#, cpu_id, val);
  write_str(&sysfs::cpufreq_governor(cpu_id), val)?;
  Ok(())
}

pub fn set_governor(cpu_id: u64, val: &str) -> Result<Option<()>> {
  allow_missing_if_cpu_exists(cpu_id, try_set_governor(cpu_id, val))
}

pub fn try_governors(cpu_id: u64) -> Result<Vec<String>> {
  let res = read_str_list(&sysfs::cpufreq_governors(cpu_id))?;
  debug!(r#"cpufreq get_governors cpu{} "{}""#, cpu_id, res.join(","));
  Ok(res)
}

pub fn governors(cpu_id: u64) -> Result<Option<Vec<String>>> {
  allow_missing_if_cpu_exists(cpu_id, try_governors(cpu_id))
}

pub fn try_cur_khz(cpu_id: u64) -> Result<u64> {
  let res = read_u64(&sysfs::cpufreq_cur_khz(cpu_id))?;
  debug!("cpufreq get_cur_khz cpu{} {}", cpu_id, res);
  Ok(res)
}

pub fn cur_khz(cpu_id: u64) -> Result<Option<u64>> {
  allow_missing_if_cpu_exists(cpu_id, try_cur_khz(cpu_id))
}

pub fn try_max_khz(cpu_id: u64) -> Result<u64> {
  let res = read_u64(&sysfs::cpufreq_max_khz(cpu_id))?;
  debug!("cpufreq get_max_khz cpu{} {}", cpu_id, res);
  Ok(res)
}

pub fn max_khz(cpu_id: u64) -> Result<Option<u64>> {
  allow_missing_if_cpu_exists(cpu_id, try_max_khz(cpu_id))
}

pub fn try_max_khz_limit(cpu_id: u64) -> Result<u64> {
  let res = read_u64(&sysfs::cpufreq_max_khz_limit(cpu_id))?;
  debug!("cpufreq get_max_khz_limit cpu{} {}", cpu_id, res);
  Ok(res)
}

pub fn max_khz_limit(cpu_id: u64) -> Result<Option<u64>> {
  allow_missing_if_cpu_exists(cpu_id, try_max_khz_limit(cpu_id))
}

pub fn try_set_max_khz(cpu_id: u64, val: u64) -> Result<()> {
  info!("cpufreq set_max_khz cpu{} {}", cpu_id, val);
  write_u64(&sysfs::cpufreq_max_khz(cpu_id), val)?;
  Ok(())
}

pub fn set_max_khz(cpu_id: u64, val: u64) -> Result<Option<()>> {
  allow_missing_if_cpu_exists(cpu_id, try_set_max_khz(cpu_id, val))
}

pub fn try_min_khz(cpu_id: u64) -> Result<u64> {
  let res = read_u64(&sysfs::cpufreq_min_khz(cpu_id))?;
  debug!("cpufreq get_min_khz cpu{} {}", cpu_id, res);
  Ok(res)
}

pub fn min_khz(cpu_id: u64) -> Result<Option<u64>> {
  allow_missing_if_cpu_exists(cpu_id, try_min_khz(cpu_id))
}

pub fn try_min_khz_limit(cpu_id: u64) -> Result<u64> {
  let res = read_u64(&sysfs::cpufreq_min_khz_limit(cpu_id))?;
  debug!("cpufreq get_min_khz_limit cpu{} {}", cpu_id, res);
  Ok(res)
}

pub fn min_khz_limit(cpu_id: u64) -> Result<Option<u64>> {
  allow_missing_if_cpu_exists(cpu_id, try_min_khz_limit(cpu_id))
}

pub fn try_set_min_khz(cpu_id: u64, val: u64) -> Result<()> {
  info!("cpufreq set_min_khz cpu{} {}", cpu_id, val);
  write_u64(&sysfs::cpufreq_min_khz(cpu_id), val)?;
  Ok(())
}

pub fn set_min_khz(cpu_id: u64, val: u64) -> Result<Option<()>> {
  allow_missing_if_cpu_exists(cpu_id, try_set_min_khz(cpu_id, val))
}
