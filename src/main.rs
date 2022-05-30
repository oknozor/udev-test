use udev;
use std::io;

#[derive(Debug)]
enum GpuDriver {
    Amd,
    Nvidia,
}

fn get_gpus() -> io::Result<Vec<GpuDriver>> {
    let mut enumerator = udev::Enumerator::new()?;
    enumerator.match_subsystem("graphics")?;
    let mut drivers = vec![];

    for device in enumerator.scan_devices()? {
        if let Some(parent) = device.parent() {
            if let Some(driver) = parent.driver() {
                match driver.to_string_lossy().as_ref() {
                    "amdgpu" => drivers.push(GpuDriver::Amd),
                    "nvidia" => drivers.push(GpuDriver::Nvidia),
                    _ => {},
                };
            }
        }
    }

    Ok(drivers)
}

fn main() {
    let drivers = get_gpus();
    println!("{:?}", drivers);
}