#![no_std]
#![no_main]

// Imports
use cortex_m;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f401_pac as pac;

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();

    // Initlialize Clocks
    // Enable HSE Clock
    dp.RCC.cr.write(|w| w.hseon().set_bit());

    // Wait for HSE clock to become ready
    while dp.RCC.cr.read().hserdy().bit() {}

    //Enable Clock to GPIOA
    dp.RCC.ahb1enr.write(|w| w.gpioaen().set_bit());

    //Configure PA5 as Output
    dp.GPIOA.moder.write(|w| unsafe { w.moder5().bits(0b01) });

    // Set PA5 Output to High signalling end of configuration
    dp.GPIOA.odr.write(|w| w.odr5().set_bit());

    // Set the Reload Value Reflecting 1 second for a 8 MHz Clock
    unsafe { cp.SYST.rvr.write(8000000) };

    // Reset the Current Value to match the Reload Value
    unsafe { cp.SYST.cvr.write(8000000) };

    // Configure the CSR register and Enable Timer
    unsafe { cp.SYST.csr.write(0x0105) };

    loop {
        // Check if Timer Expired
        if cp.SYST.has_wrapped() {
            // Toggle Output Pin
            dp.GPIOA.odr.modify(|r, w| w.odr5().bit(!r.odr5().bit()));
        }
    }
}
