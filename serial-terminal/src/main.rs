#![no_std]
#![no_main]

// Imports
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    gpio::Pin,
    pac::{self},
    prelude::*,
};

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();

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
    let mut del_var = 7_0000_i32;

    // Initialize LED to on or off
    led.set_low();

    // Application Loop
    loop {
        // Call delay function and update delay variable once done
        del_var = loop_delay(del_var, &button);

        // Toggle LED
        led.toggle();

        // Call delay function and update delay variable once done
        //del_var = loop_delay(del_var, &button);
    }
}

// Delay Function
fn loop_delay<const P: char, const N: u8>(mut del: i32, but: &Pin<P, N>) -> i32 {
    // Loop for until value of del
    for _i in 1..del {
        // Check if button got pressed
        if but.is_low() {
            // If button pressed decrease the delay value
            del = del - 3_0000_i32;
            // If updated delay value reaches zero then reset it back to starting value
            if del < 1_0000 {
                del = 7_0000_i32;
            }
            // Exit function returning updated delay value if button pressed
            return del;
        }
    }
    // Exit function returning original delay value if button no pressed (for loop ending naturally)
    del
}
