//! This crate provides an interface, `Shifter` that makes it trivially easy to
//! manipulate [shift registers][4] with a Raspberry Pi (thanks to [CuPi][1]).
//! Internally it keeps track of each shift register's state, allowing you to
//! manipulate each pin individually as if it were a regular GPIO pin!
//!
//! Why would you want to do this?  **The Raspberry Pi only has 17 usable GPIO
//! pins**.  Pin expanders like the [MCP23017][2] can add up to 16 more per chip
//! (at a cost of about ~$2-3/each) but they work over I2C which is *slow* (on
//! the Raspberry Pi anyway).  With shift registers like the [74HC595][3]
//! (~$0.05-0.10/each) you can add a *nearly infinite* amount of output pins and
//! *refresh them as fast as the hardware supports*.  You can even use many
//! sets of 3 pins to run multiple chains of shift registers in parallel.
//!
//! Realize your dream of controlling an enormous holiday lights display with a
//! single Raspberry Pi using cupi_shift!
//!
//! # Example
//!
//! ```
//! extern crate cupi_shift;
//! use cupi_shift::Shifter;
//!
//! fn main() {
//!     // First define which pins you're using for your shift register(s)
//!     let (data_pin, latch_pin, clock_pin) = (29, 28, 27);
//!
//!     // Now create a new Shifter instance using those pins
//!     let mut shifter = Shifter::new(data_pin, latch_pin, clock_pin);
//!
//!     // Next we need to call `add()` for each shift register and tell it how
//!     // many pins they have
//!     let pins = 8;
//!     let sr0 = shifter.add(pins); // Starts tracking a new shift register
//!
//!     // Now we can set the state (aka data) of our shift register
//!     shifter.set(sr0, 0b11111111, true); // Set all pins HIGH
//! }
//!
//! ```
//! # Note about pin numbering
//!
//! [CuPi][1] currently uses GPIO pin numbering.  So pin 40 (very last pin on
//! the Raspberry Pi 2) is actually pin 29.  You can refer to this image to
//! figure out which pin is which:
//!
//! http://pi4j.com/images/j8header-2b-large.png
//!
//! # Controlling individual pins
//!
//! That's all well and good (setting the state of all pins at once) but what if
//! you want to control just one pin at a time?  You can do that too:
//!
//! ```
//! // Set the 8th pin (aka pin 7) HIGH and apply this change immediately
//! shifter.set_pin_high(sr0, 7, true); // NOTE: 3rd arg is 'apply'
//! // Set the first pin (aka pin 0) LOW but don't apply just yet
//! shifter.set_pin_low(sr0, 0, false);
//! shifter.apply(); // Apply the change (the other way to apply changes)
//! ```
//!
//! # Controlling multiple shift registers
//!
//! Every time you call `Shifter.add()` it will start tracking/controlling an
//! additional shift register.  So if you have two shift registers chained
//! together you can add and control them individually like so:
//!
//! ```
//! let last = shifter.add(8); // Add an 8-pin shift register (sr_index: 0)
//! let first = shifter.add(8); // Add another (sr_index: 1)
//! // Set pin 0 HIGH on shift register 0 (all others LOW) but don't apply the change yet
//! shifter.set(last, 0b00000001, false);
//! // Set pin 7 HIGH on shift register 1 (all others LOW) and apply the change
//! shifter.set(first, 0b10000000, true);
//! ```
//!
//! **Note:** Shift registers need to be added in the order in which they are
//! chained with the *last* shift register being added first.  Why is the order
//! reversed like this?  That's how the logic of shift registers works:  Every
//! time data is "shifted out" to a shift register it dumps its memory to the
//! the next shift register in the chain.
//!
//! You can also apply changes to individual pins on individual shift registers:
//!
//! ```
//! shifter.set_pin_high(sr1, 2, false); // Set pin 2 HIGH on shift register 1
//! shifter.set_pin_low(sr0, 3, true); // Set pin 3 LOW on shift register 0 (and apply)
//! ```
//!
//! In the above example we didn't set the *apply* (3rd) argument to `true`
//! until the we were done making our changes.  If we set *apply* to `true` on
//! each we could wind up with some flickering.  The more shift registers you
//! have in your chain the more flickering you can get if you call `apply()`
//! with every state (aka data) change.
//!
//!
//! [1]: https://crates.io/crates/cupi
//! [2]: https://www.adafruit.com/product/732
//! [3]: https://www.sparkfun.com/datasheets/IC/SN74HC595.pdf
//! [4]: https://en.wikipedia.org/wiki/Shift_register

#![allow(dead_code, unused_variables)]

extern crate cupi;

// Using a singly-linked list to represent the chain of shift registers since
// it accurately represents how they're physically linked together.
use std::collections::LinkedList;
use std::cell::RefCell;
use cupi::{CuPi, PinOutput, DigitalWrite};


struct ShiftRegister {
    data: usize, // e.g. 0b01010101
    pins: u8, // Not aware of any shift registers that have more than 255 output pins
}

// This is great for debugging; displays the Shift Register data in binary:
impl std::fmt::Display for ShiftRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = format!("{:b}", self.data);
        let pad = (self.pins as usize) - string.len();
        let _ = f.write_str("0b");
        for _ in 0..pad { let _ = f.write_str("0").unwrap(); }
        f.pad_integral(true, "", &string)
    }
}

impl ShiftRegister {

    fn set(&mut self, data: usize) {
        self.data = data;
    }

    fn get_ref(self) -> RefCell<ShiftRegister> {
        RefCell::new(self)
    }
}

pub struct Shifter {
    pub data: PinOutput,
    pub latch: PinOutput,
    pub clock: PinOutput,
    shift_registers: LinkedList<ShiftRegister>,
    invert: bool,
}

impl Shifter {

    /// Returns a new `Shifter` object that will shift out data using the given
    /// *data_pin*, *latch_pin*, and *clock_pin*.  To use a `Shifter` instance
    /// you must first call the `add()` method for each shift register you
    /// have connected in sequence.
    ///
    /// # Note about pin numbering
    ///
    /// `cupi` currently uses GPIO pin numbering.  So pin 40 (very last pin on
    /// the Raspberry Pi 2) is actually pin 29.  You can refer to this image to
    /// figure out which pin is which:
    ///
    /// http://pi4j.com/images/j8header-2b-large.png
    pub fn new(data_pin: usize, latch_pin: usize, clock_pin: usize) -> Shifter {
        let cupi = CuPi::new().unwrap();
        let shift_registers: LinkedList<ShiftRegister> = LinkedList::new();
        Shifter {
            data: cupi.pin(data_pin).unwrap().output(),
            latch: cupi.pin(latch_pin).unwrap().output(),
            clock: cupi.pin(clock_pin).unwrap().output(),
            shift_registers: shift_registers,
            invert: false,
        }
    }

    /// Adds a new shift register to this Shifter and returns a reference to it.
    /// You must specify the number of pins.
    pub fn add(&mut self, pins: u8) -> usize {
        let sr = ShiftRegister { data: 0, pins: pins };
        self.shift_registers.push_back(sr);
        self.shift_registers.len() - 1
    }

    /// Sets the *data* on the shift register at the given *sr_index*.
    /// If *apply* is `true` the change will be applied immediately.
    pub fn set(&mut self, sr_index: usize, data: usize, apply: bool) {
        for (i, sr) in self.shift_registers.iter_mut().enumerate() {
            if i == sr_index {
                sr.set(data);
                break;
            }
        }
        if apply { self.apply(); }
    }

    /// Sets the given *pin* HIGH on the shift register at the given *sr_index*.
    /// If *apply* is `true` the change will be applied immediately.
    pub fn set_pin_high(&mut self, sr_index: usize, pin: u8, apply: bool) {
        for (i, sr) in self.shift_registers.iter_mut().enumerate() {
            if i == sr_index {
                let new_state = sr.data | 1 << pin;
                sr.set(new_state);
                break;
            }
        }
        if apply { self.apply(); }
    }

    /// Sets the given *pin* LOW on the shift register at the given *sr_index*.
    /// If *apply* is `true` the change will be applied immediately.
    pub fn set_pin_low(&mut self, sr_index: usize, pin: u8, apply: bool) {
        for (i, sr) in self.shift_registers.iter_mut().enumerate() {
            if i == sr_index {
                let new_state = sr.data & !(1 << pin);
                sr.set(new_state);
                break;
            }
        }
        if apply { self.apply(); }
    }

    /// This function will invert all logic so that HIGH is LOW and LOW is HIGH.
    /// Very convenient if you made a (very common) mistake in your wiring or
    /// you need reversed logic for other reasons.
    pub fn invert(&mut self) {
        match self.invert {
            true => self.invert = false,
            false => self.invert = true,
        }
    }

    /// Applies all current shift register states by shifting out all the stored
    /// data in each ShiftRegister object.
    pub fn apply(&mut self) {
        self.latch.low().unwrap();
        for sr in self.shift_registers.iter() {
            for n in 0..sr.pins {
                self.clock.low().unwrap();
                if self.invert {
                    match sr.data >> n & 1 {
                        1 => self.data.low().unwrap(),
                        0 => self.data.high().unwrap(),
                        _ => unreachable!(),
                    }
                } else {
                    match sr.data >> n & 1 {
                        0 => self.data.low().unwrap(),
                        1 => self.data.high().unwrap(),
                        _ => unreachable!(),
                    }
                }
                self.clock.high().unwrap();
            }
        }
        self.latch.high().unwrap();
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
