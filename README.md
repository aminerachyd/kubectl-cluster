## oc-cluster plugin to connect to OpenShift clusters

This plugin allows you to connect to OpenShift clusters from the command line.

### Installation

To install, clone the repo and run:

```bash
make install
```

### Configuration

The plugin stores configuration on `~/.config/oc-cluster/clusters`. This file is a YAML file that contains a list of clusters. Each cluster has the following fields:

- `name`: The name of the cluster. This is used to identify the cluster in the command line.
- `url`: The URL of the cluster.
- `username`: The username to use to connect to the cluster.

This file is created automatically when you connect to a cluster for the first time.

### Usage

To connect to a cluster, run:

```bash
oc cluster <CLUSTER_NAME>
```

When connecting to a cluster for the first time, you need to specify the URL and the username for the cluster, the plugin will prompt for the password.

```bash
oc cluster <CLUSTER_NAME> --cluster-url <CLUSTER_API_URL> --username <USERNAME>
```
