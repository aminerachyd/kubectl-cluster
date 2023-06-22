use clap::Parser;
use std::io::{self, ErrorKind};
use std::{os::unix::process::CommandExt, process::Command};

#[derive(Default, Parser)]
pub struct Args {
    cluster_name: String,

    #[arg(short, long)]
    cluster_url: Option<String>,

    #[arg(short, long)]
    username: Option<String>,
}

pub fn connect_to_cluster(args: Args) -> io::Error {
    if args.cluster_url.is_some() && args.username.is_some() {
        let cluster_url = args.cluster_url.unwrap();
        let username = args.username.unwrap();

        let login_args = ["login", &cluster_url, "-u", &username];

        Command::new("oc").args(login_args).exec()
    } else {
        println!("No username or cluster url given, aborting");
        return io::Error::from(ErrorKind::Other);
    }
}
