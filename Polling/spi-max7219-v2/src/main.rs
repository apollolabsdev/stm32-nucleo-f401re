#![no_std]
#![no_main]

// Imports
use core::convert::TryFrom;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    gpio::{Alternate, NoPin, Output, Pin},
    pac::{self, SPI1},
    prelude::*,
    spi::{Mode, NoMiso, Phase, Polarity, Spi, TransferModeNormal},
};

// Enumeration of Digit addresses in the register map of the MAX7219
// Corresponds to Table 2 in MAX7219 datasheet
#[derive(Debug)]
enum DigitRowAddress {
    Digit0 = 0x01,
    Digit1 = 0x02,
    Digit2 = 0x03,
    Digit3 = 0x04,
    Digit4 = 0x05,
    Digit5 = 0x06,
    Digit6 = 0x07,
    Digit7 = 0x08,
}

// Implement TryFrom Trait on RowAddress to retrieve corresponding digit
impl TryFrom<u8> for DigitRowAddress {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use DigitRowAddress::*;

        Ok(match value {
            0x01 => Digit0,
            0x02 => Digit1,
            0x03 => Digit2,
            0x04 => Digit3,
            0x05 => Digit4,
            0x06 => Digit5,
            0x07 => Digit6,
            0x08 => Digit7,
            invalid => return Err(invalid),
        })
    }
}

// Enumeration of command addresses in the register map of the MAX7219
// Corresponds to Table 2 in MAX7219 datasheet
enum Command {
    NoOp = 0x00,
    DecodeMode = 0x09,
    Intensity = 0x0A,
    ScanLimit = 0x0B,
    Shutdown = 0x0C,
    DisplayTest = 0x0F,
}

// Enumeration of all power modes of the MAX7219
// Corresponds to Table 3 in MAX7219 datasheet
enum Shutdown {
    ShutDownMode,
    NormalOperation,
}

// Enumeration of all decode modes of the MAX7219
// Corresponds to Table 4 in MAX7219 datasheet
enum DecodeMode {
    NoDecode = 0x00,
    CodeB0 = 0x01,
    CodeB30 = 0x0F,
    CodeB70 = 0xFF,
}

// Enumeration of all code B charcters of the MAX7219
// Corresponds to Table 5 in MAX7219 datasheet
enum SevenSegCharacter {
    Zero = 0x00,
    One = 0x01,
    Two = 0x02,
    Three = 0x03,
    Four = 0x04,
    Five = 0x05,
    Six = 0x06,
    Seven = 0x07,
    Eight = 0x08,
    Nine = 0x09,
    Dash = 0x0A,
    LetterE = 0x0B,
    LetterH = 0x0C,
    LetterL = 0x0D,
    LetterP = 0x0E,
    Blank = 0x0F,
}

// Enumeration of all possible LED intensities in the MAX7219
// Corresponds to Table 7 in MAX7219 datasheet
enum Intensity {
    Min = 0x00,
    Ratio3_32 = 0x01,
    Ratio5_32 = 0x02,
    Ratio7_32 = 0x03,
    Ratio9_32 = 0x04,
    Ratio11_32 = 0x05,
    Ratio13_32 = 0x06,
    Ratio15_32 = 0x07,
    Ratio17_32 = 0x08,
    Ratio19_32 = 0x09,
    Ratio21_32 = 0x0A,
    Ratio23_32 = 0x0B,
    Ratio25_32 = 0x0C,
    Ratio27_32 = 0x0D,
    Ratio29_32 = 0x0E,
    Max = 0x0F,
}

// Enumeration of Scan Limit Possibilities in the MAX7219
// Corresponds to Table 8 in MAX7219 datasheet
enum ScanLimit {
    Display0Only = 0x00,
    Display0And1 = 0x01,
    Display0To2 = 0x02,
    Display0To3 = 0x03,
    Display0To4 = 0x04,
    Display0To5 = 0x05,
    Display0To6 = 0x06,
    Display0To7 = 0x07,
}

// Enumeration of Display Test Modes in the MAX7219
// Corresponds to Table 10 in MAX7219 datasheet
enum DisplayTest {
    NormalOperationMode = 0x00,
    DisplayTestMode = 0x01,
}

#[derive(Debug)]
enum AddressError {
    AddressNotValid,
}

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Configure the system clocks
    // - Promote RCC structure to HAL to be able to configure clocks
    let rcc = dp.RCC.constrain();
    // - Configure system clocks
    // 8 MHz must be used for the Nucleo-F401RE board according to manual
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    // Configure the SPI pins
    // Don't need miso since communication is simplex (single direction)
    // PA5 and PA7 use SPI1 in the STM32F401RE
    let gpioa = dp.GPIOA.split();
    let sclk = gpioa.pa5.into_alternate();
    let mosi = gpioa.pa7.into_alternate();
    let mut cs = gpioa.pa6.into_push_pull_output();

    // Initialize SPI
    // Theres also two other methods to instantiate 'new' and 'init'!
    let mut spi = dp.SPI1.spi(
        (sclk, NoMiso {}, mosi),
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        2.MHz(),
        &clocks,
    );

    // configure Delay Handle
    let mut delay = dp.TIM2.delay_ms(&clocks);

    // Application Code
    // Initialize Display
    init_display(&mut spi, &mut cs, true);

    loop {
        let mut data: u8 = 1;
        for addr in 1..9 {
            draw_row_or_digit(
                &mut spi,
                &mut cs,
                DigitRowAddress::try_from(addr).unwrap(),
                data,
            )
            .unwrap();
            data = data << 1;
            delay.delay_ms(500_u32);
        }

        // Clear the LED matrix row by row with 500ms delay in between
        for addr in 1..9 {
            draw_row_or_digit(
                &mut spi,
                &mut cs,
                DigitRowAddress::try_from(addr).unwrap(),
                data,
            )
            .unwrap();
            delay.delay_ms(500_u32);
        }
    }
}

// Function that sends raw data
fn transmit_raw_data(
    arr: &[u8],
    per: &mut Spi<
        SPI1,
        (
            Pin<'A', 5_u8, Alternate<5_u8>>,
            NoPin,
            Pin<'A', 7_u8, Alternate<5_u8>>,
        ),
        TransferModeNormal,
    >,
    cs: &mut Pin<'A', 6_u8, Output>,
) -> Result<(), stm32f4xx_hal::spi::Error> {
    cs.set_low();
    let transfer = per.write(&arr);
    cs.set_high();
    transfer
}

// Function to configure device power
fn config_power_mode(
    per: &mut Spi<
        SPI1,
        (
            Pin<'A', 5_u8, Alternate<5_u8>>,
            NoPin,
            Pin<'A', 7_u8, Alternate<5_u8>>,
        ),
        TransferModeNormal,
    >,
    cs: &mut Pin<'A', 6_u8, Output>,
    mode: Shutdown,
) -> () {
    let data: u8 = match mode {
        Shutdown::NormalOperation => 0x01,
        Shutdown::ShutDownMode => 0x00,
    };

    let send_array: [u8; 2] = [Command::Shutdown as u8, data];
    // Transmit Data
    transmit_raw_data(&send_array, per, cs).unwrap();
}

// Function to Configure Decode Mode
fn config_decode_mode(
    per: &mut Spi<
        SPI1,
        (
            Pin<'A', 5_u8, Alternate<5_u8>>,
            NoPin,
            Pin<'A', 7_u8, Alternate<5_u8>>,
        ),
        TransferModeNormal,
    >,
    cs: &mut Pin<'A', 6_u8, Output>,
    mode: DecodeMode,
) -> () {
    // - Prepare Information to be Sent
    // 8-bit Data/Command Corresponding to No Decode Mode
    let data: u8 = match mode {
        DecodeMode::NoDecode => 0x00,
        DecodeMode::CodeB0 => 0x01,
        DecodeMode::CodeB30 => 0x0F,
        DecodeMode::CodeB70 => 0xFF,
    };
    // Package into array to pass to SPI write method
    // Write method will grab array and send all data in it
    let send_array: [u8; 2] = [Command::DecodeMode as u8, data];
    // Transmit Data
    transmit_raw_data(&send_array, per, cs).unwrap();
}

// Function to configure intensity
fn config_intensity(
    per: &mut Spi<
        SPI1,
        (
            Pin<'A', 5_u8, Alternate<5_u8>>,
            NoPin,
            Pin<'A', 7_u8, Alternate<5_u8>>,
        ),
        TransferModeNormal,
    >,
    cs: &mut Pin<'A', 6_u8, Output>,
    mode: Intensity,
) -> () {
    // - Prepare Information to be Sent
    // 8-bit Data/Command Corresponding to No Decode Mode
    let data: u8 = match mode {
        Intensity::Min => 0x00,
        Intensity::Ratio3_32 => 0x01,
        Intensity::Ratio5_32 => 0x02,
        Intensity::Ratio7_32 => 0x03,
        Intensity::Ratio9_32 => 0x04,
        Intensity::Ratio11_32 => 0x05,
        Intensity::Ratio13_32 => 0x06,
        Intensity::Ratio15_32 => 0x07,
        Intensity::Ratio17_32 => 0x08,
        Intensity::Ratio19_32 => 0x09,
        Intensity::Ratio21_32 => 0x0A,
        Intensity::Ratio23_32 => 0x0B,
        Intensity::Ratio25_32 => 0x0C,
        Intensity::Ratio27_32 => 0x0D,
        Intensity::Ratio29_32 => 0x0E,
        Intensity::Max => 0x0F,
    };
    // Package into array to pass to SPI write method
    // Write method will grab array and send all data in it
    let send_array: [u8; 2] = [Command::Intensity as u8, data];
    // Transmit Data
    transmit_raw_data(&send_array, per, cs).unwrap();
}

// Function to configure the scan limit
fn config_scan_limit(
    per: &mut Spi<
        SPI1,
        (
            Pin<'A', 5_u8, Alternate<5_u8>>,
            NoPin,
            Pin<'A', 7_u8, Alternate<5_u8>>,
        ),
        TransferModeNormal,
    >,
    cs: &mut Pin<'A', 6_u8, Output>,
    mode: ScanLimit,
) -> () {
    // - Prepare Information to be Sent
    // 8-bit Data/Command Corresponding to No Decode Mode
    let data: u8 = match mode {
        ScanLimit::Display0Only => 0x00,
        ScanLimit::Display0And1 => 0x01,
        ScanLimit::Display0To2 => 0x02,
        ScanLimit::Display0To3 => 0x03,
        ScanLimit::Display0To4 => 0x04,
        ScanLimit::Display0To5 => 0x05,
        ScanLimit::Display0To6 => 0x06,
        ScanLimit::Display0To7 => 0x07,
    };
    // Package into array to pass to SPI write method
    // Write method will grab array and send all data in it
    let send_array: [u8; 2] = [Command::ScanLimit as u8, data];
    // Transmit Data
    transmit_raw_data(&send_array, per, cs).unwrap();
}

// Function to test the display
fn display_test(
    per: &mut Spi<
        SPI1,
        (
            Pin<'A', 5_u8, Alternate<5_u8>>,
            NoPin,
            Pin<'A', 7_u8, Alternate<5_u8>>,
        ),
        TransferModeNormal,
    >,
    cs: &mut Pin<'A', 6_u8, Output>,
    mode: DisplayTest,
) -> () {
    // - Prepare Information to be Sent
    // 8-bit Data/Command Corresponding to No Decode Mode
    let data: u8 = match mode {
        DisplayTest::NormalOperationMode => 0x00,
        DisplayTest::DisplayTestMode => 0x01,
    };
    // Package into array to pass to SPI write method
    // Write method will grab array and send all data in it
    let send_array: [u8; 2] = [Command::DisplayTest as u8, data];
    // Transmit Data
    transmit_raw_data(&send_array, per, cs).unwrap();
}

// Function that draws a row in case of an 8x8 dot matrix or a digit in case of seven segment
fn draw_row_or_digit(
    per: &mut Spi<
        SPI1,
        (
            Pin<'A', 5_u8, Alternate<5_u8>>,
            NoPin,
            Pin<'A', 7_u8, Alternate<5_u8>>,
        ),
        TransferModeNormal,
    >,
    cs: &mut Pin<'A', 6_u8, Output>,
    digit_addr: DigitRowAddress,
    led_data: u8,
) -> Result<(), AddressError> {
    let addr: u8 = match digit_addr {
        DigitRowAddress::Digit0 => 0x01,
        DigitRowAddress::Digit1 => 0x02,
        DigitRowAddress::Digit2 => 0x03,
        DigitRowAddress::Digit3 => 0x04,
        DigitRowAddress::Digit4 => 0x05,
        DigitRowAddress::Digit5 => 0x06,
        DigitRowAddress::Digit6 => 0x07,
        DigitRowAddress::Digit7 => 0x08,
        _ => return Err(AddressError::AddressNotValid),
    };
    let send_array: [u8; 2] = [addr, led_data];
    transmit_raw_data(&send_array, per, cs).unwrap();
    Ok(())
}

// Function that clears the display
fn clear_display(
    per: &mut Spi<
        SPI1,
        (
            Pin<'A', 5_u8, Alternate<5_u8>>,
            NoPin,
            Pin<'A', 7_u8, Alternate<5_u8>>,
        ),
        TransferModeNormal,
    >,
    cs: &mut Pin<'A', 6_u8, Output>,
) -> () {
    for i in 1..9 {
        transmit_raw_data(&[i, 0_u8], per, cs).unwrap();
    }
}

// Function to initialize the display
// This function should be called before doing any display operations
fn init_display(
    per: &mut Spi<
        SPI1,
        (
            Pin<'A', 5_u8, Alternate<5_u8>>,
            NoPin,
            Pin<'A', 7_u8, Alternate<5_u8>>,
        ),
        TransferModeNormal,
    >,
    cs: &mut Pin<'A', 6_u8, Output>,
    clr_display: bool,
) -> () {
    // 1.a) Power Up Device
    config_power_mode(per, cs, Shutdown::NormalOperation);
    // 1.b) Set up Decode Mode
    config_decode_mode(per, cs, DecodeMode::NoDecode);
    // 1.c) Configure Scan Limit
    config_scan_limit(per, cs, ScanLimit::Display0To7);
    // 1.d) Configure Intensity
    config_intensity(per, cs, Intensity::Ratio15_32);
    // 1.e) Optional Screen Clear on Init
    if clr_display {
        clear_display(per, cs);
    }
}
