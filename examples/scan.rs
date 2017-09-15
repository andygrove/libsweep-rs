use std::env;

extern crate libsweep;
use libsweep::*;


fn main() {

    match env::args().nth(1) {
      Some(port) => scan(port),
      None => println!("Serial port must be specified as command-line argument e.g. /dev/ttyUSB0")
    }

}

fn scan(port: String) {

    let version = Sweep::get_version();
    println!("Version {}.{}", version >> 16, version & 0x0F);
    println!("ABI compatible: {}", Sweep::is_abi_compatible());

    let sweep = Sweep::new(port).unwrap();
    println!("Motor speed: {}", sweep.get_motor_speed().unwrap());
    println!("Sample rate: {}", sweep.get_sample_rate().unwrap());
    println!("Starting scan ...");
    sweep.start_scanning().unwrap();
    let scan = sweep.scan().unwrap();
    for sample in &scan {
        println!("Angle {}, Distance {}, Signal Strength: {}",
                 sample.angle, sample.distance, sample.signal_strength);
    }
    sweep.stop_scanning().unwrap();
}

