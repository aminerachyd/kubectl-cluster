use crate::config::cli_config::{write_config, CliConfig};
use prettytable::{format::TableFormat, Cell, Row, Table};
use serde::{Deserialize, Serialize};
use std::{
    io::{self, ErrorKind},
    os::unix::process::CommandExt,
    process::Command,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cluster {
    name: String,
    url: String,
    username: String,
    comment: String,
}

/**
 * Adds or updates a cluster in the config file
 *  - If the cluster doesn't exist, it adds it
 *  - If the cluster already exists (checks the name), updates it's url and username
 */
pub fn add_cluster(
    name: String,
    url: String,
    username: String,
    mut cli_config: CliConfig,
) -> Result<(), io::Error> {
    let cluster_exists = find_cluster(&name, &mut cli_config.clusters);

    if cluster_exists.is_some() {
        let cluster = cluster_exists.unwrap();

        cluster.url = url;

        cluster.username = username;
    } else {
        let cluster = Cluster {
            name: name.clone(),
            url,
            username,
            comment: "".to_owned(),
        };

        cli_config.clusters.push(cluster);
    }
    cli_config = write_config(cli_config)?;

    connect_to_cluster(name, cli_config)?;

    Ok(())
}

/**
 * Fetches the cluster from the config file and connects to it
 */
pub fn connect_to_cluster(
    cluster_name: String,
    mut cli_config: CliConfig,
) -> Result<(), io::Error> {
    let cluster = find_cluster(&cluster_name, &mut cli_config.clusters);

    if cluster.is_none() {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            format!(
                "Cluster with name {} not found in config file, consider adding arguments --username and --cluster-url to save it",
                cluster_name
            ),
        ));
    }

    let cluster = cluster.unwrap();

    connect_command(cluster);

    Ok(())
}

/**
 * Lists all clusters
 */
pub fn list_clusters(format: Option<String>, cli_config: CliConfig) -> Result<(), io::Error> {
    let clusters = cli_config.clusters;

    match format {
        Some(format) => match format.as_str() {
            "wide" => print_clusters_table(&clusters),
            _ => {
                unimplemented!();
            }
        },
        _ => print_clusters_table(&clusters),
    }

    Ok(())
}

/**
 * Deletes cluster with name if it exists in config
 */
pub fn delete_cluster(cluster_name: String, mut cli_config: CliConfig) -> Result<(), io::Error> {
    let clusters = &mut cli_config.clusters;

    let index_to_remove = clusters.iter().position(|c| c.name == cluster_name);

    match index_to_remove {
        Some(index) => {
            println!("Removing cluster [{}]", cluster_name);
            clusters.remove(index);
            write_config(cli_config)?;
        }
        None => {
            println!("Cluster [{}] doesn't exist", cluster_name);
        }
    }

    Ok(())
}

fn print_clusters_table(clusters: &Vec<Cluster>) {
    let table_headers = ("CLUSTER NAME", "USERNAME", "URL", "COMMENT");

    let mut table = Table::new();

    let mut table_format = TableFormat::new();

    table_format.padding(0, 2);

    table.set_format(table_format);

    table.add_row(Row::new(vec![
        Cell::new(table_headers.0),
        Cell::new(table_headers.1),
        Cell::new(table_headers.2),
        Cell::new(table_headers.3),
    ]));

    clusters.iter().for_each(|c| {
        table.add_row(Row::new(vec![
            Cell::new(&c.name),
            Cell::new(&c.username),
            Cell::new(&c.url),
            Cell::new(&c.comment),
        ]));
    });

    table.printstd();
}

fn find_cluster<'a>(name: &str, clusters: &'a mut Vec<Cluster>) -> Option<&'a mut Cluster> {
    clusters.iter_mut().find(|c| c.name.eq(name))
}

/**
 * Command to connect to the cluster
 * Uses `oc login`, perhaps use another method (login through rest api instead) ?
 */
fn connect_command(cluster: &Cluster) {
    Command::new("oc")
        .arg("login")
        .arg(&cluster.url)
        .arg("-u")
        .arg(&cluster.username)
        .exec();
}
