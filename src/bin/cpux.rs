fn main() -> anyhow::Result<()> {
  Ok(cpux::cli::run(std::env::args())?)
}
