use std::fs::File;

use anyhow::Result;
use anyhow::anyhow;
use csv::Writer;
use serde::Serialize;

const PAGE_SIZE_MIB: f32 = 2.0;

#[derive(Debug, Default)]
pub struct Measurements {
    benchmark: String,
    device: String,
    iodepth: u32,
    measurements: Vec<f32>,
}

#[derive(Serialize)]
struct MeasurementCsv {
    benchmark: String,
    device: String,
    iodepth: u32,
    mean: f32,
    stderr: f32,
}

impl Measurements {
    pub fn new(bench_name: &str, device_name: &str, iodepth: u32) -> Self {
        let mut ret = Measurements::default();
        ret.benchmark = bench_name.to_string();
        ret.device = device_name.to_string();
        ret.iodepth = iodepth;
        ret
    }

    pub fn add_measurement(&mut self, measurement: f32) {
        self.measurements.push(measurement);
    }

    pub fn mean(&self) -> Option<f32> {
        let len = self.measurements.len();
        if len <= 0 {
            return None;
        }

        Some(self.measurements.iter().sum::<f32>() / len as f32)
    }

    pub fn std_dev(&self) -> Option<f32> {
        match (self.mean(), self.measurements.len()) {
            (Some(mean), count) => {
                let var = self
                    .measurements
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

    pub fn write_to_csv(&self, w: &mut Writer<File>) -> Result<()> {
        let mean = self.mean().expect("len must not be 0");
        let stderr = self.std_dev().expect("len must not be 0");

        let e = MeasurementCsv {
            benchmark: self.benchmark.clone(),
            device: self.device.clone(),
            iodepth: self.iodepth,
            mean,
            stderr,
        };

        w.serialize(e)?;
        Ok(())
    }
}

pub fn throughput_mib(num_pages: f32, duration_s: f32) -> Result<f32> {
    if duration_s <= 0.0 {
        return Err(anyhow!("Duration can't be zero!"));
    }
    Ok(num_pages * PAGE_SIZE_MIB / duration_s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_experiment() {
        let exp = Measurements::new("test", "device", 64);
        assert_eq!(exp.benchmark, "test");
        assert_eq!(exp.device, "device");
        assert_eq!(exp.iodepth, 64);
    }
}
