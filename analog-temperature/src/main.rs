#![no_std]
#![no_main]

// Imports
use core::fmt::Write; // allows use to use the WriteLn! macro for easy printing
use cortex_m_rt::entry;
use libm::log;
use panic_halt as _;
use stm32f4xx_hal::{
    adc::{config::AdcConfig, config::SampleTime, Adc},
    pac,
    prelude::*,
    serial::config::Config,
};

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();

    // ADC Configuration Steps:
    // 1) Configure the temperature sensor temperature pin into analog and obtain handler.
    let gpioa = dp.GPIOA.split();
    let temperature_pin = gpioa.pa0.into_analog();
    // 2) Create Handler for adc peripheral (PA0 is connected to ADC1)
    // Configure ADC for single shot conversion
    let mut adc = Adc::adc1(dp.ADC1, true, AdcConfig::default());

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

    static R0: f64 = 100000.0;
    static B: f64 = 4275.0; // B value of the thermistor

    // Algorithim
    // 1) Get adc reading
    // 2) Convert to temperature
    // 3) Send over Serial
    // 4) Go Back to step 1

    // Application Loop
    loop {
        // Get ADC reading
        let sample = adc.convert(&temperature_pin, SampleTime::Cycles_480);

        //Convert to temperature
        let mut r: f64 = 4094.0 / sample as f64 - 1.0;
        r = R0 * r;
        let temperature = (1.0 / (log(r / R0) / B + 1.0 / 298.15)) - 273.15;

        // Send temperature to serial interface
        writeln!(tx, "Temperature {:02} Celcius\r", temperature).unwrap();
    }
}
