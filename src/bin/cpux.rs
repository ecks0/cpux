use anyhow::Result;

fn main() -> Result<()> {
  Ok(cpux::cli::run(std::env::args())?)
}
