# cupi_shift

A Rust crate for manipulating shift registers via the GPIO pins on a Raspberry Pi.

[![Build Status](https://travis-ci.org/inre/cupi.svg?branch=master)](https://travis-ci.org/inre/cupi_shift)

## Overview

This crate provides an interface, `Shifter` that makes it trivially easy to
manipulate [shift registers][4] with a Raspberry Pi (thanks to [CuPi][1]).
Internally it keeps track of each shift register's state, allowing you to
manipulate each pin individually as if it were a regular GPIO pin!

Why would you want to do this?  **The Raspberry Pi only has 17 usable GPIO
pins**.  Pin expanders like the [MCP23017][2] can add up to 16 more per chip
(at a cost of about ~$2-3/each) but they work over I2C which is *slow* (on
the Raspberry Pi anyway).  With shift registers like the [74HC595][3]
(~$0.05-0.10/each) you can add a *nearly infinite* amount of output pins and
*refresh them as fast as the hardware supports*.  You can even use many
sets of 3 pins to run multiple chains of shift registers in parallel.

Realize your dream of controlling an enormous holiday lights display with a
single Raspberry Pi using cupi_shift!

# Example

```rust
extern crate cupi_shift;
use cupi_shift::Shifter;

fn main() {
    // First define which pins you're using for your shift register(s)
    let (data_pin, latch_pin, clock_pin) = (29, 28, 27);

    // Now create a new Shifter instance using those pins
    let mut shifter = Shifter::new(data_pin, latch_pin, clock_pin);

    // Next we need to call `add()` for each shift register and tell it how
    // many pins they have
    let pins = 8;
    let sr0 = shifter.add(pins); // Starts tracking a new shift register

    // Now we can set the state (aka data) of our shift register
    shifter.set(sr0, 0b11111111, true); // Set all pins HIGH
}
```

# Note about pin numbering

[CuPi][1] currently uses GPIO pin numbering.  So pin 40 (very last pin on
the Raspberry Pi 2) is actually pin 29.  You can refer to this image to
figure out which pin is which:

http://pi4j.com/images/j8header-2b-large.png

# Controlling individual pins

That's all well and good (setting the state of all pins at once) but what if
you want to control just one pin at a time?  You can do that too:

```rust
// Set the 8th pin (aka pin 7) HIGH and apply this change immediately
shifter.set_pin_high(sr0, 7, true); // NOTE: 3rd arg is 'apply'
// Set the first pin (aka pin 0) LOW but don't apply just yet
shifter.set_pin_low(sr0, 0, false);
shifter.apply(); // Apply the change (the other way to apply changes)
```

# Controlling multiple shift registers

Every time you call `Shifter.add()` it will start tracking/controlling an
additional shift register.  So if you have two shift registers chained
together you can add and control them individually like so:

```rust
let last = shifter.add(8); // Add an 8-pin shift register (sr_index: 0)
let first = shifter.add(8); // Add another (sr_index: 1)
// Set pin 0 HIGH on shift register 0 (all others LOW) but don't apply the change yet
shifter.set(last, 0b00000001, false);
// Set pin 7 HIGH on shift register 1 (all others LOW) and apply the change
shifter.set(first, 0b10000000, true);
```

**Note:** Shift registers need to be added in the order in which they are
chained with the *last* shift register being added first.  Why is the order
reversed like this?  That's how the logic of shift registers works:  Every
time data is "shifted out" to a shift register it dumps its memory to the
the next shift register in the chain.

You can also apply changes to individual pins on individual shift registers:

```rust
shifter.set_pin_high(sr1, 2, false); // Set pin 2 HIGH on shift register 1
shifter.set_pin_low(sr0, 3, true); // Set pin 3 LOW on shift register 0 (and apply)
```

In the above example we didn't set the *apply* (3rd) argument to `true`
until the we were done making our changes.  If we set *apply* to `true` on
each we could wind up with some flickering.  The more shift registers you
have in your chain the more flickering you can get if you call `apply()`
with every state (aka data) change.


[1]: https://crates.io/crates/cupi
[2]: https://www.adafruit.com/product/732
[3]: https://www.sparkfun.com/datasheets/IC/SN74HC595.pdf
[4]: https://en.wikipedia.org/wiki/Shift_register

# Raspberry Pi pinout reference

[image](http://pi4j.com/images/j8header-2b-large.png)

# API documentation

[documentation](https://liftoff.github.io/cupi_shift/cupi_shift/)

# Raspberry Pi cross-compilation instructions

[instructions](https://github.com/Ogeon/rust-on-raspberry-pi)
