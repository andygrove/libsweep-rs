extern crate libc;

use libc::{int32_t};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
struct sweep_device;

#[repr(C)]
struct sweep_error;

#[repr(C)]
struct sweep_scan;

#[link(name = "sweep")]
extern {
  fn sweep_error_message(error: *const sweep_error) -> *const c_char;
  fn sweep_is_abi_compatible() -> bool;
  fn sweep_get_version() -> int32_t;
  fn sweep_device_construct_simple(port: *const c_char, error: *const *mut sweep_error) -> *const sweep_device;
  fn sweep_device_start_scanning(device: *const sweep_device, error: *const *mut sweep_error);
  fn sweep_device_stop_scanning(device: *const sweep_device, error: *const *mut sweep_error);
  fn sweep_device_get_motor_speed(device: *const sweep_device, error: *const *mut sweep_error) -> int32_t;
  fn sweep_device_get_sample_rate(device: *const sweep_device, error: *const *mut sweep_error) -> int32_t;
  fn sweep_device_get_scan(device: *const sweep_device, error: *const *mut sweep_error) -> *const sweep_scan;
  fn sweep_scan_get_number_of_samples(scan: *const sweep_scan) -> int32_t;
  fn sweep_scan_get_angle(scan: *const sweep_scan, sample: int32_t) -> int32_t;
  fn sweep_scan_get_distance(scan: *const sweep_scan, sample: int32_t) -> int32_t;
  fn sweep_scan_get_signal_strength(scan: *const sweep_scan, sample: int32_t) -> int32_t;
}


fn check(error: *mut sweep_error) -> Result<(), String> {
  if error.is_null() {
    //println!("No error");
    Ok(())
  } else {
    Err(unsafe {
      format!("{:?}", CStr::from_ptr(sweep_error_message(error)))
    })
  }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {

        unsafe {

        let mut err : *mut sweep_error = std::ptr::null_mut();

        let c = sweep_is_abi_compatible();
        println!("sweep_is_abi_compatible returned {:?}", c);

        let v = sweep_get_version();
        println!("sweep_get_version returned {:?}", v);

        println!("constructing device");
        let device = sweep_device_construct_simple(CString::new("/dev/ttyUSB0").unwrap().as_ptr(), &err);
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
