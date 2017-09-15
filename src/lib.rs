extern crate libc;

use libc::{int32_t};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
struct SweepDevice;

#[repr(C)]
struct SweepError;

#[repr(C)]
struct SweepScan;

#[link(name = "sweep")]
extern {
    fn sweep_error_message(error: *const SweepError) -> *const c_char;
    fn sweep_is_abi_compatible() -> bool;
    fn sweep_get_version() -> int32_t;
    fn sweep_device_construct_simple(port: *const c_char, error: *const *mut SweepError) -> *const SweepDevice;
    fn sweep_device_start_scanning(device: *const SweepDevice, error: *const *mut SweepError);
    fn sweep_device_stop_scanning(device: *const SweepDevice, error: *const *mut SweepError);
    fn sweep_device_get_motor_ready(device: *const SweepDevice, error: *const *mut SweepError) -> bool;
    /// Blocks until device is ready to adjust motor speed, then adjusts motor speed
    fn sweep_device_set_motor_speed(device: *const SweepDevice, hz: int32_t, error: *const *mut SweepError);
    fn sweep_device_get_motor_speed(device: *const SweepDevice, error: *const *mut SweepError) -> int32_t;
    fn sweep_device_get_sample_rate(device: *const SweepDevice, error: *const *mut SweepError) -> int32_t;

    fn sweep_device_get_scan(device: *const SweepDevice, error: *const *mut SweepError) -> *const SweepScan;
    fn sweep_scan_get_number_of_samples(scan: *const SweepScan) -> int32_t;
    fn sweep_scan_get_angle(scan: *const SweepScan, sample: int32_t) -> int32_t;
    fn sweep_scan_get_distance(scan: *const SweepScan, sample: int32_t) -> int32_t;
    fn sweep_scan_get_signal_strength(scan: *const SweepScan, sample: int32_t) -> int32_t;
}

fn get_error(err: *mut SweepError) -> String {
    format!("{:?}", unsafe { CStr::from_ptr(sweep_error_message(err)) })
}

pub struct Sweep {
    device: *const SweepDevice
}

pub struct Sample {
    pub angle: i32,
    pub distance: i32,
    pub signal_strength: i32
}

impl Sweep {
    
    pub fn new(device: String) -> Result<Self, String> {
        unsafe {
            let err : *mut SweepError = std::ptr::null_mut();
            let device = sweep_device_construct_simple(CString::new(device).unwrap().as_ptr(), &err);
            if err.is_null() {
                Ok(Sweep { device: device })
            } else {
                Err(get_error(err))
            }
        }
    }

    pub fn get_version() -> int32_t {
        unsafe {
            sweep_get_version()
        }
    }

    pub fn is_abi_compatible() -> bool {
        unsafe {
            sweep_is_abi_compatible()
        }
    }

    pub fn set_motor_speed(&self, speed_hertz: int32_t) -> Result<(), String> {
        unsafe {
            let err : *mut SweepError = std::ptr::null_mut();
            sweep_device_set_motor_speed(self.device, speed_hertz, &err);
            if err.is_null() {
                Ok(())
            } else {
                Err(get_error(err))
            }
        }
    }

    pub fn get_motor_speed(&self) -> Result<int32_t, String> {
        unsafe {
            let err : *mut SweepError = std::ptr::null_mut();
            let speed = sweep_device_get_motor_speed(self.device, &err);
            if err.is_null() {
                Ok(speed)
            } else {
                Err(get_error(err))
            }
        }
    }

    pub fn get_sample_rate(&self) -> Result<int32_t, String> {
        unsafe {
            let err : *mut SweepError = std::ptr::null_mut();
            let sample_rate = sweep_device_get_sample_rate(self.device, &err);
            if err.is_null() {
                Ok(sample_rate)
            } else {
                Err(get_error(err))
            }
        }
    }

    pub fn get_motor_ready(&self) -> Result<bool, String> {
        unsafe {
            let err : *mut SweepError = std::ptr::null_mut();
            let motor_ready = sweep_device_get_motor_ready(self.device, &err);
            if err.is_null() {
                Ok(motor_ready)
            } else {
                Err(get_error(err))
            }
        }
    }

    pub fn start_scanning(&self) -> Result<(), String> {
        unsafe {
            let err : *mut SweepError = std::ptr::null_mut();
            sweep_device_start_scanning(self.device, &err);
            if err.is_null() {
                Ok(())
            } else {
                Err(get_error(err))
            }
        }
    }

    pub fn stop_scanning(&self) -> Result<(), String> {
        unsafe {
            let err : *mut SweepError = std::ptr::null_mut();
            sweep_device_stop_scanning(self.device, &err);
            if err.is_null() {
                Ok(())
            } else {
                Err(get_error(err))
            }
        }
    }

    pub fn scan(&self) -> Result<Vec<Sample>, String> {
        unsafe {
            let err : *mut SweepError = std::ptr::null_mut();
            let scan = sweep_device_get_scan(self.device, &err);
            if err.is_null() {
                let sample_count = sweep_scan_get_number_of_samples(scan);

                let mut points: Vec<Sample> = vec![];

                for n in 0..sample_count {
                    let angle = sweep_scan_get_angle(scan, n);
                    let distance = sweep_scan_get_distance(scan, n);
                    let signal_strength = sweep_scan_get_signal_strength(scan, n);
                    points.push(Sample { angle: angle, distance: distance, signal_strength: signal_strength });
                }

                Ok(points)
            } else {
                Err(get_error(err))
            }
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_api() {
        
        // the serial port varies depending on your operating system and environment
        let port = String::from("/dev/tty.usbserial-DM00KC6Z");
//        let port = String::from("/dev/ttyUSB0");
        
        let version = Sweep::get_version();
        println!("Version {}.{}", version >> 16, version & 0x0F);
        println!("ABI compatible: {}", Sweep::is_abi_compatible());
        let sweep = Sweep::new(port).unwrap();
        println!("Motor speed: {}", sweep.get_motor_speed().unwrap());
        println!("Sample rate: {}", sweep.get_sample_rate().unwrap());
        println!("Starting scan ...");
        sweep.start_scanning().unwrap();
        let points = sweep.scan().unwrap();
        for Sample in &points {
            println!("Angle {}, Distance {}, Signal Strength: {}",
                     Sample.angle, Sample.distance, Sample.signal_strength);
        }
        sweep.stop_scanning().unwrap();
    }

}
