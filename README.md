# libsweep-rs

[![Version](https://img.shields.io/crates/v/libsweep.svg)](https://crates.io/crates/libsweep)
[![Docs](https://docs.rs/libsweep/badge.svg)](https://docs.rs/libsweep)

This is a Rust wrapper for the Sweep SDK for interacting with the Scanse Sweep LIDAR unit.

# Usage
```rust
let port = String::from("/dev/ttyUSB0");
let sweep = Sweep::new(port).unwrap();
sweep.start_scanning().unwrap();
let points = sweep.scan().unwrap();
for point in &points {
    println!("Angle {}, Distance {}, Signal Strength: {}",
        point.angle, point.distance, point.signal_strength);
}
sweep.stop_scanning().unwrap();
```

# Instructions

This crate relies on the Sweep SDK (https://github.com/scanse/sweep-sdk) being installed.
 
Connect the LIDAR unit and find out which port it is using. On Raspberry Pi it is likely `/dev/ttyUSB0` or similar. On Mac it is likely somthing like `/dev/tty.usbserial-DM00KC6Z`.

Run the example using this syntax, passing the port as a command-line parameter.

```bash
cargo run --example scan /dev/ttyUSB0
```

You should see output like:

```
Version 1.2
ABI compatible: true
Motor speed: 5
Sample rate: 500
Starting scan ...
Angle 349375, Distance 67, Signal Strength: 191
Angle 352125, Distance 44, Signal Strength: 191
Angle 354687, Distance 40, Signal Strength: 191
Angle 357437, Distance 40, Signal Strength: 199
```
