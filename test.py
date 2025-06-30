#!/usr/bin/env python3

import paramiko
import os
import re
import matplotlib.pyplot as plt

SSH_ALIAS = "debian-local"
BENCHMARK_CMD = "/root/benchmark"  # your remote command

throughput_pattern = re.compile(r"([\d.]+)\s+MiB/s")

def main():
    # Load SSH config from ~/.ssh/config
    ssh_config_path = os.path.expanduser("~/.ssh/config")
    ssh_config = paramiko.SSHConfig()
    with open(ssh_config_path) as f:
        ssh_config.parse(f)

    host_config = ssh_config.lookup(SSH_ALIAS)
    hostname = host_config.get("hostname", SSH_ALIAS)  # real hostname or fallback
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

    stdin, stdout, stderr = client.exec_command(BENCHMARK_CMD)
    output = stdout.read().decode()
    client.close()

    print("Benchmark output:")
    print(output)

    throughputs = []
    for line in output.splitlines():
        match = throughput_pattern.search(line)
        if match:
            throughputs.append(float(match.group(1)))

    if not throughputs:
        print("No throughput values found in output!")
        return

    x = [(i + 1) * 10 for i in range(len(throughputs))]


    plt.figure(figsize=(8, 5))
    plt.title("AsyncZero SSD")
    plt.xlabel("Huge Pages per Cycle")
    plt.ylabel("Throughput (MiB/s)")

    plt.axhline(y=1781, color='red', linestyle='--', linewidth=1.5, label='fio')
    plt.plot(x, throughputs, color='blue', label="AsyncZero")
    plt.legend()
    plt.grid(True)
    plt.tight_layout()

    os.makedirs("fig", exist_ok=True)
    plt.savefig("fig/throughput_remote.png")

if __name__ == "__main__":
    main()
