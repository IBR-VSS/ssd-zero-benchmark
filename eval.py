#!/usr/bin/env python3

import paramiko
import os
import pandas as pd
from plotnine import *

SSH_ALIAS = "debian-local"
REMOTE_CSV_PATH = "/root/bench/throughput.csv"
LOCAL_CSV_PATH = "./bench-results/throughput.csv"

def fetch_csv_via_ssh():
    # Load SSH config from ~/.ssh/config
    ssh_config_path = os.path.expanduser("~/.ssh/config")
    ssh_config = paramiko.SSHConfig()
    with open(ssh_config_path) as f:
        ssh_config.parse(f)

    host_config = ssh_config.lookup(SSH_ALIAS)
    hostname = host_config.get("hostname", SSH_ALIAS)
    username = host_config.get("user", "root")
    identityfile = host_config.get("identityfile", [None])[0]

    print(f"Connecting to {SSH_ALIAS} -> Hostname: {hostname}, User: {username}")

    client = paramiko.SSHClient()
    client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
    client.load_system_host_keys()
    client.connect(
        hostname,
        username=username,
        key_filename=identityfile,
    )

    # SFTP to fetch CSV file
    sftp = client.open_sftp()
    sftp.get(REMOTE_CSV_PATH, LOCAL_CSV_PATH)
    sftp.close()
    client.close()

    print(f"CSV fetched to {LOCAL_CSV_PATH}")

def plot_throughput(csv_path):
    df = pd.read_csv(csv_path)

    plot = (
        ggplot(df, aes(x="inflights", y="throughput_mean", color="benchmark")) +
        geom_line() +
        geom_point() +
        geom_errorbar(aes(ymin="throughput_mean - throughput_stderr",
                          ymax="throughput_mean + throughput_stderr")) +
        labs(
            title="SSD-Zeroing",
            x="Inflight",
            y="Throughput (MiB/s)",
            color="Benchmark",
        ) +
        coord_cartesian(ylim=(0, None))
    )

    plot.save("fig/throughput_plot.png")
    print("Plot saved to fig/throughput_plot.png")

def main():
    fetch_csv_via_ssh()
    plot_throughput(LOCAL_CSV_PATH)

if __name__ == "__main__":
    main()
