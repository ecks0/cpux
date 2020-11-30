use {
  crate::{
    cpu,
    cpufreq,
    i915,
    intel_pstate as pstate,
    utils::{Hertz, HertzUnit, Indices, Toggles},
  },
  fern,
  log::{LevelFilter, error},
  tabular::{Row, Table},
  structopt::StructOpt,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {

  #[error(transparent)] CpuxCpu(#[from] crate::cpu::Error),
  #[error(transparent)] CpuxCpufreq(#[from] crate::cpufreq::Error),
  #[error(transparent)] CpuxDrmI915(#[from] crate::i915::Error),
  #[error(transparent)] CpuxIntelPstate(#[from] crate::intel_pstate::Error),
  #[error(transparent)] LogSetLogger(#[from] log::SetLoggerError),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, StructOpt)]
#[structopt(about="View and set CPU and related parameters.")]
pub struct Cli {

  #[structopt(short, long, value_name="indices", help="Target CPUs, default all, e.g. 0,1,2-5,9,12-15")]
  cpus: Option<Indices>,

  #[structopt(long, takes_value=false, help="Prints CPU online and frequency summary, default")]
  cpu: bool,

  #[structopt(short="o", long, value_name="bool", help="CPU online status, true or false (per --cpus)")]
  cpu_on: Option<bool>,

  #[structopt(short="O", long, value_name="list", help="CPU online status, e.g. 10-1 ⇒ 0=on 1=off 2=skip 3=on")]
  cpu_on_each: Option<Toggles>,

  #[structopt(long, takes_value=false, help="Prints CPU frequency governor summary, default if detected")]
  freq: bool,

  #[structopt(short="g", long, value_name="gov", help="Frequency governor (per --cpus)")]
  freq_gov: Option<String>,

  #[structopt(short="x", long, value_name="hz", help="Max frequency, e.g. 4100mhz, 4.1ghz (per --cpus)")]
  freq_max: Option<Hertz>,

  #[structopt(short="n", long, value_name="hz", help="Min frequency, e.g. 800mhz, 0.8ghz (per --cpus)")]
  freq_min: Option<Hertz>,

  #[structopt(long, takes_value=false, help="Prints Intel GPU driver summary, default if detected")]
  i915: bool,

  #[structopt(long, value_name="hz", help="Intel GPU boost frequency, e.g. 1100mhz, 1.1ghz")]
  i915_freq_boost: Option<Hertz>,

  #[structopt(long, value_name="hz", help="Intel GPU maximum frequency, e.g. 900mhz, 0.9ghz")]
  i915_freq_max: Option<Hertz>,

  #[structopt(long, value_name="hz", help="Intel GPU minimum frequency, e.g. 350mhz, 0.35ghz")]
  i915_freq_min: Option<Hertz>,

  #[structopt(long, value_name="level", help="Log level, default warn, e.g. error|warn|info|debug|trace")]
  log_level: Option<LevelFilter>,

  #[structopt(long, takes_value=false, help="Prints Intel pstate driver summary, default if detected")]
  pstate: bool,
  
  #[structopt(long, value_name="0-15", help="Intel pstate energy/performance bias hint (per --cpus)")]
  pstate_epb: Option<u64>,

  #[structopt(long, value_name="pref", help="Intel pstate energy/performance preference (per --cpus)")]
  pstate_epp: Option<String>,

  #[structopt(short, long, takes_value=false, help="Do not print the default summaries")]
  quiet: Option<bool>,

  #[structopt(name = "REFRESH", help="Refresh summaries every REFRESH seconds")]
  refresh: Option<u64>,
}

impl Cli {

  fn setup_logging(&self) -> Result<()> {
    Ok(fern::Dispatch
      ::new()
      .format(|out, message, record| {
        out.finish(format_args!("{0: >5} {1}", record.level(), message))
      })
      .level(self.log_level.clone().unwrap_or(LevelFilter::Warn))
      .filter(|m| m.target().starts_with("cpux"))
      .chain(std::io::stderr())
      .apply()?)
  }

  fn has_cpu_control_args(&self) -> bool {
    return
      self.cpu_on.is_some() ||
      self.cpu_on_each.is_some() ||
      self.freq_gov.is_some() ||
      self.freq_max.is_some() ||
      self.freq_min.is_some() ||
      self.pstate_epb.is_some() ||
      self.pstate_epp.is_some();
  }

  fn apply_cpu_controls(&self) -> Result<()> {
    if ! self.has_cpu_control_args() { return Ok(()); }
    let cpu_ids = if let Some(cpus) = self.cpus.clone() { cpus } else { Indices::from_vec(cpu::cpus()?) };
    for cpu_id in cpu_ids {
      let mut cpu_online = cpu::online(cpu_id)?.unwrap_or(true);
      if ! cpu_online { cpu::try_set_online(cpu_id, true)?; }
      if let Some(ref cpu_on) = self.cpu_on { cpu_online = *cpu_on; }
      if let Some(ref freq_gov) = self.freq_gov { cpufreq::set_governor(cpu_id, freq_gov)?; }
      if let Some(ref freq_max) = self.freq_max { cpufreq::set_max_khz(cpu_id, freq_max.value(HertzUnit::Khz) as u64)?; }
      if let Some(ref freq_min) = self.freq_min { cpufreq::set_min_khz(cpu_id, freq_min.value(HertzUnit::Khz) as u64)?; }
      if let Some(pstate_epb) = self.pstate_epb { pstate::set_epb(cpu_id, pstate_epb)?; }
      if let Some(ref pstate_epp) = self.pstate_epp { pstate::set_epp(cpu_id, pstate_epp)?; }
      if ! cpu_online { cpu::set_online(cpu_id, false)?; }
    }
    if let Some(ref cpu_on_each) = self.cpu_on_each {
      for (cpu_id, status) in cpu_on_each.iter().enumerate() {
        if let Some(status) = status { cpu::set_online(cpu_id as u64, *status)?; }
      }
    }
    Ok(())
  }

  fn has_i915_control_args(&self) -> bool {
    return
      self.i915_freq_boost.is_some() ||
      self.i915_freq_max.is_some() ||
      self.i915_freq_min.is_some();
  }

  fn apply_i915_controls(&self) -> Result<()> {
    if ! self.has_i915_control_args() { return Ok(()); }
    let cards = if let Ok(Some(cards)) = i915::cards() { cards } else { return Ok(()) };
    for card_id in cards {
      if let Some(ref i915_freq_boost) = self.i915_freq_boost { i915::set_boost_mhz(card_id, i915_freq_boost.value(HertzUnit::Mhz) as u64)?; }
      if let Some(ref i915_freq_max) = self.i915_freq_max { i915::set_max_mhz(card_id, i915_freq_max.value(HertzUnit::Mhz) as u64)?; }
      if let Some(ref i915_freq_min) = self.i915_freq_min { i915::set_min_mhz(card_id, i915_freq_min.value(HertzUnit::Mhz) as u64)?; }
    }
    Ok(())
  }

  fn apply_controls(&self) -> Result<()> {
    self.apply_cpu_controls()?;
    self.apply_i915_controls()?;
    Ok(())
  }

  fn format_table_cpu(cpu_ids: Vec<u64>) -> Result<String> {
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
        .with_cell(cpufreq::cur_khz(cpu_id)?.map(|v| Hertz::format(v)).unwrap_or("n/a".to_string()))
        .with_cell(cpufreq::min_khz(cpu_id)?.map(|v| Hertz::format(v)).unwrap_or("n/a".to_string()))
        .with_cell(cpufreq::max_khz(cpu_id)?.map(|v| Hertz::format(v)).unwrap_or("n/a".to_string()))
        .with_cell(cpufreq::min_khz_limit(cpu_id)?.map(|v| Hertz::format(v)).unwrap_or("n/a".to_string()))
        .with_cell(cpufreq::max_khz_limit(cpu_id)?.map(|v| Hertz::format(v)).unwrap_or("n/a".to_string())));
    }
    let mut buf = tab.to_string();
    buf.push('\n');
    Ok(buf)
  }
  
  fn format_table_freq(cpu_ids: Vec<u64>) -> Result<String> {
    if cpu_ids.len() == 0 { return Ok("".to_string()); }
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
    let mut buf = tab.to_string();
    buf.push('\n');
    Ok(buf)
  }
  
  fn format_table_i915(card_ids: Option<Vec<u64>>) -> Result<String> {
    let card_ids = if let Some(card_ids) = card_ids { card_ids } else { return Ok("".to_string()); };
    if card_ids.len() == 0 { return Ok("".to_string()); }
    let mut tab = Table::new("{:<} {:<} {:<} {:<} {:<} {:<} {:<} {:<} {:<}");
    tab.add_row(Row::new()
      .with_cell("Card")
      .with_cell("Driver")
      .with_cell("Actual")
      .with_cell("Req'd")
      .with_cell("Min")
      .with_cell("Max")
      .with_cell("Boost")
      .with_cell("Min limit")
      .with_cell("Max limit"));
    tab.add_row(Row::new()
      .with_cell("------")
      .with_cell("-------")
      .with_cell("--------")
      .with_cell("--------")
      .with_cell("--------")
      .with_cell("--------")
      .with_cell("--------")
      .with_cell("---------")
      .with_cell("---------"));
    for card_id in card_ids {
      tab.add_row(Row::new()
        .with_cell(format!("card{}", card_id))
        .with_cell(format!("i915"))
        .with_cell(i915::actual_mhz(card_id)?.map(|v| Hertz::format(v)).unwrap_or("n/a".to_string()))
        .with_cell(i915::requested_mhz(card_id)?.map(|v| Hertz::format(v)).unwrap_or("n/a".to_string()))
        .with_cell(i915::min_mhz(card_id)?.map(|v| Hertz::format(v)).unwrap_or("n/a".to_string()))
        .with_cell(i915::max_mhz(card_id)?.map(|v| Hertz::format(v)).unwrap_or("n/a".to_string()))
        .with_cell(i915::boost_mhz(card_id)?.map(|v| Hertz::format(v)).unwrap_or("n/a".to_string()))
        .with_cell(i915::min_mhz_limit(card_id)?.map(|v| Hertz::format(v)).unwrap_or("n/a".to_string()))
        .with_cell(i915::max_mhz_limit(card_id)?.map(|v| Hertz::format(v)).unwrap_or("n/a".to_string())));
    }
    let mut buf = tab.to_string();
    buf.push('\n');
    Ok(buf)
  }

  fn format_table_pstate(cpu_ids: Vec<u64>) -> Result<String> {
    if cpu_ids.len() == 0 { return Ok("".to_string()); }
    let mut tab = Table::new("{:<} {:<} {:<} {:<}");
    tab.add_row(Row::new()
      .with_cell("CPU")
      .with_cell("EPB")
      .with_cell("EP Pref")
      .with_cell("EP Prefs"));
    tab.add_row(Row::new()
      .with_cell("--------")
      .with_cell("----")
      .with_cell("--------------------")
      .with_cell("--------------------"));
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
    res.push('\n');
    Ok(res)
  }
  
  fn has_table_args(&self) -> bool {
    return
      self.cpu ||
      self.freq ||
      self.i915 ||
      self.pstate
  }

  fn format_tables(&self) -> Result<String> {

    fn indent(text: &str, level: usize) -> String {
      let i = std::iter::repeat(" ").take(level).collect::<String>();
      text
        .split('\n')
        .map(|s| format!("{}{}", i, s))
        .collect::<Vec<String>>()
        .join("\n")
      // FIXME
    }

    let cpu_ids = cpu::cpus()?;
    let has_table_args = self.has_table_args();
    let mut buf = String::new();
    buf.push('\n');
    if self.pstate || (! has_table_args && pstate::available()) 
      { buf.push_str(&Self::format_table_pstate(cpu_ids.clone())?); }
    if self.freq || (! has_table_args && cpufreq::available())
      { buf.push_str(&Self::format_table_freq(cpu_ids.clone())?); }
    if self.cpu || ! has_table_args
      { buf.push_str(&Self::format_table_cpu(cpu_ids.clone())?); }
    if self.i915 || (! has_table_args && i915::available())
      { buf.push_str(&Self::format_table_i915(i915::cards()?)?); }
    let mut buf = indent(&buf, 2).trim_end().to_string();
    buf.push_str("\n\n");
    Ok(buf)
  }

  fn print_tables(&self) -> Result<()> {
    print!("{}", self.format_tables()?);
    Ok(())
  }

  fn refresh(&self) -> Result<()> {
    let refresh = if let Some(refresh) = self.refresh { refresh } else { return Ok(()); };
    let refresh = if refresh == 0 { 1 } else { refresh };
    let refresh = std::time::Duration::from_secs(refresh);
    loop {
      print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
      self.print_tables()?;
      std::thread::sleep(refresh);
    }
  }

  pub fn run(&self) -> Result<()> {
    self.setup_logging()?;
    self.apply_controls()?;
    if self.refresh.is_some() {
      self.refresh()?;
    } else {
      self.print_tables()?;
    }
    Ok(())
  }
}
