#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::fmt::Write;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use heapless::String;

use cortex_m_rt::entry;
use embassy_stm32::dma::NoDma;
use embassy_stm32::i2c::I2c;
use embassy_stm32::interrupt;
use embassy_stm32::time::hz;
use embassy_stm32::usart::{Config, UartTx};
use embassy_time::Delay;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // Initialize and create handle for devicer peripherals
    let p = embassy_stm32::init(Default::default());

    let irq = interrupt::take!(I2C1_EV);
    // I2C Configuration
    let mut i2c = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        irq,
        NoDma,
        NoDma,
        hz(100000),
        Default::default(),
    );

    //Configure UART
    let mut usart = UartTx::new(p.USART2, p.PA2, NoDma, Config::default());

    // Create empty String for message
    let mut msg: String<64> = String::new();

    // Delay Handle
    let mut delay = Delay;

    struct Coeffs {
        ac5: i16,
        ac6: i16,
        mc: i16,
        md: i16,
    }

    let mut calib_coeffs = Coeffs {
        ac5: 0,
        ac6: 0,
        mc: 0,
        md: 0,
    };

    const BMP180_ADDR: u8 = 0x77;
    const REG_ID_ADDR: u8 = 0xD0;
    const AC5_MSB_ADDR: u8 = 0xB2;
    const AC6_MSB_ADDR: u8 = 0xB4;
    const MC_MSB_ADDR: u8 = 0xBC;
    const MD_MSB_ADDR: u8 = 0xBE;
    const CTRL_MEAS_ADDR: u8 = 0xF4;
    const MEAS_OUT_LSB_ADDR: u8 = 0xF7;
    const MEAS_OUT_MSB_ADDR: u8 = 0xF6;

    let mut rx_buffer: [u8; 2] = [0; 2];
    let mut rx_word: i16;

    // Read Device ID as Sanity Check
    i2c.blocking_write(BMP180_ADDR, &[REG_ID_ADDR]).unwrap();
    i2c.blocking_read(BMP180_ADDR, &mut rx_buffer).unwrap();

    if rx_buffer[0] == 0x55 {
        core::writeln!(&mut msg, "Device ID is {}\r", rx_buffer[0]).unwrap();
        // Transmit Message
        usart.blocking_write(msg.as_bytes()).unwrap();
        // Clear String for next message
        msg.clear();
    } else {
        core::writeln!(&mut msg, "Device ID Cannot be Detected \r").unwrap();
        usart.blocking_write(msg.as_bytes()).unwrap();
        msg.clear();
    }

    // Read Calibration Coefficients
    // Read AC5
    i2c.blocking_write_read(BMP180_ADDR, &[AC5_MSB_ADDR], &mut rx_buffer)
        .unwrap();
    rx_word = ((rx_buffer[0] as i16) << 8) | rx_buffer[1] as i16;
    core::writeln!(&mut msg, "AC5 = {} \r", rx_word).unwrap();
    usart.blocking_write(msg.as_bytes()).unwrap();
    msg.clear();
    calib_coeffs.ac5 = rx_word;

    // Read AC6
    i2c.blocking_write_read(BMP180_ADDR, &[AC6_MSB_ADDR], &mut rx_buffer)
        .unwrap();
    rx_word = ((rx_buffer[0] as i16) << 8) | rx_buffer[1] as i16;
    core::writeln!(&mut msg, "AC6 = {} \r", rx_word).unwrap();
    usart.blocking_write(msg.as_bytes()).unwrap();
    msg.clear();
    calib_coeffs.ac6 = rx_word;

    // Read MC
    i2c.blocking_write_read(BMP180_ADDR, &[MC_MSB_ADDR], &mut rx_buffer)
        .unwrap();
    rx_word = ((rx_buffer[0] as i16) << 8) | rx_buffer[1] as i16;
    core::writeln!(&mut msg, "MC = {} \r", rx_word).unwrap();
    usart.blocking_write(msg.as_bytes()).unwrap();
    msg.clear();
    calib_coeffs.mc = rx_word;

    // Read MD
    i2c.blocking_write_read(BMP180_ADDR, &[MD_MSB_ADDR], &mut rx_buffer)
        .unwrap();
    rx_word = ((rx_buffer[0] as i16) << 8) | rx_buffer[1] as i16;
    core::writeln!(&mut msg, "MD = {} \r", rx_word).unwrap();
    usart.blocking_write(msg.as_bytes()).unwrap();
    msg.clear();
    calib_coeffs.md = rx_word;

    // Application Loop
    loop {
        // Kick off Temperature Measurement by writing 0x2E in register 0xF4
        i2c.blocking_write(BMP180_ADDR, &[CTRL_MEAS_ADDR, 0x2E])
            .unwrap();
        // Wait at least 4.5 ms for measurment to complete as specified by the datasheet
        delay.delay_ms(5_u32);

        // Collect Temperature Measurment
        // Read Measurement MSB
        // Achieving same as above using an alternate method syntax here to do a write followed by read
        i2c.blocking_write(BMP180_ADDR, &[MEAS_OUT_MSB_ADDR])
            .unwrap();
        i2c.blocking_read(BMP180_ADDR, &mut rx_buffer).unwrap();
        rx_word = (rx_buffer[0] as i16) << 8;
        // Read Measurement LSB
        i2c.blocking_write(BMP180_ADDR, &[MEAS_OUT_LSB_ADDR])
            .unwrap();
        i2c.blocking_read(BMP180_ADDR, &mut rx_buffer).unwrap();
        rx_word |= rx_buffer[0] as i16;

        // Uncomment following line to print raw uncompenstated temperature value
        //writeln!(tx, "UT = {} \r", rx_word).unwrap();

        // Calculate Temperature According to Datasheet Formulas
        let x1 = (rx_word as i32 - calib_coeffs.ac6 as i32) * (calib_coeffs.ac5 as i32) >> 15;
        let x2 = ((calib_coeffs.mc as i32) << 11) / (x1 + calib_coeffs.md as i32);
        let b5 = x1 + x2;
        let t = ((b5 + 8) >> 4) / 10;

        // Print Temperature Value
        core::writeln!(&mut msg, "Temperature = {:} \r", t).unwrap();
        usart.blocking_write(msg.as_bytes()).unwrap();
        msg.clear();
    }
}
