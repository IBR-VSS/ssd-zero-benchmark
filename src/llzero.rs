use anyhow::{anyhow, Result};
use nix::{ioctl_readwrite, libc::c_int};

const LLZERO_MAGIC: u8 = 0x55;
ioctl_readwrite!(llzero_bench, LLZERO_MAGIC, 0x00, BenchArgs);

pub struct Benchmark {
    fd: c_int,
    args: BenchArgs,
}

impl Benchmark {
    pub fn new(fd: c_int, n_pages: u32, iodepth: u32, const_sector: bool) -> Result<Benchmark> {
        if iodepth > 128 {
            return Err(anyhow!("Max iodepth: 128"));
        }
        let args = BenchArgs {
            n_pages,
            iodepth,
            const_sector,
            duration_ns: 0,
        };
        Ok(Benchmark { fd, args })
    }

    pub fn run(&mut self) -> Result<f32> {
        unsafe {
            llzero_bench(self.fd, &mut self.args)?;
        }
        let duration_s = self.get_duration_ns() as f32 * 1e-9;
        Ok(duration_s)
    }

    fn get_duration_ns(&self) -> u64 {
        self.args.duration_ns
    }
}

/// ioctl arguments
#[repr(C)]
pub struct BenchArgs {
    n_pages: u32,
    iodepth: u32,
    const_sector: bool,
    /// ioctl output
    duration_ns: u64,
}
