use {
  crate::sysfs,
  std::fs,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {

  #[error("Path name conversion error")]
  PathCodec,

  #[error("Bad path")]
  BadPath,

  #[error(transparent)] StdIo(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

fn allow_missing_files<T>(result: Result<T>) -> Result<Option<T>> {
  match result {
    Ok(ok) => Ok(Some(ok)),
    Err(Error::StdIo(err)) =>
      match err.kind() {
        std::io::ErrorKind::NotFound => Ok(None),
        // std::io::ErrorKind::Other =>
        //   match err.raw_os_error() {
        //     Some(6)  |  // ENXIO "No such device or address",
        //     Some(16)    // EBUSY "Resource busy"
        //       => Ok(None),
        //     _ => Err(err)?
        //   },
        _ => Err(err)?,
      },
    Err(err) => Err(err)?,
  }
}

pub fn try_cards() -> Result<Vec<u64>> {
  let mut cards: Vec<u64> = vec![];
  for ent in fs::read_dir(sysfs::drm())? {
    let ent = ent?.file_name();
    let ent = if let Some(ent) = ent.to_str() { ent } else { return Err(Error::PathCodec); };
    if ent.starts_with("card") {
      cards.push(if let Some(i) = (&ent[4..]).parse::<u64>().ok() { i } else { continue; }); // FIXME trace
    }
  }
  Ok(cards)
}

pub fn cards() -> Result<Option<Vec<u64>>> {
  let ents = if let Ok(e) = fs::read_dir(sysfs::drm()) { e } else { return Ok(None); };
  let mut cards: Vec<u64> = vec![];
  for ent in ents {
    let ent = if let Ok(ent) = ent { ent } else { continue; }; // FIXME trace
    let ent = ent.file_name();
    let ent = if let Some(ent) = ent.to_str() { ent } else { continue; }; // FIXME trace
    if ent.starts_with("card") {
      cards.push(if let Ok(i) = (&ent[4..]).parse::<u64>() { i } else { continue; }); // FIXME trace
    }
  }
  Ok(Some(cards))
}

pub fn try_card_driver(card_id: u64) -> Result<String> {
  let file_name = fs::read_link(&sysfs::drm_card_driver(card_id))?;
  let file_name = file_name.file_name();
  let file_name = if let Some(f) = file_name { f.to_str() } else { return Err(Error::BadPath); }; // FIXME details
  if let Some(f) = file_name { Ok(f.to_string()) } else { Err(Error::PathCodec) }
}

pub fn card_driver(card_id: u64) -> Result<Option<String>> {
  allow_missing_files(try_card_driver(card_id))
}
