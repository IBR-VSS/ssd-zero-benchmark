use anyhow::Result;
use clap::Parser;
use csv::Writer;
use experiment::{Measurements, throughput_mib};
use llzero::Benchmark;
use std::fs;
use std::path::Path;
use std::{fs::File, os::fd::AsRawFd};

mod csv_writer;
mod experiment;
mod llzero;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("device"))]
    device: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
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

    println!("Preparing benchmark...");
    let prev_status = llzero::is_enabled()?;
    llzero::disable()?;
    let prev_delay: u32 = llzero::get_delay()?.trim().parse()?;
    llzero::set_delay(0)?;

    println!("Running benchmark...");
    let n_pages = 1000;
    let benchmark_name = "SSD-Zero";
    for p in 1..=10 {
        let iodepth = p * 10;
        let mut m = Measurements::new(benchmark_name, &args.device, iodepth);
        let const_sector = true;
        let mut bench = Benchmark::new(fd, n_pages, iodepth, const_sector)?;

        for _ in 0..5 {
            let duration = bench.run()?;
            let throughput = throughput_mib(n_pages as f32, duration)?;
            m.add_measurement(throughput);
        }

        let avg = m.mean().expect("Measurements must not be empty");
        let std_dev = m.std_dev().expect("Measurements must not be empty");
        m.write_to_csv(&mut wtr)?;
        println!("avg: {} MiB/s, std_dev: {}", avg, std_dev);
    }

    println!("Writing to {}...", csv_path.display());
    wtr.flush()?;

    println!("Benchmark finished. Restoring previous state..");
    llzero::set_delay(prev_delay)?;
    if prev_status {
        llzero::enable()?;
    }

    Ok(())
}
