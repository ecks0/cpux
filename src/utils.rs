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
pub struct Indices(Vec<u64>);

impl Indices {

  pub fn from_vec(o: Vec<u64>) -> Self { Self(o) }

  pub fn dedup(&mut self) { self.0.dedup(); }

  pub fn iter(&self) -> IndicesIter { IndicesIter(Box::new(self.0.iter())) }

  pub fn sort(&mut self) { self.0.sort(); }

  pub fn to_vec(self) -> Vec<u64> { self.0 }
}

pub struct IndicesIter<'a>(Box<dyn Iterator<Item=&'a u64> + 'a>);

impl<'a> Iterator for IndicesIter<'a> {
  type Item = &'a u64;

  fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}

impl IntoIterator for Indices {
  type Item = u64;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
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

  pub fn iter(&self) -> TogglesIter { TogglesIter(Box::new(self.0.iter())) }
}

pub struct TogglesIter<'a>(Box<dyn Iterator<Item=&'a Option<bool>> + 'a>);

impl<'a> Iterator for TogglesIter<'a> {
  type Item = &'a Option<bool>;

  fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}

impl IntoIterator for Toggles {
  type Item = Option<bool>;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
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
