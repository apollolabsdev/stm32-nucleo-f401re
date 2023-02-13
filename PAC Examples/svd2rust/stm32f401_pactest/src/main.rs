#![no_std]
#![no_main]

// Imports
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f401_pac::Peripherals;

#[entry]
fn main() -> ! {
    let per = Peripherals::take().unwrap();
    loop {}
}
