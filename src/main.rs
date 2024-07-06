#![no_std]
#![no_main]

use cortex_m::interrupt::free;
use cortex_m_rt::entry;
use hal::delay::Delay;
use hal::prelude::*;
use hal::stm32;
use stm32f0xx_hal as hal;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

#[entry]
fn main() -> ! {
    let mut p = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = p.RCC.configure().sysclk(48.mhz()).freeze(&mut p.FLASH);

    let s = p.GPIOA.split(&mut rcc);

    let mut led = free(|cs| s.pa5.into_push_pull_output(cs));

    let mut delay = Delay::new(cp.SYST, &rcc);
    loop {
        delay.delay_ms(1_000_u16);
        led.toggle().unwrap();
    }
}
