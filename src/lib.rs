use clap::Parser;
use config::cluster::{add_cluster, connect_to_cluster};
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

    if args.cluster_url.is_some() && args.username.is_some() {
        add_cluster(
            args.cluster_name,
            args.cluster_url.unwrap(),
            args.username.unwrap(),
            cli_config,
        )?;

        Ok(())
    } else {
        connect_to_cluster(args.cluster_name, cli_config)?;

        Ok(())
    }
}
