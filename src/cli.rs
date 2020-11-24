use {
  clap::{
    App,
    crate_version,
    load_yaml,
  },
  crate::{
    epb,
    freq,
    online,
    sysfs,
  },
  fern,
  log::{LevelFilter, error},
  tabular::{Row, Table},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {

  #[error("{0}: error parsing value as {1}")]
  ParseValue(&'static str, &'static str),

  #[error(transparent)] CpuxEpb(#[from] crate::epb::Error),
  #[error(transparent)] CpuxFreq(#[from] crate::freq::Error),
  #[error(transparent)] CpuxOnlin(#[from] crate::online::Error),
  #[error(transparent)] CpuxSysfs(#[from] crate::sysfs::Error),
  #[error(transparent)] LogSetLogger(#[from] log::SetLoggerError),
}

type Result<T> = std::result::Result<T, Error>;

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
    let ghz = match (&text[..text.len()-3]).parse::<f32>() {
      Ok(val) => val,
      Err(_) => return None,
    };
    Some((ghz * 1_000_000_f32) as u64)
  }
  else if text.ends_with("mhz") {
    let mhz = match (&text[..text.len()-3]).parse::<f32>() {
      Ok(val) => val,
      Err(_) => return None,
    };
    Some((mhz * 1_000_f32) as u64)
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
  else if khz < 1_000_000 { format!("{:.1} mhz", khz as f32/1_000_f32) }
  else { format!("{:.1} ghz", khz as f32/1_000_000_f32) }
}

fn summarize_ep_pref(cpu_ids: Vec<u64>) -> String {
  let mut tab = Table::new("{:<} {:<} {:<}");
  tab.add_row(Row::new()
    .with_cell("CPU")
    .with_cell("EP Pref")
    .with_cell("EP Prefs"));
    tab.add_row(Row::new()
    .with_cell("--------")
    .with_cell("----------------")
    .with_cell("----------------"));
  for cpu_id in cpu_ids {
    tab.add_row(Row::new()
      .with_cell(format!("cpu{}", cpu_id))
      .with_cell(freq::ep_pref(cpu_id).ok().unwrap_or("n/a".to_string()))
      .with_cell(freq::ep_prefs(cpu_id).ok().map(|v| v.join(",")).unwrap_or("n/a".to_string())));
  }
  tab.to_string()
}

fn summarize_governor(cpu_ids: Vec<u64>) -> String {
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
      .with_cell(freq::governor(cpu_id).ok().unwrap_or("n/a".to_string()))
      .with_cell(freq::governors(cpu_id).ok().map(|v| v.join(",")).unwrap_or("n/a".to_string())));
  }
  tab.to_string()
}

fn summarize_cpu(cpu_ids: Vec<u64>) -> String {
  let mut tab = Table::new("{:<} {:<} {:<} {:<} {:<} {:<} {:<} {:<}");
  tab.add_row(Row::new()
    .with_cell("CPU")
    .with_cell("Online")
    .with_cell("Cur")
    .with_cell("Min")
    .with_cell("Max")
    .with_cell("Min limit")
    .with_cell("Max limit")
    .with_cell("EPB"));
  tab.add_row(Row::new()
    .with_cell("-------")
    .with_cell("-------")
    .with_cell("-----------")
    .with_cell("-----------")
    .with_cell("-----------")
    .with_cell("-----------")
    .with_cell("-----------")
    .with_cell("----"));
  for cpu_id in cpu_ids {
    tab.add_row(Row::new()
      .with_cell(format!("cpu{}", cpu_id))
      .with_cell(online::get(cpu_id).ok().map(|v| v.to_string()).unwrap_or("n/a".to_string()))
      .with_cell(freq::cur_khz(cpu_id).ok().map(|v| format_khz(v)).unwrap_or("n/a".to_string()))
      .with_cell(freq::min_khz(cpu_id).ok().map(|v| format_khz(v)).unwrap_or("n/a".to_string()))
      .with_cell(freq::max_khz(cpu_id).ok().map(|v| format_khz(v)).unwrap_or("n/a".to_string()))
      .with_cell(freq::min_khz_limit(cpu_id).ok().map(|v| format_khz(v)).unwrap_or("n/a".to_string()))
      .with_cell(freq::max_khz_limit(cpu_id).ok().map(|v| format_khz(v)).unwrap_or("n/a".to_string()))
      .with_cell(epb::get(cpu_id).ok().map(|v| v.to_string()).unwrap_or("n/a".to_string())));
  }
  tab.to_string()
}

fn summarize(cpu_ids: &Vec<u64>) -> String {
  let mut summary = String::new();
  summary.push_str("\n");
  summary.push_str(&summarize_ep_pref(cpu_ids.clone()));
  summary.push_str("\n");
  summary.push_str(&summarize_governor(cpu_ids.clone()));
  summary.push_str("\n");
  summary.push_str(&summarize_cpu(cpu_ids.clone()));
  summary.push_str("\n");
  summary
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
  let epb_hint = parse_opt::<u64>("--epb-hint", "u64", m.value_of("EPB_HINT"))?;
  let freq_ep_pref = m.value_of("FREQ_EP_PREF");
  let freq_gov = m.value_of("FREQ_GOV");
  let freq_max = parse_freq_opt("--freq-max", m.value_of("FREQ_MAX"))?;
  let freq_min = parse_freq_opt("--freq-min", m.value_of("FREQ_MIN"))?;
  let online = parse_opt::<bool>("--online", "bool", m.value_of("ONLINE"))?;
  let online_each = parse_bit_field_opt("--online-cpus", m.value_of("ONLINE_EACH"))?;
  let wait = parse_opt::<u64>("WAIT", "u64", m.value_of("WAIT"))?;
  let cpu_ids = sysfs::cpu_ids()?;
  let has_work_flags =
    epb_hint.is_some() ||
    freq_ep_pref.is_some() ||
    freq_gov.is_some() ||
    freq_max.is_some() ||
    freq_min.is_some() ||
    online.is_some() ||
    online_each.is_some();
  if has_work_flags {
    let cpus =
      match parse_indices_opt("--cpus", m.value_of("CPUS"))? {
        Some(val) => val,
        None => cpu_ids.clone(),
      };
    for cpu_id in cpus {
      let mut cpu_online = online::or(online::get(cpu_id), true)?;
      if ! cpu_online { online::ok(online::set(cpu_id, true))?; }
      if let Some(epb_hint) = epb_hint { epb::ok(epb::set(cpu_id, epb_hint))?; }
      if let Some(freq_ep_pref) = freq_ep_pref { freq::ok(freq::set_ep_pref(cpu_id, freq_ep_pref))?; }
      if let Some(freq_gov) = freq_gov { freq::ok(freq::set_governor(cpu_id, freq_gov))?; }
      if let Some(freq_max) = freq_max { freq::ok(freq::set_max_khz(cpu_id, freq_max))?; }
      if let Some(freq_min) = freq_min { freq::ok(freq::set_min_khz(cpu_id, freq_min))?; }
      if let Some(online) = online { cpu_online = online; }
      if ! cpu_online { online::ok(online::set(cpu_id, false))?; }
    }
    if let Some(online_each) = online_each {
      for (cpu_id, status) in online_each.iter().enumerate() {
        if let Some(status) = status { online::ok(online::set(cpu_id as u64, *status))?; }
      }
    }
  }
  if let Some(wait) = wait {
    let wait = if wait == 0 { 1 } else { wait };
    let interval = std::time::Duration::from_secs(wait);
    loop {
      let now = std::time::Instant::now();
      print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
      print!("{}", summarize(&cpu_ids));
      std::thread::sleep(interval - now.elapsed());
    };
  } else {
    if ! m.is_present("QUIET") { print!("{}", summarize(&cpu_ids)); }
  }
  Ok(())
}
