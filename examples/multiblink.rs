#![feature(alloc_system)]

extern crate alloc_system;
extern crate cupi;
extern crate cupi_shift;

use cupi_shift::Shifter;
use cupi::{delay_ms};

/// This example toggles ("blinks") all output pins on multiple shift registers.
///
/// Make sure the pin numbering matches what you're using on your Raspberry Pi
/// and the number of shift registers you have chained together.
fn main() {

    // I like to use the last three pins on the bottom right of the RPi:
    let (data_pin, latch_pin, clock_pin) = (29, 28, 27); // MAKE SURE THESE MATCH YOUR SETUP!
    // Create our Shifter instance with the specified pins
    let mut shifter = Shifter::new(data_pin, latch_pin, clock_pin);
    let pins = 8; // Number of output pins on our shift registers
    // Call .add() once for each shift register in the chain...
    let sr0 = shifter.add(pins); // The values returned by .add() are just indexes
    let sr1 = shifter.add(pins);
    // If you find that these states are inverted/backwards you can swap them with the `invert()` method...
    // shifter.invert(); // Uncomment to invert
    let loops = 2; // How many times to loop
    println!("Looping {} times...", loops);
    for i in 0..loops {
        println!("Loop {}: All ON", i+1);
    // Note that the 3rd argument (false) controls whether the change should be applied immediately:
        shifter.set(sr0, 0b11111111, false); // All on sr0
        shifter.set(sr1, 0b11111111, true); // All on sr1
    // Applying the change as a final step is much more efficient and prevents flickering
        delay_ms(1000);
        println!("Loop {}: All OFF", i+1);
        shifter.set(sr0, 0b00000000, false); // All off sr0
        shifter.set(sr1, 0b00000000, false); // All off sr1
        shifter.apply(); // The other way to apply changes
        delay_ms(1000);
    }

}
