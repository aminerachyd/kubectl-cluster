use clap::Parser;
use kubectl_cluster::{connect_to_cluster, Args};

fn main() {
    let args = Args::parse();

    let output = connect_to_cluster(args);

    dbg!(output);
}
