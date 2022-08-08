#![no_std]
#![no_main]

// Imports
use core::fmt::Write; // allows use to use the WriteLn! macro for easy printing
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    gpio::PinState,
    pac::{self},
    prelude::*,
    serial::config::Config,
};

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Configure the ultasonic device echo pin as input and obtain handler.
    let gpioa = dp.GPIOA.split();
    let mut echo = gpioa.pa8;

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

    // Delay Configuration
    // Set up a microsecond delay handler
    let mut delay = dp.TIM1.delay_us(&clocks);

    // Counter/timer congig
    // Set up a microsecond counter handler
    let mut counter = dp.TIM2.counter_us(&clocks);

    // Algorithim
    // 1) Set pin ouput to low for 5 us to get clean low pulse
    // 2) Set pin output to high (trigger) for 10us
    // 3) Switch back to input
    // 4) Keep checking if pin goes high
    // 5) Once pin goes high start kick off counter/timer
    // 6) Wait for Pin to go low
    // 7) Obtain pulse measurement from timer
    // 8) Print out measurement on Serial
    // 9) Go back to 1)

    // Application Loop
    loop {
        // 1) Set pin ouput to low for 5 us to get clean low pulse
        echo.with_push_pull_output_in_state(PinState::Low, |_f| delay.delay_us(5_u32));

        // 2) Set pin output to high (trigger) for 10us
        // 3) Switch back to input
        echo.with_push_pull_output_in_state(PinState::High, |_f| delay.delay_us(10_u32));

        // Wait until pin goes high
        while !(echo.is_high()) {}

        // Kick off timer measurement with a max timeout Duration of 100ms?? defined by data sheet (longest distance that can be measured)
        counter.start(1000.millis()).unwrap();

        // Wait until pin goes low.
        while !(echo.is_low()) {}

        // Stop timer and collect elapsed time
        let duration = counter.now().duration_since_epoch();
        counter.cancel().unwrap();

        // Calculate the distance in cms using formula in datasheet
        let distance_cm = duration.to_micros() / 2 / 29;

        // Send calculated distance to serial interface
        writeln!(tx, "Distance {:02} cm\r", distance_cm).unwrap();
    }
}
