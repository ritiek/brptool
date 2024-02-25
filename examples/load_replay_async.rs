use anyhow::Result;
use tokio::fs::File;

use brp_tool::async_brp::{get_header, load_replay_message};

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {
    #[arg(short, long)]
    pub path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    let mut file = File::open(args.path).await?;
    let header = get_header(&mut file).await?;

    dbg!(header.file_id);
    dbg!(header.protocol_version);

    let mut message;

    loop {
        message = load_replay_message(&mut file).await?;
        dbg!(message);
    }
}
