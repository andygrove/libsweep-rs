extern crate libsweep;
use libsweep::*;


fn main() {
    let sweep = Sweep::new(String::from("/dev/ttyUSB0")).unwrap();
    sweep.start_scanning().unwrap();
}

