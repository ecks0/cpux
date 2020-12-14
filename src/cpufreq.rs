use {
  crate::{
    cpu,
    pseudofs,
    pseudofs::{Read, Write},
    sysfs,
    units::{Hertz, HertzUnit}
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
      if let pseudofs::Error::NotFound(_, _) = err {
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
  let res = String::read(&sysfs::cpufreq_governor(cpu_id))?;
  debug!(r#"cpufreq get_governor cpu{} "{}""#, cpu_id, res);
  Ok(res)
}

pub fn governor(cpu_id: u64) -> Result<Option<String>> {
  allow_missing_if_cpu_exists(cpu_id, try_governor(cpu_id))
}

pub fn try_set_governor(cpu_id: u64, val: &str) -> Result<()> {
  info!(r#"cpufreq set_governor cpu{} "{}""#, cpu_id, val);
  val.write(&sysfs::cpufreq_governor(cpu_id))?;
  Ok(())
}

pub fn set_governor(cpu_id: u64, val: &str) -> Result<Option<()>> {
  allow_missing_if_cpu_exists(cpu_id, try_set_governor(cpu_id, val))
}

pub fn try_governors(cpu_id: u64) -> Result<Vec<String>> {
  let res = Vec::read(&sysfs::cpufreq_governors(cpu_id))?;
  debug!(r#"cpufreq get_governors cpu{} "{}""#, cpu_id, res.join(","));
  Ok(res)
}

pub fn governors(cpu_id: u64) -> Result<Option<Vec<String>>> {
  allow_missing_if_cpu_exists(cpu_id, try_governors(cpu_id))
}

pub fn try_cur(cpu_id: u64) -> Result<Hertz> {
  let khz = u64::read(&sysfs::cpufreq_cur_khz(cpu_id))?;
  debug!("cpufreq get_cur_khz cpu{} {}", cpu_id, khz);
  Ok(Hertz::from_khz(khz as f64))
}

pub fn cur(cpu_id: u64) -> Result<Option<Hertz>> {
  allow_missing_if_cpu_exists(cpu_id, try_cur(cpu_id))
}

pub fn try_max(cpu_id: u64) -> Result<Hertz> {
  let khz = u64::read(&sysfs::cpufreq_max_khz(cpu_id))?;
  debug!("cpufreq get_max_khz cpu{} {}", cpu_id, khz);
  Ok(Hertz::from_khz(khz as f64))
}

pub fn max(cpu_id: u64) -> Result<Option<Hertz>> {
  allow_missing_if_cpu_exists(cpu_id, try_max(cpu_id))
}

pub fn try_max_limit(cpu_id: u64) -> Result<Hertz> {
  let khz = u64::read(&sysfs::cpufreq_max_khz_limit(cpu_id))?;
  debug!("cpufreq get_max_khz_limit cpu{} {}", cpu_id, khz);
  Ok(Hertz::from_khz(khz as f64))
}

pub fn max_limit(cpu_id: u64) -> Result<Option<Hertz>> {
  allow_missing_if_cpu_exists(cpu_id, try_max_limit(cpu_id))
}

pub fn try_set_max<H: AsRef<Hertz>>(cpu_id: u64, val: H) -> Result<()> {
  let khz = val.as_ref().khz() as u64;
  info!("cpufreq set_max_khz cpu{} {}", cpu_id, khz);
  khz.write(&sysfs::cpufreq_max_khz(cpu_id))?;
  Ok(())
}

pub fn set_max<H: AsRef<Hertz>>(cpu_id: u64, val: H) -> Result<Option<()>> {
  allow_missing_if_cpu_exists(cpu_id, try_set_max(cpu_id, val))
}

pub fn try_min(cpu_id: u64) -> Result<Hertz> {
  let khz = u64::read(&sysfs::cpufreq_min_khz(cpu_id))?;
  debug!("cpufreq get_min_khz cpu{} {}", cpu_id, khz);
  Ok(Hertz::from_khz(khz as f64))
}

pub fn min(cpu_id: u64) -> Result<Option<Hertz>> {
  allow_missing_if_cpu_exists(cpu_id, try_min(cpu_id))
}

pub fn try_min_limit(cpu_id: u64) -> Result<Hertz> {
  let khz = u64::read(&sysfs::cpufreq_min_khz_limit(cpu_id))?;
  debug!("cpufreq get_min_khz_limit cpu{} {}", cpu_id, khz);
  Ok(Hertz::from_khz(khz as f64))
}

pub fn min_limit(cpu_id: u64) -> Result<Option<Hertz>> {
  allow_missing_if_cpu_exists(cpu_id, try_min_limit(cpu_id))
}

pub fn try_set_min<H: AsRef<Hertz>>(cpu_id: u64, val: H) -> Result<()> {
  let khz = val.as_ref().khz() as u64;
  info!("cpufreq set_min_khz cpu{} {}", cpu_id, khz);
  khz.write(&sysfs::cpufreq_min_khz(cpu_id))?;
  Ok(())
}

pub fn set_min<H: AsRef<Hertz>>(cpu_id: u64, val: H) -> Result<Option<()>> {
  allow_missing_if_cpu_exists(cpu_id, try_set_min(cpu_id, val))
}
