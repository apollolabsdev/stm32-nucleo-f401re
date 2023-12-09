#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::fmt::Write;

use heapless::String;

use libm::log;

use cortex_m_rt::entry;
use embassy_stm32::adc::Adc;
use embassy_stm32::dma::NoDma;
use embassy_stm32::usart::{Config, UartTx};
use embassy_time::Delay;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // Initialize and create handle for devicer peripherals
    let mut p = embassy_stm32::init(Default::default());

    // ADC Configuration
    let mut delay = Delay;
    // Create Handler for adc peripheral (PA0 is connected to ADC1)
    let mut adc = Adc::new(p.ADC1, &mut delay);

    //Configure UART
    let mut usart = UartTx::new(p.USART2, p.PA2, NoDma, Config::default()).unwrap();

    // Create empty String for message
    let mut msg: String<64> = String::new();

    static R0: f64 = 100000.0;
    static B: f64 = 4275.0; // B value of the thermistor

    // Algorithm
    // 1) Get adc reading
    // 2) Convert to temperature
    // 3) Send over Serial
    // 4) Go Back to step 1

    // Application Loop
    loop {
        // Get ADC reading
        let sample = adc.read(&mut p.PA0);

        //Convert to temperature
        let mut r: f64 = 4094.0 / sample as f64 - 1.0;
        r = R0 * r;
        let temperature = (1.0 / (log(r / R0) / B + 1.0 / 298.15)) - 273.15;

        // Format Message
        core::writeln!(&mut msg, "Temperature {:02} Celcius\r", temperature).unwrap();

        // Transmit Message
        usart.blocking_write(msg.as_bytes()).unwrap();

        // Clear String for next message
        msg.clear();
    }
}
