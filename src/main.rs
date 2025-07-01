use anyhow::Result;
use nix::ioctl_readwrite;
use std::{fs::File, os::fd::AsRawFd};

const PAGE_SIZE_MIB: f32 = 2.0;

#[repr(C)]
struct Args {
    reps: u32,
    pages_per_cycle: u32,
    duration_ns: u64,
}

const LLZERO_MAGIC: u8 = 0x55;
ioctl_readwrite!(llzero_bench, LLZERO_MAGIC, 0x00, Args);

// Runs benchmark and returns the duration in secs
fn benchmark(fd: nix::libc::c_int, reps: u32, pages_per_cycle: u32) -> Result<f32> {
    let mut args = Args {
        reps,
        pages_per_cycle,
        duration_ns: 0,
    };

    unsafe {
        llzero_bench(fd, &mut args)?;
    }
    let duration_s = args.duration_ns as f32 * 1e-9;
    Ok(duration_s)
}

fn main() -> Result<()> {
    let file = File::open("/dev/async-zero")?;
    let fd = file.as_raw_fd();

    println!("Running benchmark...");
    for p in 1..=10 {
        // let mut args = Args {
        //     reps: 1000,
        //     pages_per_cycle: p * 10,
        //     duration_ns: 0,
        // };

        let mut measurements: Vec<f32> = vec![];
        for _ in 0..5 {
            let duration = benchmark(fd, 1000, p * 10)?;
            measurements.push(duration);
        }
        let avg: f32 = measurements.iter().sum::<f32>() / measurements.len() as f32;
        let throughput = 1000 as f32 * PAGE_SIZE_MIB / avg;
        println!("{} MiB/s", throughput);
    }
    println!("Benchmark finished.");
    Ok(())
}
