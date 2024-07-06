#![no_std]
#![no_main]

use core::cell::RefCell;
use cortex_m::asm;
use cortex_m::interrupt::free;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use hal::prelude::*;
use hal::stm32;
use panic_halt as _;
use stm32f0xx_hal as hal;
use stm32f0xx_hal::gpio::gpioa::PA5;
use stm32f0xx_hal::gpio::Output;
use stm32f0xx_hal::gpio::PushPull;
use stm32f0xx_hal::pac::interrupt;
use stm32f0xx_hal::pac::TIM3;
use stm32f0xx_hal::timers::Timer;

static TIMER: Mutex<RefCell<Option<Timer<TIM3>>>> = Mutex::new(RefCell::new(None));
static LED: Mutex<RefCell<Option<PA5<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let mut p = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);

    let s = p.GPIOA.split(&mut rcc);

    free(|cs| {
        LED.borrow(cs)
            .replace(Some(s.pa5.into_push_pull_output(cs)))
    });

    let mut timer = hal::timers::Timer::tim3(p.TIM3, 1.hz(), &mut rcc);
    timer.wait().ok();
    //imer.start(1.hz());
    timer.listen(stm32f0xx_hal::timers::Event::TimeOut);

    free(|cs| TIMER.borrow(cs).replace(Some(timer)));

    unsafe {
        cortex_m::peripheral::NVIC::unmask(interrupt::TIM3);
        cortex_m::peripheral::NVIC::unpend(interrupt::TIM3);
    }

    loop {
        asm::wfi();
    }
}

#[interrupt]
fn TIM3() {
    free(|cs| LED.borrow(cs).borrow_mut().as_mut().unwrap().toggle())
        .expect("Failed to toggle LED");
    free(|cs| TIMER.borrow(cs).borrow_mut().as_mut().unwrap().wait())
        .expect("Failed to wait for timer");
}
