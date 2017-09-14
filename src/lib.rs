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

fn check(error: *mut SweepError) -> Result<(), String> {
    if error.is_null() {
        //println!("No error");
        Ok(())
    } else {
        Err(unsafe {
            format!("{:?}", CStr::from_ptr(sweep_error_message(error)))
        })
    }
}

pub struct Sweep {
    device: *const SweepDevice
}

impl Sweep {
    
    pub fn new(device: String) -> Result<Self, String> {
        unsafe {
            let mut err : *mut SweepError = std::ptr::null_mut();
            let device = sweep_device_construct_simple(CString::new(device).unwrap().as_ptr(), &err);
            if err.is_null() {
                Ok(Sweep { device: device })
            } else {
                Err(get_error(err))
            }
        }
    }

    pub fn start_scanning(&self) -> Result<(), String> {
        unsafe {
            let mut err : *mut SweepError = std::ptr::null_mut();
            sweep_device_start_scanning(self.device, &err);
            if err.is_null() {
                Ok(())
            } else {
                Err(get_error(err))
            }
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;
/*
    #[test]
    fn rust_calls_work() {
      let mut sweep = Sweep::new(String::from("/dev/ttyUSB0")).unwrap();
      let data = sweep.scan();
      for i in 0..data.len() {
        println!("Angle {}, Distance {}, Signal Strength: {}", data[i].angle, data[i].distance, data[i].signal_strength);
      }
    }
*/
    #[test]
    fn ffi_calls_work() {

        unsafe {

        let mut err : *mut SweepError = std::ptr::null_mut();

        let c = sweep_is_abi_compatible();
        println!("sweep_is_abi_compatible returned {:?}", c);

        let v = sweep_get_version();
        println!("sweep_get_version returned {:?}", v);

        println!("constructing device");


        check(err).unwrap();

        println!("Motor speed: {}", sweep_device_get_motor_speed(device, &err));
        check(err).unwrap();

        println!("Sample rate: {}", sweep_device_get_sample_rate(device, &err));
        check(err).unwrap();

        println!("start scanning");
        sweep_device_start_scanning(device, &err);
        check(err).unwrap();

        let scan = sweep_device_get_scan(device, &err);
        check(err).unwrap();

        let sample_count = sweep_scan_get_number_of_samples(scan);

        for n in 0..sample_count {
          let angle = sweep_scan_get_angle(scan, n);
          let distance = sweep_scan_get_distance(scan, n);
          let signal = sweep_scan_get_signal_strength(scan, n);
          println!("Angle {}, Distance {}, Signal Strength: {}", angle, distance, signal);
        }

        println!("stop scanning");
        sweep_device_stop_scanning(device, &err);
        check(err).unwrap();

        }
    }
}
