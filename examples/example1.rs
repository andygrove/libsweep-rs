extern crate libsweep;
use libsweep::*;


fn main() {
    let sweep = Sweep::new(String::from("/dev/ttyUSB0")).unwrap();
    sweep.start_scanning().unwrap();
    let points = sweep.scan().unwrap();
    for point in &points {
        println!("Angle {}, Distance {}, Signal Strength: {}",
                 point.angle, point.distance, point.signal_strength);
    }
    sweep.stop_scanning().unwrap();
}

