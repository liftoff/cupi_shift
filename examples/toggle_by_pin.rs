#![feature(alloc_system)]

extern crate alloc_system;
extern crate cupi;
extern crate cupi_shift;

use cupi_shift::Shifter;
use cupi::{delay_ms};

/// Ever wish you could address each of your shift register pins like they were
/// regular GPIO pins?  You can do that with cupi_shift!
///
/// This example sequentially toggles individual pins on your shift register(s)
/// using the per-pin API.
///
/// Make sure the pin numbering matches what you're using on your Raspberry Pi
/// and the number of shift registers you have chained together.
fn main() {

    // I like to use the last three pins on the bottom right of the RPi:
    let (data_pin, latch_pin, clock_pin) = (29, 28, 27); // MAKE SURE THESE MATCH YOUR SETUP!
    let mut shifter = Shifter::new(data_pin, latch_pin, clock_pin);
    let pins = 8; // Number of output pins on our shift registers
    let sr0 = shifter.add(pins); // Adds/tracks a new shift register
    // If you find that these states are inverted/backwards you can swap them with the `invert()` method...
    // shifter.invert(); // Uncomment to invert
    for _ in 0..2 {
        for i in 0..pins {
            println!("Setting pin {} HIGH", i);
            shifter.set_pin_high(sr0, i, true);
            delay_ms(100);
            println!("Setting pin {} LOW", i);
            shifter.set_pin_low(sr0, i, true);
            delay_ms(100);
        }
    }
}
