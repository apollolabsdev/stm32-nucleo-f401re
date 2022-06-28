#![no_std]
#![no_main]

// Imports
use core::fmt::Write; // allows use to use the WriteLn! macro for easy printing
use cortex_m_rt::entry;
use debouncr::{debounce_3, Edge};
use panic_halt as _;
use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
    serial::{Config, Serial},
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

    // Serial config steps:
    // 1) Need to configure the system clocks
    // - Promote RCC structure to HAL to be able to configure clocks
    let rcc = dp.RCC.constrain();
    // - Configure system clocks
    // 8 MHz must be used for the Nucleo-F401RE board according to manual
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    // 2) Configure/Define TX pin
    // Note that we already split port A earlier for the led pin
    // Use PA2 as it is connected to the host serial interface
    let tx_pin = gpioa.pa2.into_alternate();

    // 3) Configure Serial perihperal channel
    // We're going to use USART2 since its pins are the ones connected to the USART host interface
    // To configure/instantiate serial peripheral channel we have two options:
    // Use the device peripheral handle to directly access USART2 and instantiate a transmitter instance
    let mut tx = dp
        .USART2
        .tx(
            tx_pin,
            Config::default()
                .baudrate(115200.bps())
                .wordlength_8()
                .parity_none(),
            &clocks,
        )
        .unwrap();
    // or
    // Use the Serial abstraction to instantiate a transmitter instance
    // let mut tx = Serial::tx(
    //     dp.USART2,
    //     tx_pin,
    //     Config::default()
    //         .baudrate(115200.bps())
    //         .wordlength_8()
    //         .parity_none(),
    //     &clocks,
    // )
    // .unwrap();

    // Create and initialize a delay variable to manage delay loop
    let mut del_var = 7_0000_i32;

    // Initialize LED to on or off
    led.set_low();

    // Initialize debouncer to false because button is active low
    // Chose 3 consecutive states based on testing
    let mut debouncer = debounce_3(false);

    // Variable to keep track of how many button presses occured
    let mut value: u8 = 0;

    // Application Loop
    loop {
        // Enter Delay Loop
        for _i in 1..del_var {
            // Keep checking if button got pressed
            if debouncer.update(button.is_low()) == Some(Edge::Falling) {
                // If button is pressed print to derial and decrease the delay value
                writeln!(tx, "Button Press {:02}\r", value).unwrap();
                // Increment value keeping track of button presses
                value = value.wrapping_add(1);
                // Decrement the amount of delay
                del_var = del_var - 3_0000_i32;
                // If updated delay value drops below threshold then reset it back to starting value
                if del_var < 1_0000 {
                    del_var = 7_0000_i32;
                }
                // Exit delay loop since button was pressed
                break;
            }
        }

        // Delay loop without button debouncing
        // for _i in 1..del_var {
        //     // Keep checking if button got pressed
        //     if button.is_low() {
        //         // If button is pressed print to derial and decrease the delay value
        //         writeln!(tx, "Button Pressed\r").unwrap();
        //         del_var = del_var - 3_0000_i32;
        //         // If updated delay value drops below threshold then reset it back to starting value
        //         if del_var < 1_0000 {
        //             del_var = 7_0000_i32;
        //         }
        //         // Exit loop
        //         break;
        //     }
        // }

        // Toggle LED
        led.toggle();
    }
}
