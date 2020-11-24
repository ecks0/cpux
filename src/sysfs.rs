use {
  std::{
    fs::read_to_string,
    path::PathBuf,
  },
  log::trace,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {

  #[error("{0}: {1}")]
  Io(PathBuf, std::io::Error),

  #[error("Error parsing value `{1}` in file {0}")]
  Parse(PathBuf, String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn cpu_ids() -> Result<Vec<u64>> {
  let path = cpu_present();
  trace!("sysfs cpu_ids {}", path.display());
  let val = read_to_string(&path).map_err(|e| Error::Io(path.clone(), e))?;
  Ok(crate::cli::parse_indices(val.trim_end()).ok_or(Error::Parse(path, val))?)
}

fn cpu_present() -> PathBuf {
  PathBuf::from("/sys/devices/system/cpu/present")
}

fn cpu(cpu_id: u64) -> PathBuf {
  PathBuf::from(format!("/sys/devices/system/cpu/cpu{}", cpu_id))
}

fn cpu_freq(cpu_id: u64) -> PathBuf {
  let mut p = cpu(cpu_id);
  p.push("cpufreq");
  p
}

pub fn cpu_freq_ep_pref(cpu_id: u64) -> PathBuf {
  let mut p = cpu_freq(cpu_id);
  p.push("energy_performance_preference");
  p
}

pub fn cpu_freq_ep_prefs(cpu_id: u64) -> PathBuf {
  let mut p = cpu_freq(cpu_id);
  p.push("energy_performance_available_preferences");
  p
}

pub fn cpu_freq_governor(cpu_id: u64) -> PathBuf {
  let mut p = cpu_freq(cpu_id);
  p.push("scaling_governor");
  p
}

pub fn cpu_freq_governors(cpu_id: u64) -> PathBuf {
  let mut p = cpu_freq(cpu_id);
  p.push("scaling_available_governors");
  p
}

pub fn cpu_freq_cur_khz(cpu_id: u64) -> PathBuf {
  let mut p = cpu_freq(cpu_id);
  p.push("scaling_cur_freq");
  p
}

pub fn cpu_freq_max_khz(cpu_id: u64) -> PathBuf {
  let mut p = cpu_freq(cpu_id);
  p.push("scaling_max_freq");
  p
}

pub fn cpu_freq_max_khz_limit(cpu_id: u64) -> PathBuf {
  let mut p = cpu_freq(cpu_id);
  p.push("cpuinfo_max_freq");
  p
}

pub fn cpu_freq_min_khz(cpu_id: u64) -> PathBuf {
  let mut p = cpu_freq(cpu_id);
  p.push("scaling_min_freq");
  p
}

pub fn cpu_freq_min_khz_limit(cpu_id: u64) -> PathBuf {
  let mut p = cpu_freq(cpu_id);
  p.push("cpuinfo_min_freq");
  p
}

pub fn cpu_online(cpu_id: u64) -> PathBuf {
  let mut p = cpu(cpu_id);
  p.push("online");
  p
}

pub fn cpu_epb(cpu_id: u64) -> PathBuf {
  let mut p = cpu(cpu_id);
  p.push("power");
  p.push("energy_perf_bias");
  p
}
