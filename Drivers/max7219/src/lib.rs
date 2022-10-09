#![no_std]

use embedded_hal as hal;

use hal::blocking::spi::Write;
use hal::digital::v2::OutputPin;
use hal::spi::{Mode, Phase, Polarity};

// This is a constant for convenience so that the SPI module can be configured with the correct mode for the MAX7219 device
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

/// MAX7219 Driver
pub struct MAX7219<SPI, CS> {
    spi: SPI,
    cs: CS,
}

///
/// Error raised in case there was an error
/// during communication with the MAX7219 chip.
///
#[derive(Debug)]
pub enum DriverError {
    /// An error occurred when working with SPI
    SpiError,
    /// An error occurred when working with a PIN
    PinError,
}

impl<SPI, CS> MAX7219<SPI, CS>
where
    SPI: Write<u8>,
    CS: OutputPin,
{
    pub fn new(spi: SPI, cs: CS) -> Result<Self, DriverError> {
        let max7219 = MAX7219 { spi: spi, cs: cs };
        Ok(max7219)
    }

    pub fn transmit_raw_data(&mut self, arr: &[u8]) -> Result<(), DriverError> {
        self.cs.set_low().map_err(|_| DriverError::PinError)?;
        let transfer = self.spi.write(&arr).map_err(|_| DriverError::SpiError);
        self.cs.set_high().map_err(|_| DriverError::PinError)?;
        transfer
    }

    pub fn config_power_mode(&mut self, mode: Shutdown) -> () {
        let data: u8 = match mode {
            Shutdown::NormalOperation => 0x01,
            Shutdown::ShutDownMode => 0x00,
        };

        let send_array: [u8; 2] = [Command::Shutdown as u8, data];
        // Transmit Data
        self.transmit_raw_data(&send_array).unwrap();
    }

    // Function to Configure Decode Mode
    pub fn config_decode_mode(&mut self, mode: DecodeMode) -> () {
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
        self.transmit_raw_data(&send_array).unwrap();
    }

    // Function to configure intensity
    pub fn config_intensity(&mut self, mode: Intensity) -> () {
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
        self.transmit_raw_data(&send_array).unwrap();
    }

    // Function to configure the scan limit
    pub fn config_scan_limit(&mut self, mode: ScanLimit) -> () {
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
        self.transmit_raw_data(&send_array).unwrap();
    }

    // Function to test the display
    pub fn display_test(&mut self, mode: DisplayTest) -> () {
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
        self.transmit_raw_data(&send_array).unwrap();
    }

    // Function that draws a row in case of an 8x8 dot matrix or a digit in case of seven segment
    pub fn draw_row_or_digit(&mut self, digit_addr: DigitRowAddress, led_data: u8) -> () {
        // Can do an exhaustive match of DigitRowAddress
        // let addr: u8 = match digit_addr {
        //     DigitRowAddress::Digit0 => 0x01,
        //     DigitRowAddress::Digit1 => 0x02,
        //     DigitRowAddress::Digit2 => 0x03,
        //     DigitRowAddress::Digit3 => 0x04,
        //     DigitRowAddress::Digit4 => 0x05,
        //     DigitRowAddress::Digit5 => 0x06,
        //     DigitRowAddress::Digit6 => 0x07,
        //     DigitRowAddress::Digit7 => 0x08,
        // };
        // OR typecast DigitRowAddress passed through digit_addr as u8
        let send_array: [u8; 2] = [digit_addr as u8, led_data];
        self.transmit_raw_data(&send_array).unwrap();
        ()
    }

    // Function that clears the display
    pub fn clear_display(&mut self) -> () {
        for i in 1..9 {
            self.transmit_raw_data(&[i]).unwrap();
        }
    }

    // Function to initialize the display
    // This function should be called before doing any display operations
    pub fn init_display(&mut self, clr_display: bool) -> () {
        // 1.a) Power Up Device
        self.config_power_mode(Shutdown::NormalOperation);
        // 1.b) Set up Decode Mode
        self.config_decode_mode(DecodeMode::NoDecode);
        // 1.c) Configure Scan Limit
        self.config_scan_limit(ScanLimit::Display0To7);
        // 1.d) Configure Intensity
        self.config_intensity(Intensity::Ratio15_32);
        // 1.e) Optional Screen Clear on Init
        if clr_display {
            self.clear_display();
        }
    }
}

// Enumeration of command addresses in the register map of the MAX7219
// Corresponds to Table 2 in MAX7219 datasheet
pub enum Command {
    NoOp = 0x00,
    DecodeMode = 0x09,
    Intensity = 0x0A,
    ScanLimit = 0x0B,
    Shutdown = 0x0C,
    DisplayTest = 0x0F,
}

// Enumeration of all power modes of the MAX7219
// Corresponds to Table 3 in MAX7219 datasheet
pub enum Shutdown {
    ShutDownMode,
    NormalOperation,
}

// Enumeration of all decode modes of the MAX7219
// Corresponds to Table 4 in MAX7219 datasheet
pub enum DecodeMode {
    NoDecode = 0x00,
    CodeB0 = 0x01,
    CodeB30 = 0x0F,
    CodeB70 = 0xFF,
}
// Enumeration of all possible LED intensities in the MAX7219
// Corresponds to Table 7 in MAX7219 datasheet
pub enum Intensity {
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
pub enum ScanLimit {
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
pub enum DisplayTest {
    NormalOperationMode = 0x00,
    DisplayTestMode = 0x01,
}

#[repr(u8)]
#[derive(Debug)]
pub enum DigitRowAddress {
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
