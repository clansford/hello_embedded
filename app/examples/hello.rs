//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use panic_halt as _;

#[entry]
fn main() -> ! {
  hprintln!("Howdy Howdy Howdy");

  // exit QEMU
  // NOTE do not run this on hardware; it can corrupt OpenOCD state
  // debug::exit(debug::EXIT_SUCCESS);

  loop {}
}
