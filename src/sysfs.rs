use {
  std::path::PathBuf,
};

pub fn cpu(cpu_id: u64) -> PathBuf {
  PathBuf::from(format!("/sys/devices/system/cpu/cpu{}", cpu_id))
}

pub fn cpu_present() -> PathBuf {
  PathBuf::from("/sys/devices/system/cpu/present")
}

pub fn cpu_cpufreq() -> PathBuf {
  PathBuf::from("/sys/devices/system/cpu/cpufreq")
}

pub fn cpufreq(cpu_id: u64) -> PathBuf {
  let mut p = cpu(cpu_id);
  p.push("cpufreq");
  p
}

pub fn cpufreq_cur_khz(cpu_id: u64) -> PathBuf {
  let mut p = cpufreq(cpu_id);
  p.push("scaling_cur_freq");
  p
}

pub fn cpufreq_max_khz(cpu_id: u64) -> PathBuf {
  let mut p = cpufreq(cpu_id);
  p.push("scaling_max_freq");
  p
}

pub fn cpufreq_governor(cpu_id: u64) -> PathBuf {
  let mut p = cpufreq(cpu_id);
  p.push("scaling_governor");
  p
}

pub fn cpufreq_governors(cpu_id: u64) -> PathBuf {
  let mut p = cpufreq(cpu_id);
  p.push("scaling_available_governors");
  p
}

pub fn cpufreq_max_khz_limit(cpu_id: u64) -> PathBuf {
  let mut p = cpufreq(cpu_id);
  p.push("cpuinfo_max_freq");
  p
}

pub fn cpufreq_min_khz(cpu_id: u64) -> PathBuf {
  let mut p = cpufreq(cpu_id);
  p.push("scaling_min_freq");
  p
}

pub fn cpufreq_min_khz_limit(cpu_id: u64) -> PathBuf {
  let mut p = cpufreq(cpu_id);
  p.push("cpuinfo_min_freq");
  p
}

pub fn cpu_online(cpu_id: u64) -> PathBuf {
  let mut p = cpu(cpu_id);
  p.push("online");
  p
}

pub fn intel_pstate() -> PathBuf {
  PathBuf::from("/sys/devices/system/cpu/intel_pstate")
}

pub fn intel_pstate_epb(cpu_id: u64) -> PathBuf {
  let mut p = cpu(cpu_id);
  p.push("power");
  p.push("energy_perf_bias");
  p
}

pub fn intel_pstate_epp(cpu_id: u64) -> PathBuf {
  let mut p = cpufreq(cpu_id);
  p.push("energy_performance_preference");
  p
}

pub fn intel_pstate_epps(cpu_id: u64) -> PathBuf {
  let mut p = cpufreq(cpu_id);
  p.push("energy_performance_available_preferences");
  p
}

pub fn intel_pstate_status() -> PathBuf {
  let mut p = intel_pstate();
  p.push("status");
  p
}

pub fn drm() -> PathBuf {
  PathBuf::from("/sys/class/drm")
}

pub fn drm_card(card_id: u64) -> PathBuf {
  let mut p = drm();
  p.push(format!("card{}", card_id));
  p
}

pub fn drm_card_driver(card_id: u64) -> PathBuf {
  let mut p = drm_card(card_id);
  p.push("device");
  p.push("driver");
  p
}

pub fn i915_module() -> PathBuf {
  PathBuf::from("/sys/module/i915")
}

pub fn i915_rp0_mhz(card_id: u64) -> PathBuf {
  let mut p = drm_card(card_id);
  p.push("gt_RP0_freq_mhz");
  p
}

pub fn i915_rp1_mhz(card_id: u64) -> PathBuf {
  let mut p = drm_card(card_id);
  p.push("gt_RP1_freq_mhz");
  p
}

pub fn i915_rpn_mhz(card_id: u64) -> PathBuf {
  let mut p = drm_card(card_id);
  p.push("gt_RPn_freq_mhz");
  p
}

pub fn i915_act_mhz(card_id: u64) -> PathBuf {
  let mut p = drm_card(card_id);
  p.push("gt_act_freq_mhz");
  p
}

pub fn i915_boost_mhz(card_id: u64) -> PathBuf {
  let mut p = drm_card(card_id);
  p.push("gt_boost_freq_mhz");
  p
}

pub fn i915_cur_mhz(card_id: u64) -> PathBuf {
  let mut p = drm_card(card_id);
  p.push("gt_cur_freq_mhz");
  p
}

pub fn i915_max_mhz(card_id: u64) -> PathBuf {
  let mut p = drm_card(card_id);
  p.push("gt_max_freq_mhz");
  p
}

pub fn i915_min_mhz(card_id: u64) -> PathBuf {
  let mut p = drm_card(card_id);
  p.push("gt_min_freq_mhz");
  p
}
