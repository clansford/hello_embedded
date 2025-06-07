#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use hal::gpio::PinState;
use hal::{pac, prelude::*};
use panic_halt as _;
use stm32f4xx_hal as hal;

#[entry]
fn main() -> ! {
  if let (Some(dp), Some(cp)) =
    (pac::Peripherals::take(), cortex_m::peripheral::Peripherals::take())
  {
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output_in_state(PinState::Low);
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();
    let mut delay = cp.SYST.delay(&clocks);

    loop {
      led.toggle();
      delay.delay_ms(5000);
    }
  }

  #[allow(clippy::empty_loop)]
  loop {}
}
