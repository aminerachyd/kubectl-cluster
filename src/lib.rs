use clap::Parser;
use config::cluster::connect_to_cluster;
use std::io::{self};

mod config;

#[derive(Default, Parser)]
pub struct Args {
    cluster_name: String,

    #[arg(short, long)]
    cluster_url: Option<String>,

    #[arg(short, long)]
    username: Option<String>,
}

pub fn run() -> Result<(), io::Error> {
    let args = Args::parse();

    let cli_config = config::cli_config::read_config()?;

    connect_to_cluster(args.cluster_name, cli_config)?;

    Ok(())
}
