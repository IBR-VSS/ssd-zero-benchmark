use std::path::PathBuf;

use anyhow::{Result, anyhow};
use nix::{ioctl_readwrite, libc::c_int};

const LLZERO_MAGIC: u8 = 0x55;
ioctl_readwrite!(llzero_bench, LLZERO_MAGIC, 0x00, BenchArgs);

pub struct Benchmark {
    fd: c_int,
    args: BenchArgs,
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

const PARAM_PATH: &'static str = "/sys/module/page_zero/parameters";

pub fn is_enabled() -> Result<bool> {
    let mut path = PathBuf::from(PARAM_PATH);
    path.push("enabled");

    let status = std::fs::read(path)?;
    let status = str::from_utf8(&status)?.trim();

    let ret = match status {
        "Y" => true,
        _ => false,
    };

    Ok(ret)
}

pub fn enable() -> Result<()> {
    let mut path = PathBuf::from(PARAM_PATH);
    path.push("enabled");

    std::fs::write(path, "Y")?;
    Ok(())
}

pub fn disable() -> Result<()> {
    let mut path = PathBuf::from(PARAM_PATH);
    path.push("enabled");

    std::fs::write(path, "N")?;
    Ok(())
}

pub fn set_delay(delay_us: u32) -> Result<()> {
    let mut path = PathBuf::from(PARAM_PATH);
    path.push("delay");

    std::fs::write(path, delay_us.to_string())?;
    Ok(())
}

pub fn get_delay() -> Result<String> {
    let mut path = PathBuf::from(PARAM_PATH);
    path.push("delay");

    let delay = std::fs::read(path)?;
    let delay = str::from_utf8(&delay)?.to_string();
    Ok(delay)
}
