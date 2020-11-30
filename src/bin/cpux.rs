use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
  Ok(cpux::cli::Cli::from_args().run()?)
}
