extern crate libsweep;
use libsweep::*;


fn main() {
    // the serial port varies depending on your operating system and environment
    //let port = String::from("/dev/tty.usbserial-DM00KC6Z");
    let port = String::from("/dev/ttyUSB0");

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

