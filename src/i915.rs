use {
  crate::{
    drm,
    pseudofs,
    pseudofs::{Read, Write},
    sysfs,
    units::{Hertz, HertzUnit},
  },
  log::{debug, info},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {

  #[error(transparent)] CpuxDrm(#[from] crate::drm::Error),
  #[error(transparent)] CpuxPseudofs(#[from] crate::pseudofs::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

fn allow_missing_files<T>(result: Result<T>) -> Result<Option<T>> {
  match result {
    Ok(ok) => Ok(Some(ok)),
    Err(Error::CpuxPseudofs(err)) =>  Ok(pseudofs::allow_missing_files(Err(err))?),
    Err(err) => Err(err),
  }
}

pub fn available() -> bool {
  sysfs::i915_module().is_dir()
}

pub fn try_cards() -> Result<Vec<u64>> {
  let mut cards: Vec<u64> = vec![];
  for card_id in drm::try_cards()? {
    if drm::try_card_driver(card_id)?.eq("i915") {
      cards.push(card_id);
    }
  }
  Ok(cards)
}

pub fn cards() -> Result<Option<Vec<u64>>> {
  allow_missing_files(try_cards()) // FIXME
}

pub fn try_actual(card_id: u64) -> Result<Hertz> {
  let mhz = u64::read(&sysfs::i915_act_mhz(card_id))?;
  debug!("i915 get_actual_mhz card{} {}", card_id, mhz);
  Ok(Hertz::from_mhz(mhz as f64))
}

pub fn actual(card_id: u64) -> Result<Option<Hertz>> {
  Ok(allow_missing_files(try_actual(card_id))?)
}

pub fn try_boost(card_id: u64) -> Result<Hertz> {
  let mhz = u64::read(&sysfs::i915_boost_mhz(card_id))?;
  debug!("i915 get_boost_mhz card{} {}", card_id, mhz);
  Ok(Hertz::from_mhz(mhz as f64))
}

pub fn boost(card_id: u64) -> Result<Option<Hertz>> {
  Ok(allow_missing_files(try_boost(card_id))?)
}

pub fn try_set_boost<H: AsRef<Hertz>>(card_id: u64, val: H) -> Result<()> {
  let mhz = val.as_ref().mhz() as u64;
  info!("i915 set_boost_mhz card{} {}", card_id, mhz);
  mhz.write(&sysfs::i915_boost_mhz(card_id))?;
  Ok(())
}

pub fn set_boost<H: AsRef<Hertz>>(card_id: u64, val: H) -> Result<Option<()>> {
  Ok(allow_missing_files(try_set_boost(card_id, val))?)
}

pub fn try_max(card_id: u64) -> Result<Hertz> {
  let mhz = u64::read(&sysfs::i915_max_mhz(card_id))?;
  debug!("i915 get_max_mhz card{} {}", card_id, mhz);
  Ok(Hertz::from_mhz(mhz as f64))
}

pub fn max(card_id: u64) -> Result<Option<Hertz>> {
  Ok(allow_missing_files(try_max(card_id))?)
}

pub fn try_max_limit(card_id: u64) -> Result<Hertz> {
  let mhz = u64::read(&sysfs::i915_rp0_mhz(card_id))?;
  debug!("i915 get_max_mhz_limit card{} {}", card_id, mhz);
  Ok(Hertz::from_mhz(mhz as f64))
}

pub fn max_limit(card_id: u64) -> Result<Option<Hertz>> {
  Ok(allow_missing_files(try_max_limit(card_id))?)
}

pub fn try_set_max<H: AsRef<Hertz>>(card_id: u64, val: H) -> Result<()> {
  let mhz = val.as_ref().mhz() as u64;
  info!("i915 set_max_mhz card{} {}", card_id, mhz);
  mhz.write(&sysfs::i915_max_mhz(card_id))?;
  Ok(())
}

pub fn set_max<H: AsRef<Hertz>>(card_id: u64, val: H) -> Result<Option<()>> {
  Ok(allow_missing_files(try_set_max(card_id, val))?)
}

pub fn try_min(card_id: u64) -> Result<Hertz> {
  let mhz = u64::read(&sysfs::i915_min_mhz(card_id))?;
  debug!("i915 get_min_mhz card{} {}", card_id, mhz);
  Ok(Hertz::from_mhz(mhz as f64))
}

pub fn min(card_id: u64) -> Result<Option<Hertz>> {
  Ok(allow_missing_files(try_min(card_id))?)
}

pub fn try_min_limit(card_id: u64) -> Result<Hertz> {
  let mhz = u64::read(&sysfs::i915_rpn_mhz(card_id))?;
  debug!("i915 get_min_mhz_limit card{} {}", card_id, mhz);
  Ok(Hertz::from_mhz(mhz as f64))
}

pub fn min_limit(card_id: u64) -> Result<Option<Hertz>> {
  Ok(allow_missing_files(try_min_limit(card_id))?)
}

pub fn try_set_min<H: AsRef<Hertz>>(card_id: u64, val: H) -> Result<()> {
  let mhz = val.as_ref().mhz() as u64;
  info!("i915 set_min_mhz card{} {}", card_id, mhz);
  mhz.write(&sysfs::i915_min_mhz(card_id))?;
  Ok(())
}

pub fn set_min<H: AsRef<Hertz>>(card_id: u64, val: H) -> Result<Option<()>> {
  Ok(allow_missing_files(try_set_min(card_id, val))?)
}

pub fn try_requested(card_id: u64) -> Result<Hertz> {
  let mhz = u64::read(&sysfs::i915_cur_mhz(card_id))?;
  debug!("i915 get_requested_mhz card{} {}", card_id, mhz);
  Ok(Hertz::from_mhz(mhz as f64))
}

pub fn requested(card_id: u64) -> Result<Option<Hertz>> {
  Ok(allow_missing_files(try_requested(card_id))?)
}

pub fn try_optimum_limit(card_id: u64) -> Result<Hertz> {
  let mhz = u64::read(&sysfs::i915_rp1_mhz(card_id))?;
  debug!("i915 get_optimum_mhz_limit card{} {}", card_id, mhz);
  Ok(Hertz::from_mhz(mhz as f64))
}

pub fn optimum_limit(card_id: u64) -> Result<Option<Hertz>> {
  Ok(allow_missing_files(try_optimum_limit(card_id))?)
}
