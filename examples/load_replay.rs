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
    let replay = load_replay(file)?;

    dbg!(replay.header.file_id);
    dbg!(replay.header.protocol_version);

    for message in replay.messages {
        dbg!(message);
    }

    Ok(())
}
