#[derive(thiserror::Error, Debug)]
pub enum Error {

  #[error("Error parsing frequency string: {0}")]
  ParseHertz(String),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum HertzUnit {
  Hz = 1,
  Khz = 1_000,
  Mhz = 1_000_000,
  Ghz = 1_000_000_000,
  Thz = 1_000_000_000_000,
}

impl HertzUnit {

  pub fn multiple(&self) -> u64 { self.clone() as u64 }
}

#[derive(Clone, Debug)]
pub struct Hertz(u64);

impl Hertz {

  pub fn new(hz: u64) -> Self { Self(hz) }

  pub fn from_khz(khz: f64) -> Self { Self((khz * HertzUnit::Khz.multiple() as f64) as u64) }

  pub fn from_mhz(mhz: f64) -> Self { Self((mhz * HertzUnit::Mhz.multiple() as f64) as u64) }

  pub fn from_ghz(ghz: f64) -> Self { Self((ghz * HertzUnit::Ghz.multiple() as f64) as u64) }

  pub fn from_thz(thz: f64) -> Self { Self((thz * HertzUnit::Thz.multiple() as f64) as u64) }

  pub fn is_hz(&self) -> bool { self.0 < HertzUnit::Khz.multiple() }

  pub fn is_khz(&self) -> bool { self.0 < HertzUnit::Mhz.multiple() }

  pub fn is_mhz(&self) -> bool { self.0 < HertzUnit::Ghz.multiple() }

  pub fn is_ghz(&self) -> bool { self.0 < HertzUnit::Thz.multiple() }

  pub fn is_thz(&self) -> bool { self.0 >= HertzUnit::Khz.multiple() }

  pub fn hz(&self) -> u64 { self.0.clone() }

  pub fn khz(&self) -> f64 { self.0 as f64 / HertzUnit::Khz.multiple() as f64 }

  pub fn mhz(&self) -> f64 { self.0 as f64 / HertzUnit::Mhz.multiple() as f64 }

  pub fn ghz(&self) -> f64 { self.0 as f64 / HertzUnit::Ghz.multiple() as f64 }

  pub fn thz(&self) -> f64 { self.0 as f64 / HertzUnit::Thz.multiple() as f64 }
}

impl From<u64> for Hertz { fn from(hz: u64) -> Self { Self::new(hz) } }

impl From<Hertz> for String { fn from(hz: Hertz) -> Self { hz.to_string() } }

impl AsRef<Hertz> for Hertz { fn as_ref(&self) -> &Hertz { self } }

impl std::str::FromStr for Hertz {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    let unit =
      match &(&s[s.len()-3..].to_lowercase())[..] {
        "khz" => HertzUnit::Khz,
        "mhz" => HertzUnit::Mhz,
        "ghz" => HertzUnit::Ghz,
        "thz" => HertzUnit::Thz,
        _ => HertzUnit::Hz,
      };
    if let HertzUnit::Hz = unit {
      Ok(Self(s.parse::<u64>().map_err(|e| Error::ParseHertz(s.to_string()))?))
    } else {
      let val = &s[..s.len()-3].parse::<f64>().map_err(|e| Error::ParseHertz(s.to_string()))?;
      Ok(Self((val * unit.multiple() as f64) as u64))
    }
  }
}

impl std::fmt::Display for Hertz {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let val =
      if self.is_hz() { format!("{} Hz", self.0) }
      else if self.is_khz() { format!("{:.1} KHz", self.khz()) }
      else if self.is_mhz() { format!("{:.1} MHz", self.mhz()) }
      else if self.is_ghz() { format!("{:.1} GHz", self.ghz()) }
      else { format!("{:.1} THz", self.thz()) };
    write!(f, "{}", val)
  }
}
