#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
// use cortex_m_semihosting::hprintln;
use hal::gpio::PinState;
use hal::{pac, prelude::*};
use panic_halt as _;
use stm32f4xx_hal as hal;
use switch_hal::{InputSwitch, IntoSwitch};

#[entry]
fn main() -> ! {
  let peripherals = pac::Peripherals::take();
  if let Some(peripherals) = peripherals {
    let gpioa = peripherals.GPIOA.split();
    let gpioc = peripherals.GPIOC.split();

    let mut led = gpioa.pa5.into_push_pull_output_in_state(PinState::Low);
    let button = gpioc.pc13.into_active_low_switch();

    loop {
      let is_pressed = button.is_active().unwrap();
      if is_pressed {
        led.set_high();
      } else {
        led.set_low();
      }
    }
  }

  #[allow(clippy::empty_loop)]
  loop {}
}
