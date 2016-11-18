#![feature(alloc_system)]

extern crate alloc_system;
extern crate cupi;
extern crate cupi_shift;

use cupi_shift::Shifter;
use cupi::{delay_ms};

/// This example toggles ("blinks") all output pins on a single shift register.
/// To see an example that uses multiple shift registers see multiblink.rs
///
/// Make sure the pin numbering matches what you're using on your Raspberry Pi
/// and the number of shift registers you have chained together.
fn main() {

    // I like to use the last three pins on the bottom right of the RPi:
    let (data_pin, latch_pin, clock_pin) = (29, 28, 27); // MAKE SURE THESE MATCH YOUR SETUP!
    // Create our Shifter instance with the specified pins
    let mut shifter = Shifter::new(data_pin, latch_pin, clock_pin);
    // We can't do anything with it until we add at least one shift register:
    let pins = 8; // Number of output pins on our shift register
    let sr0 = shifter.add(pins); // Adds/tracks a new shift register
    // If you find that these states are inverted/backwards you can swap them with the `invert()` method...
    // shifter.invert(); // Uncomment to invert
    let loops = 2; // How many times to loop
    println!("Looping {} times...", loops);
    for i in 0..loops {
        println!("Loop {}: All ON", i+1);
        shifter.set(sr0, 0b11111111, true); // All on
        delay_ms(1000);
        println!("Loop {}: All OFF", i+1);
        shifter.set(sr0, 0b00000000, true); // All off
        delay_ms(1000);
    }

}
