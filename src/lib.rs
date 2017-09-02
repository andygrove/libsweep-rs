extern crate libc;

use libc::{int32_t};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
struct sweep_device {}

#[repr(C)]
struct sweep_error {}

#[link(name = "sweep")]
extern {
  fn sweep_error_message(error: *const sweep_error) -> *const c_char;
  fn sweep_is_abi_compatible() -> bool;
  fn sweep_get_version() -> int32_t;
  fn sweep_device_construct_simple(port: *const c_char, error: *const *mut sweep_error) -> *const sweep_device;
  fn sweep_device_start_scanning(device: *const sweep_device, error: *const *mut sweep_error);
}


fn check(error: *mut sweep_error) {
  if (error.is_null()) {
    println!("No error");
  } else {
    unsafe {
      let error_string = sweep_error_message(error);
      println!("Sweep error message: {:?}", CStr::from_ptr(error_string));
    }
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
        let device = sweep_device_construct_simple(CString::new("TBD").unwrap().as_ptr(), &err);
        check(err);

        println!("start scanning");
        sweep_device_start_scanning(device, &err);
        check(err);


        }
    }
}
