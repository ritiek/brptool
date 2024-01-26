use anyhow::Result;
use std::fs::File;

use brp_tool::brp::load_replay;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {
    /// Path to .brp file
    #[arg(short, long)]
    pub path: PathBuf,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let file = File::open(args.path)?;
    let messages = load_replay(file)?;
    dbg!(messages);

    Ok(())
}
