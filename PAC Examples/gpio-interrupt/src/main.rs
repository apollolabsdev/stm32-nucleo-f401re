#![no_std]
#![no_main]

// Imports
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use pac::{interrupt, Peripherals};
use panic_halt as _;
use stm32f401_pac as pac;

// Create a Global Variable for the Peripherals
static G_PER: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();

    // GPIO Configuration

    // 1. Enable Clock to GPIOA & GPIOC
    dp.RCC
        .ahb1enr
        .write(|w| w.gpioaen().set_bit().gpiocen().set_bit());

    // 2. Configure PA5 as Output
    dp.GPIOA.moder.write(|w| unsafe { w.moder5().bits(0b01) });

    // Interrupt Configuration

    // 1. Enable the SYSCFG bit in the RCC register
    dp.RCC.apb2enr.write(|w| w.syscfgen().set_bit());

    // 2. Configure SYSCFG EXTICR4 Register
    dp.SYSCFG.exticr4.write(|w| unsafe { w.exti13().bits(2) });

    // 3. Disable the EXTI Mask using Interrupt Mask Register (IMR)
    dp.EXTI.imr.write(|w| w.mr13().set_bit());

    // 4. Configure the Rising Edge Trigger in the EXTI RTSR Register
    dp.EXTI.rtsr.write(|w| w.tr13().set_bit());

    // 5. Enable EXT13 at NVIC Level
    unsafe { cortex_m::peripheral::NVIC::unmask(interrupt::EXTI15_10) }

    // Since Initialization is complete, move Peripherals struct to Global Context
    cortex_m::interrupt::free(|cs| {
        G_PER.borrow(cs).replace(Some(dp));
    });

    // Application Loop
    loop {}
}

// Handler for pins connected to line 10 to 15
#[interrupt]
fn EXTI15_10() {
    // Start a Critical Section
    cortex_m::interrupt::free(|cs| {
        // Obtain Access to Peripherals Global Data
        let mut dp = G_PER.borrow(cs).borrow_mut();
        // Check if PA13 caused the interrupt
        if dp.as_mut().unwrap().EXTI.pr.read().pr13().bit() {
            // Clear Interrupt Flag for Button
            dp.as_mut().unwrap().EXTI.pr.write(|w| w.pr13().set_bit());
            // Toggle Output LED
            dp.as_mut()
                .unwrap()
                .GPIOA
                .odr
                .modify(|r, w| w.odr5().bit(!r.odr5().bit()));
        }
    });
}
