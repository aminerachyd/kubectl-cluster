use std::{
    io::{self, ErrorKind},
    os::unix::process::CommandExt,
    process::Command,
};

use serde::{Deserialize, Serialize};

use super::cli_config::CliConfig;

#[derive(Serialize, Deserialize, Debug)]
pub struct Cluster {
    name: String,
    url: String,
    username: String,
}

pub fn find_cluster<'a>(name: &str, clusters: &'a Vec<Cluster>) -> Option<&'a Cluster> {
    clusters.iter().find(|&c| c.name.eq(name))
}

fn add_cluster(c: Cluster, clusters: &mut Vec<Cluster>) {
    clusters.push(c)
}

pub fn connect_to_cluster(cluster_name: String, cli_config: CliConfig) -> Result<(), io::Error> {
    let cluster = find_cluster(&cluster_name, &cli_config.clusters);

    if cluster.is_none() {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            format!(
                "Cluster with name {} not found in config file",
                cluster_name
            ),
        ));
    }

    let cluster = cluster.unwrap();

    Command::new("oc")
        .arg("login")
        .arg(&cluster.url)
        .arg("-u")
        .arg(&cluster.username)
        .exec();

    Ok(())
}
