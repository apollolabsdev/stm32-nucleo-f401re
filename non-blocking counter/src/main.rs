#![no_std]
#![no_main]

// Imports
use cortex_m_rt::entry;
use fugit::{Duration, ExtU32};
use panic_halt as _;
use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
};

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    // Configure the LED pin as a push pull ouput and obtain handler.
    // On the Nucleo FR401 theres an on-board LED connected to pin PA5.
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    // Configure the button pin (if needed) and obtain handler.
    // On the Nucleo FR401 there is a button connected to pin PC13.
    // Pin is input by default
    let gpioc = dp.GPIOC.split();
    let button = gpioc.pc13;

    // Create and initialize a delay variable to manage delay loop
    let mut del_var: Duration<u32, 1, 1000> = 2001.millis();

    // Initialize LED to on or off
    led.set_low();

    // Create a Millisecond Counter Handle
    let mut counter = dp.TIM1.counter_ms(&clocks);

    // Application Loop
    loop {
        // Start counter with with del_var duration
        counter.start(del_var).unwrap();
        // Enter loop and check for button press until counter reaches del_var
        while counter.now().duration_since_epoch() < del_var - 1.millis() {
            // Check if button is pressed at any point
            if button.is_low() {
                // If button pressed decrease the delay value by 500 ms
                del_var = del_var - 500.millis();
                // If updated delay value drops below 500 ms then reset it back to starting value to 2 secs
                if del_var.to_millis() < 500_u32 {
                    del_var = 2001.millis();
                }
                // Exit delay loop since button was pressed
                break;
            }
        }
        // Toggle LED
        led.toggle();
    }
}
