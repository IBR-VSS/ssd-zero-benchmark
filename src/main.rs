use anyhow::Result;
use nix::ioctl_readwrite;
use std::{fs::File, os::fd::AsRawFd, time::Instant};

const PAGE_SIZE_MIB: f32 = 2.0;

#[repr(C)]
struct Args {
    reps: u32,
    pages_per_cycle: u32,
    duration_ns: u64,
}

const LLZERO_MAGIC: u8 = 0x55;
ioctl_readwrite!(llzero_bench, LLZERO_MAGIC, 0x00, Args);

fn main() -> Result<()> {
    let file = File::open("/dev/async-zero")?;
    let fd = file.as_raw_fd();

    for p in 1..=10 {
        let mut args = Args {
            reps: 1000,
            pages_per_cycle: p * 10,
            duration_ns: 0,
        };

        let mut measurements: Vec<f32> = vec![];
        for _ in 0..5 {
            let start = Instant::now();
            unsafe {
                llzero_bench(fd, &mut args)?;
            }
            let end = Instant::now();
            let duration = end.duration_since(start);
            measurements.push(duration.as_secs_f32());
        }
        let avg: f32 = measurements.iter().sum::<f32>() / measurements.len() as f32;
        let throughput = 1000 as f32 * PAGE_SIZE_MIB / avg;
        println!("{} MiB/s", throughput);
    }
    Ok(())
}
