#![no_std]
#![no_main]

// Imports
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f401_pac as pac;

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();

    //Enable Clock to GPIOA & GPIOC
    dp.RCC
        .ahb1enr
        .write(|w| w.gpioaen().set_bit().gpiocen().set_bit());

    //Configure PA5 as Output
    dp.GPIOA.moder.write(|w| unsafe { w.moder5().bits(0b01) });

    // Application Loop
    loop {
        // Read PC13 Input Value
        if !dp.GPIOC.idr.read().idr13().bit() {
            // If low then set PA5 output to High (Turn on LED)
            dp.GPIOA.odr.write(|w| w.odr5().set_bit());
        } else {
            // If high then set PA5 output to Low (Turn off LED)
            dp.GPIOA.odr.write(|w| w.odr5().clear_bit());
        }
    }
}
