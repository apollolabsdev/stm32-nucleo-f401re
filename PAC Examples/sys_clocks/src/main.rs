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

    // Enable HSE Clock
    dp.RCC.cr.write(|w| w.hseon().set_bit());

    // Wait for HSE clock to become ready
    while dp.RCC.cr.read().hserdy().bit() {}

    // Configure PCLK1 Prescalar
    dp.RCC.cfgr.write(|w| unsafe { w.ppre1().bits(0b100) });

    // Configure PLL M
    dp.RCC.pllcfgr.write(|w| {
        w.pllm5()
            .bit(false)
            .pllm4()
            .bit(false)
            .pllm3()
            .bit(true)
            .pllm2()
            .bit(false)
            .pllm1()
            .bit(false)
            .pllm0()
            .bit(false)
    });

    // Configure PLL N
    dp.RCC.pllcfgr.write(|w| {
        w.plln8()
            .set_bit()
            .plln7()
            .clear_bit()
            .plln6()
            .set_bit()
            .plln5()
            .clear_bit()
            .plln4()
            .set_bit()
            .plln3()
            .clear_bit()
            .plln2()
            .clear_bit()
            .plln1()
            .clear_bit()
            .plln0()
            .clear_bit()
    });

    // Configure PLL P
    dp.RCC
        .pllcfgr
        .write(|w| w.pllp0().bit(true).pllp1().bit(false));

    // Enable PLL
    dp.RCC.cr.write(|w| w.pllon().set_bit());

    // Wait for PLL to become ready
    while dp.RCC.cr.read().pllrdy().bit() {}

    // Select PLL as System Clock Source
    dp.RCC.cfgr.write(|w| w.sw1().set_bit().sw0().clear_bit());

    // Wait for PLL to be selected as System Clock Source
    while dp.RCC.cfgr.read().sws1().bit_is_set() && dp.RCC.cfgr.read().sws0().bit_is_clear() {}

    //Enable Clock to GPIOA
    dp.RCC.ahb1enr.write(|w| w.gpioaen().set_bit());

    //Configure PA5 as Output
    dp.GPIOA.moder.write(|w| unsafe { w.moder5().bits(0b01) });

    // Set PA5 Output to High signalling end of configuration
    dp.GPIOA.odr.write(|w| w.odr5().set_bit());

    loop {}
}
