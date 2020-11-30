#[derive(thiserror::Error, Debug)]
pub enum Error {

  #[error("Error parsing frequency string: {0}")]
  ParseHertz(String),

  #[error("Error parsing indices string: {0}")]
  ParseIndices(String),
  
  #[error("Error parsing on/off list from string: {0}")]
  ParseToggles(String),
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

  pub fn value(&self) -> u64 { self.clone() as u64 }
}

#[derive(Clone, Debug)]
pub struct Hertz(u64);

impl Hertz {

  fn new(hz: u64) -> Self { Self(hz) }

  pub fn format(hz: u64) -> String {
    Self::new(hz).to_string()
  }

  pub fn value(&self, unit: HertzUnit) -> f64 {
    if let HertzUnit::Hz = unit { self.0 as f64 }
    else { self.0 as f64 / unit.value() as f64 }
  }
}

impl std::str::FromStr for Hertz {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    let unit =
      if s.len() > 3 {
        let label = &(&s[s.len()-3..].to_lowercase())[..];
        match label {
          "khz" => HertzUnit::Khz,
          "mhz" => HertzUnit::Mhz,
          "ghz" => HertzUnit::Ghz,
          "thz" => HertzUnit::Thz,
          _ => HertzUnit::Hz,
        }
      } else {
        HertzUnit::Hz
      };
    if let HertzUnit::Hz = unit {
      Ok(Self(s.parse::<u64>().map_err(|e| Error::ParseHertz(s.to_string()))?))
    } else {
      let val = &s[..s.len()-3].parse::<f64>().map_err(|e| Error::ParseHertz(s.to_string()))?;
      Ok(Self((val * unit.value() as f64) as u64))
    }
  }
}

impl std::string::ToString for Hertz {

  fn to_string(&self) -> String { 
    if self.0 <= HertzUnit::Khz as u64 {
      format!("{} Hz", self.0)
    }
    else if self.0 <= HertzUnit::Mhz as u64 {
      format!("{:.1} KHz", self.value(HertzUnit::Khz))
    }
    else if self.0 <= HertzUnit::Ghz as u64 {
      format!("{:.1} MHz", self.value(HertzUnit::Mhz))
    }
    else if self.0 <= HertzUnit::Thz as u64 {
      format!("{:.1} GHz", self.value(HertzUnit::Ghz))
    }
    else {
      format!("{:.1} THz", self.value(HertzUnit::Thz))
    }
  }
}

#[derive(Clone, Debug)]
pub struct Indices(Vec<u64>);

impl Indices {

  pub fn from_vec(mut o: Vec<u64>) -> Self {
    o.sort();
    o.dedup();
    Self(o)
  }

  pub fn to_vec(self) -> Vec<u64> { self.0 }

  pub fn iter(&self) -> IndicesIter {
    IndicesIter(Box::new(self.0.iter()))
  }
}

pub struct IndicesIter<'a>(Box<dyn Iterator<Item=&'a u64> + 'a>);

impl<'a> Iterator for IndicesIter<'a> {
  type Item = &'a u64;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.next()
  }
}

impl IntoIterator for Indices {
  type Item = u64;

  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl std::str::FromStr for Indices {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    let mut ids: Vec<u64> = vec![];
    for part in s.split(',') {
      let val: Vec<&str> = part.split('-').collect();
      match &val[..] {
        &[id] =>
          match id.parse::<u64>() {
            Ok(val) => ids.push(val),
            Err(_) => return Err(Error::ParseIndices(s.to_string())),
          },
        &[first, last] =>
          std::ops::Range {
            start:
              match first.parse::<u64>() {
                Ok(val) => val,
                Err(_) => return Err(Error::ParseIndices(s.to_string())),
              },
            end:
              match last.parse::<u64>() {
                Ok(val) => 1 + val,
                Err(_) => return Err(Error::ParseIndices(s.to_string())),
              },
          }.for_each(|i| ids.push(i)),
        _ => return Err(Error::ParseIndices(s.to_string())),
      }
    }
    Ok(Self(ids))
  }
}

#[derive(Clone, Debug)]
pub struct Toggles(Vec<Option<bool>>);

impl Toggles {

  pub fn iter(&self) -> TogglesIter {
    TogglesIter(Box::new(self.0.iter()))
  }
}

pub struct TogglesIter<'a>(Box<dyn Iterator<Item=&'a Option<bool>> + 'a>);

impl<'a> Iterator for TogglesIter<'a> {
  type Item = &'a Option<bool>;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.next()
  }
}

impl<'a> IntoIterator for &'a Toggles {
  type Item = &'a Option<bool>;

  type IntoIter = TogglesIter<'a>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl std::str::FromStr for Toggles {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    let mut bits: Vec<Option<bool>> = vec![];
    for c in s.chars() {
      match c {
        '0' => bits.push(Some(false)),
        '1' => bits.push(Some(true)),
        '-' => bits.push(None),
        ' ' => continue,
        _ => return Err(Error::ParseToggles(s.to_string())),
      }
    }
    Ok(Self(bits))
  }
}
