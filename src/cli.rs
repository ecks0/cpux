use {
  clap::{
    App,
    crate_version,
    load_yaml,
  },
  crate::{
    cpu,
    cpufreq,
    intel_pstate as pstate,
  },
  fern,
  log::{LevelFilter, error},
  tabular::{Row, Table},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {

  #[error("{0}: error parsing value as {1}")]
  ParseValue(&'static str, &'static str),

  #[error(transparent)] CpuxCpu(#[from] crate::cpu::Error),
  #[error(transparent)] CpuxCpufreq(#[from] crate::cpufreq::Error),
  #[error(transparent)] CpuxIntelPstate(#[from] crate::intel_pstate::Error),
  #[error(transparent)] LogSetLogger(#[from] log::SetLoggerError),
}

type Result<T> = std::result::Result<T, Error>;

fn round(val: f64, scale: u8) -> f64 {
  let scale = scale as f64 * 100.0;
  (val * scale).round() / scale
}

fn parse_opt<T>(name: &'static str, value_kind: &'static str, opt: Option<&str>) -> Result<Option<T>>
where
  T: std::str::FromStr,
{
  match opt {
    None => Ok(None),
    Some(val) =>
      match val.parse::<T>() {
        Ok(val) => Ok(Some(val)),
        Err(_) => Err(Error::ParseValue(name, value_kind)),
      }
  }
}

pub(crate) fn parse_indices(text: &str) -> Option<Vec<u64>> {
  let mut ids: Vec<u64> = vec![];
  for part in text.split(',') {
    let val: Vec<&str> = part.split('-').collect();
    match &val[..] {
      &[id] =>
        match id.parse::<u64>() {
          Ok(val) => ids.push(val),
          Err(_) => return None,
        },
      &[first, last] =>
        std::ops::Range {
          start:
            match first.parse::<u64>() {
              Ok(val) => val,
              Err(_) => return None,
            },
          end:
            match last.parse::<u64>() {
              Ok(val) => 1 + val,
              Err(_) => return None,
            },
        }.for_each(|i| ids.push(i)),
      _ => return None,
    }
  }
  Some(ids)
}

fn parse_indices_opt(name: &'static str, opt: Option<&str>) -> Result<Option<Vec<u64>>> {
  match opt {
    None => Ok(None),
    Some(val) =>
      if val.eq("all") { Ok(None) }
      else {
        match parse_indices(val) {
          Some(val) => Ok(Some(val)),
          None => Err(Error::ParseValue(name, "e.g. `0,1,2-5,9,12-15`"))
        }
      },
  }
}

fn parse_bit_field(text: &str) -> Option<Vec<Option<bool>>> {
  let mut bits: Vec<Option<bool>> = vec![];
  for c in text.chars() {
    match c {
      '0' => bits.push(Some(false)),
      '1' => bits.push(Some(true)),
      '-' => bits.push(None),
      ' ' => continue,
      _ => return None,
    }
  }
  Some(bits)
}

fn parse_bit_field_opt(name: &'static str, opt: Option<&str>) -> Result<Option<Vec<Option<bool>>>> {
  match opt {
    None => Ok(None),
    Some(val) =>
      match parse_bit_field(val) {
        Some(val) => Ok(Some(val)),
        None => Err(Error::ParseValue(name, "e.g. 10-1 => 0:on 1:off 2:skip 3:on")),
      }
  }
}

fn parse_freq(text: &str) -> Option<u64> {
  let text = text.to_lowercase();
  if text.ends_with("ghz") {
    let ghz = match (&text[..text.len()-3]).parse::<f64>() {
      Ok(val) => val,
      Err(_) => return None,
    };
    Some((ghz * 1_000_000.0) as u64)
  }
  else if text.ends_with("mhz") {
    let mhz = match (&text[..text.len()-3]).parse::<f64>() {
      Ok(val) => val,
      Err(_) => return None,
    };
    Some((mhz * 1_000.0) as u64)
  }
  else {
    match text.parse::<u64>() {
      Ok(val) => Some(val),
      Err(_) => None,
    }
  }
}

fn parse_freq_opt(name: &'static str, opt: Option<&str>) -> Result<Option<u64>> {
  match opt {
    None => Ok(None),
    Some(val) =>
      match parse_freq(val) {
        Some(val) => Ok(Some(val)),
        None => Err(Error::ParseValue(name, "e.g. `4100000`, `4100mhz`, `4.1ghz`")),
      },
  }
}

fn format_khz(khz: u64) -> String {
  if khz < 1_000 { format!("{} khz", khz) }
  else if khz < 1_000_000 { format!("{:.1} mhz", round(khz as f64/1_000.0, 1)) }
  else { format!("{:.1} ghz", khz as f32/1_000_000_f32) }
}

fn indent(text: &str, level: usize) -> String {
  let i = std::iter::repeat(" ").take(level).collect::<String>();
  text
    .split('\n')
    .map(|s| format!("{}{}", i, s))
    .collect::<Vec<String>>()
    .join("\n")
  // FIXME
}

fn summarize_cpu(cpu_ids: Vec<u64>) -> Result<String> {
  let mut tab = Table::new("{:<} {:<} {:<} {:<} {:<} {:<} {:<}");
  tab.add_row(Row::new()
    .with_cell("CPU")
    .with_cell("Online")
    .with_cell("Cur")
    .with_cell("Min")
    .with_cell("Max")
    .with_cell("Min limit")
    .with_cell("Max limit"));
  tab.add_row(Row::new()
    .with_cell("-------")
    .with_cell("-------")
    .with_cell("-----------")
    .with_cell("-----------")
    .with_cell("-----------")
    .with_cell("-----------")
    .with_cell("-----------"));
  for cpu_id in cpu_ids {
    tab.add_row(Row::new()
      .with_cell(format!("cpu{}", cpu_id))
      .with_cell(cpu::online(cpu_id)?.unwrap_or(true))
      .with_cell(cpufreq::cur_khz(cpu_id)?.map(|v| format_khz(v)).unwrap_or("n/a".to_string()))
      .with_cell(cpufreq::min_khz(cpu_id)?.map(|v| format_khz(v)).unwrap_or("n/a".to_string()))
      .with_cell(cpufreq::max_khz(cpu_id)?.map(|v| format_khz(v)).unwrap_or("n/a".to_string()))
      .with_cell(cpufreq::min_khz_limit(cpu_id)?.map(|v| format_khz(v)).unwrap_or("n/a".to_string()))
      .with_cell(cpufreq::max_khz_limit(cpu_id)?.map(|v| format_khz(v)).unwrap_or("n/a".to_string())));
  }
  Ok(tab.to_string())
}

fn summarize_freq(cpu_ids: Vec<u64>) -> Result<String> {
  let mut tab = Table::new("{:<} {:<} {:<}");
  tab.add_row(Row::new()
    .with_cell("CPU")
    .with_cell("Governor")
    .with_cell("Governors"));
  tab.add_row(Row::new()
    .with_cell("-------")
    .with_cell("----------------")
    .with_cell("----------------"));
  for cpu_id in cpu_ids {
    tab.add_row(Row::new()
      .with_cell(format!("cpu{}", cpu_id))
      .with_cell(cpufreq::governor(cpu_id)?.unwrap_or("n/a".to_string()))
      .with_cell(cpufreq::governors(cpu_id)?.map(|v| v.join(",")).unwrap_or("n/a".to_string())));
  }
  Ok(tab.to_string())
}

fn summarize_pstate(cpu_ids: Vec<u64>) -> Result<String> {
  let mut tab = Table::new("{:<} {:<} {:<} {:<}");
  tab.add_row(Row::new()
    .with_cell("CPU")
    .with_cell("EPB")
    .with_cell("EP Pref")
    .with_cell("EP Prefs"));
  tab.add_row(Row::new()
    .with_cell("--------")
    .with_cell("----")
    .with_cell("----------------")
    .with_cell("----------------"));
  for cpu_id in cpu_ids {
    tab.add_row(Row::new()
      .with_cell(format!("cpu{}", cpu_id))
      .with_cell(pstate::epb(cpu_id)?.map(|v| v.to_string()).unwrap_or("n/a".to_string()))
      .with_cell(pstate::epp(cpu_id)?.unwrap_or("n/a".to_string()))
      .with_cell(pstate::epps(cpu_id)?.map(|v| v.join(",")).unwrap_or("n/a".to_string())));
  }
  let mut res = String::new();
  res.push_str(&format!("intel_pstate: {}\n\n", pstate::status()?.unwrap_or("n/a".to_string())));
  res.push_str(&tab.to_string());
  Ok(res)
}

fn summarize(cpu_ids: &Vec<u64>, cpu: bool, freq: bool, pstate: bool) -> Result<String> {
  let mut sum = String::new();
  if pstate { sum.push_str(&summarize_pstate(cpu_ids.clone())?); sum.push_str("\n"); }
  if freq { sum.push_str(&summarize_freq(cpu_ids.clone())?); sum.push_str("\n"); }
  if cpu { sum.push_str(&summarize_cpu(cpu_ids.clone())?); }
  Ok(indent(&sum, 2))
}

fn setup_logging(flag: &'static str, level: &str) -> Result<()> {
  let level =
    match level {
      "error" => LevelFilter::Error,
      "warn" => LevelFilter::Warn,
      "info" => LevelFilter::Info,
      "debug" => LevelFilter::Debug,
      "trace" => LevelFilter::Trace,
      _ => Err(Error::ParseValue(flag, "error|warn|info|debug|trace"))?,
    };
  Ok(fern::Dispatch
    ::new()
    .format(|out, message, record| {
      out.finish(format_args!("{0: >5} {1}", record.level(), message))
    })
    .level(level)
    .filter(|m| m.target().starts_with("cpux"))
    .chain(std::io::stderr())
    .apply()?)
}

pub fn run<I, T>(it: I) -> Result<()> 
where
  I: IntoIterator<Item=T>,
  T: Into<std::ffi::OsString> + Clone, 
{
  let yaml = load_yaml!("cli.yaml");
  let m =
    App
      ::from(yaml)
      .version(crate_version!())
      .get_matches_from(it);
  setup_logging("--log-level", m.value_of("LOG_LEVEL").unwrap_or("warn"))?;
  let cpu_on = parse_opt::<bool>("-o/--cpu-on", "bool", m.value_of("CPU_ON"))?;
  let cpu_on_each = parse_bit_field_opt("-O/--cpu-on-each", m.value_of("CPU_ON_EACH"))?;
  let freq_gov = m.value_of("FREQ_GOV");
  let freq_max = parse_freq_opt("-x/--freq-max", m.value_of("FREQ_MAX"))?;
  let freq_min = parse_freq_opt("-n/--freq-min", m.value_of("FREQ_MIN"))?;
  let pstate_epb = parse_opt::<u64>("--pstate-epb", "u64", m.value_of("PSTATE_EPB"))?;
  let pstate_epp = m.value_of("PSTATE_EPP");
  let quiet = m.is_present("QUIET");
  let show_all = m.is_present("SHOW_ALL");
  let show_cpu = if show_all { true } else { m.is_present("SHOW_CPU") };
  let show_freq = if show_all { true } else { m.is_present("SHOW_FREQ") };
  let show_pstate = if show_all { true } else { m.is_present("SHOW_PSTATE") };
  let wait = parse_opt::<u64>("WAIT", "u64", m.value_of("WAIT"))?;
  let cpu_ids = cpu::ids()?;
  let has_work_flags_cpu = cpu_on.is_some() || cpu_on_each.is_some();
  let has_work_flags_freq = freq_gov.is_some() || freq_max.is_some() || freq_min.is_some();
  let has_work_flags_pstate = pstate_epb.is_some() || pstate_epp.is_some();
  let has_work_flags = has_work_flags_cpu || has_work_flags_freq || has_work_flags_pstate;
  if has_work_flags {
    let cpus =
      match parse_indices_opt("-c/--cpus", m.value_of("CPUS"))? {
        Some(val) => val,
        None => cpu_ids.clone(),
      };
    for cpu_id in cpus {
      let mut cpu_online = cpu::online(cpu_id)?.unwrap_or(true);
      if ! cpu_online { cpu::try_set_online(cpu_id, true)?; }
      if let Some(cpu_on) = cpu_on { cpu_online = cpu_on; }
      if let Some(freq_gov) = freq_gov { cpufreq::set_governor(cpu_id, freq_gov)?; }
      if let Some(freq_max) = freq_max { cpufreq::set_max_khz(cpu_id, freq_max)?; }
      if let Some(freq_min) = freq_min { cpufreq::set_min_khz(cpu_id, freq_min)?;}
      if let Some(pstate_epb) = pstate_epb { pstate::set_epb(cpu_id, pstate_epb)?; }
      if let Some(pstate_epp) = pstate_epp { pstate::set_epp(cpu_id, pstate_epp)?; }
      if ! cpu_online { cpu::set_online(cpu_id, false)?; }
    }
    if let Some(cpu_on_each) = cpu_on_each {
      for (cpu_id, status) in cpu_on_each.iter().enumerate() {
        if let Some(status) = status { cpu::set_online(cpu_id as u64, *status)?; }
      }
    }
  }
  let has_show_flags = show_cpu || show_freq || show_pstate;
  let print_summary =
    || -> Result<()> {
      let summary =
        if has_show_flags {  summarize(&cpu_ids, show_cpu, show_freq, show_pstate)? }
        else { summarize(&cpu_ids, true, true, has_work_flags_pstate)? };
      print!("\n{}\n", summary);
      Ok(())
    };
  if let Some(wait) = wait {
    let wait = if wait == 0 { 1 } else { wait };
    let interval = std::time::Duration::from_secs(wait);
    loop {
      print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
      print_summary()?;
      std::thread::sleep(interval);
    };
  } else {
    if has_show_flags || ! quiet { print_summary()?; }
  }
  Ok(())
}
