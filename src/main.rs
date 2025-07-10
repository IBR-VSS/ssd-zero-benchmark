use anyhow::anyhow;
use anyhow::Result;
use csv::Writer;
use llzero::Benchmark;
use serde::Serialize;
use std::fs;
use std::path::Path;
use std::{fs::File, os::fd::AsRawFd};

const PAGE_SIZE_MIB: f32 = 2.0;

mod llzero;

#[derive(Debug, Serialize)]
struct Experiment {
    benchmark: String,
    inflights: u32,
    throughput_mean: f32,
    throughput_stderr: f32,
}

fn mean(measurements: &[f32]) -> Option<f32> {
    let len = measurements.len();
    if len <= 0 {
        return None;
    }
    Some(measurements.iter().sum::<f32>() / len as f32)
}

fn std_dev(measurements: &[f32]) -> Option<f32> {
    match (mean(measurements), measurements.len()) {
        (Some(mean), count) => {
            let var = measurements
                .iter()
                .map(|value| {
                    let diff = mean - (*value as f32);
                    diff * diff
                })
                .sum::<f32>()
                / count as f32;

            Some(var.sqrt())
        }
        _ => None,
    }
}

fn throughput_mib(num_pages: f32, duration_s: f32) -> Result<f32> {
    if duration_s <= 0.0 {
        return Err(anyhow!("Duration can't be zero!"));
    }
    Ok(num_pages * PAGE_SIZE_MIB / duration_s)
}

fn main() -> Result<()> {
    let file = File::open("/dev/async-zero")?;
    let fd = file.as_raw_fd();
    let csv_path = Path::new("bench/throughput.csv");
    let bench_path = csv_path.parent().expect("Parent dir");

    if !fs::exists(bench_path)? {
        println!(
            "{} does not exist. Creating directory..",
            bench_path.display()
        );
        fs::create_dir(bench_path)?;
    }

    let mut wtr = Writer::from_path(csv_path)?;

    println!("Running benchmark...");
    let n_pages = 1000;
    let benchmark_name = "SSD-Zero".to_string();
    for p in 1..=10 {
        let mut measurements: Vec<f32> = vec![];
        let iodepth = p * 10;
        let const_sector = true;
        let mut bench = Benchmark::new(fd, n_pages, iodepth, const_sector)?;

        for _ in 0..5 {
            let duration = bench.run()?;
            let throughput = throughput_mib(n_pages as f32, duration)?;
            measurements.push(throughput);
        }

        let avg = mean(&measurements).expect("Measurements must not be empty");
        let std_dev = std_dev(&measurements).expect("Measurements must not be empty");
        wtr.serialize(Experiment {
            benchmark: benchmark_name.clone(),
            inflights: iodepth,
            throughput_mean: avg,
            throughput_stderr: std_dev,
        })?;
        println!("avg: {} MiB/s, std_dev: {}", avg, std_dev);
    }

    println!("Writing to {}...", csv_path.display());
    wtr.flush()?;
    println!("Benchmark finished.");
    Ok(())
}
