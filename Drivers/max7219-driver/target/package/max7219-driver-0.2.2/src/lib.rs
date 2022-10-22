//! This crate provides a platform agnostic driver for the MAX7219 LED Driver IC.
//!
//! This driver was built using the [embedded-hal](https://docs.rs/embedded-hal/0.2.7/embedded_hal/) traits.
//!
//! ## Usage
//!
//! An updated version of the library should be available on crates.io. Add the following to your Cargo.toml to get is a dependency.
//! ```rust
//! [dependencies]
//! max7219-driver = "*"
//! ```
//!
//! ### Instantiating
//!
//! Create an instance of the driver with the `new` method, by passing SPI and Output pin instances.
//! ```rust
//! use max7219-driver::MAX7219;
//! let spi = // SPI instantiation code
//! let cs = // Output pin instantiation code
//! let mut max7219 = MAX7219::new(spi, cs).unwrap();
//! ```
//!
//! ### Initializing
//!
//! Initialize the driver instance with the `init_display` method. A boolean needs to be specified to indicate whether to clear the display after init or not.
//! ```rust
//!  max7219.init_display(true);
//! ```
//!
//! ## Cascading Multiple MAX7219 Devices
//!
//! If you are cascading multiple devices that are driving 8x8 LED Dot Matrices instead use the `MAX7219LedMat` driver as follows:
//!
//! ### Instantiating
//!
//! ```rust
//! use max7219-driver::MAX7219LedMat;
//! let spi = // SPI instantiation code
//! let cs = // Output pin instantiation code
//! let mut max7219: MAX7219LedMat<_, _, BUFLEN, COUNT> = MAX7219LedMat::new(spi, cs).unwrap();
//! ```
//!
//! `BUFLEN` should be replaced with value equivalent to the total number of pixels/LED in the cascaded displays
//! and `COUNT` with the number of displays
//!
//! Example:
//!
//! If four displays are cascaded then `BUFLEN` should be replaced with 256 (= 8 x 8 x 4) and `COUNT` replaced with 4 resulting in
//!
//! ```rust
//! let mut max7219: MAX7219LedMat<_, _, 256, 4> = MAX7219LedMat::new(spi, cs).unwrap();
//! ```
//!
//! ### Initializing
//!
//! Exactly the same as earlier, the new driver takes care of initializing all the cscaded displays.
//!
//! ```rust
//!  max7219.init_display(true);
//! ```

#![no_std]

use embedded_graphics::pixelcolor::BinaryColor;
use embedded_hal as hal;

use embedded_graphics::prelude::*;

use num_enum::TryFromPrimitive;

use hal::blocking::spi::Write;
use hal::digital::v2::OutputPin;
use hal::spi::{Mode, Phase, Polarity};

/// This is a constant for convenience so that the SPI module can be configured with the correct SPI mode compatible the MAX7219 device
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

/// The MAX7219 Driver that initializes and communicates with a single MAX7219 IC or chain of ICs.
/// This driver usage is specific to connecting the 8x8 LED Dot Matrix
pub struct MAX7219LedMat<SPI, CS, const BUFLEN: usize, const COUNT: usize> {
    spi: SPI,
    cs: CS,
    framebuffer: [u8; BUFLEN],
}

impl<SPI, CS, const BUFLEN: usize, const COUNT: usize> MAX7219LedMat<SPI, CS, BUFLEN, COUNT>
where
    SPI: Write<u8>,
    CS: OutputPin,
{
    ///
    /// Constructor method. Creates a new instance of the MAX7219 driver.
    ///
    /// # Arguments
    ///
    /// * `spi` - a SPI peripheral instance
    /// * `cs` - a OutputPin instance
    ///
    /// # Errors
    ///
    /// * `DriverError` - returned if there is an error during transfer or addressing device
    ///
    pub fn new(spi: SPI, cs: CS) -> Result<Self, DriverError> {
        let max7219 = MAX7219LedMat::<SPI, CS, BUFLEN, COUNT> {
            spi: spi,
            cs: cs,
            framebuffer: [0; BUFLEN],
        };
        Ok(max7219)
    }

    /// Method to flush framebuffer to display. This method needs to be called everytime a new framebuffer is created,
    /// otherwise the frame will not appear on the screen.
    ///     
    /// # Arguments
    ///
    /// * None
    ///
    /// # Errors
    ///
    /// * `DriverError` - returned if there is an error during transfer or addressing device
    ///
    pub fn flush(&mut self) -> Result<(), DriverError> {
        // Iterate over all row addresses
        for addr in 0..8 {
            // Prepare Device to accept data
            self.cs.set_low().map_err(|_| DriverError::PinError)?;

            // Send frame data for each Display
            for disp in (0..COUNT).rev() {
                let base = ((disp + addr) * COUNT) * 8;
                let arr = &self.framebuffer[base..base + 8];
                // Convert each row in the framebuffer to a decimal num
                let mut res: u8 = 0;
                for i in 0..arr.len() {
                    res |= arr[i] << arr.len() - 1 - i;
                }
                self.spi
                    .write(&[addr as u8 + 1, res])
                    .map_err(|_| DriverError::SpiError)?;
            }

            // Latch Data Sent to Device(s)
            self.cs.set_high().map_err(|_| DriverError::PinError)?;
        }
        Ok(())
    }

    ///
    /// Transmits raw data to the MAX7219 IC.
    ///
    /// # Arguments
    ///
    /// * `arr` - slice of data that needs to be transmitted
    ///
    /// # Errors
    ///
    /// * `DriverError` - returned if there is an error during transfer or addressing device
    ///
    pub fn transmit_raw_data(&mut self, arr: &[u8]) -> Result<(), DriverError> {
        self.cs.set_low().map_err(|_| DriverError::PinError)?;
        let transfer = self.spi.write(&arr).map_err(|_| DriverError::SpiError);
        self.cs.set_high().map_err(|_| DriverError::PinError)?;
        transfer
    }

    ///
    /// Configures the power mode of the MAX7219 IC.
    ///
    /// # Arguments
    ///
    /// * `mode` - one of the options in the `Shutdown` enum
    ///
    pub fn config_power_mode(&mut self, mode: Shutdown) -> () {
        let data: u8 = match mode {
            Shutdown::NormalOperation => 0x01,
            Shutdown::ShutDownMode => 0x00,
        };

        let send_array: [u8; 2] = [Command::Shutdown as u8, data];
        // Transmit Data
        self.transmit_raw_data(&send_array).unwrap();
    }

    ///
    /// Configures the decode mode on the input sent to the MAX7219 IC.
    ///
    /// # Arguments
    ///
    /// * `mode` - one of the options in the `DecodeMode` enum
    ///
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

    ///
    /// Configures the intensity of the LEDs on the display connected to the MAX7219 IC.
    ///
    /// # Arguments
    ///
    /// * `mode` - one of the options in the `Intensity` enum
    ///
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

    ///
    /// Configures the scanlimit for the MAX7219 IC.
    /// Applicable mostly to seven segment displays if certain digits (ex. on the left)
    /// need not to be shown.
    ///
    /// # Arguments
    ///
    /// * `mode` - one of the options in the `ScanLimit` enum
    ///
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

    ///
    /// Method to perform a visual test of the display.
    /// If performing a test, display needs to be put back in normal operation mode after done.
    ///
    /// # Arguments
    ///
    /// * `mode` - one of the options in the `DisplayTest` enum
    ///
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

    ///
    /// Method to draw a row if the MAX7219 is driving an 8x8 LED dot matrix display.
    /// Alternatively method will draw a digit in case the MAX7219 is driving a seven-segment display
    ///
    /// # Arguments
    ///
    /// * `digit_addr` - one of the options in the `DigitRowAddress` enum
    /// * `led_data` - the led row or seven segment digit activation data
    ///
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

    ///
    /// Method to clear the display.
    ///
    pub fn clear_display(&mut self) -> () {
        for i in 1..9 {
            self.transmit_raw_data(&[i, 0]).unwrap();
        }
    }

    ///
    /// Method to initialize the MAX7219 and the connected display.
    /// This method has to be called before doing any display operations otherwise the display will not operate properly.
    /// The method provides an option to leave the display uncleared after initalization.
    ///  
    /// # Arguments
    ///
    /// * `clr_display` - Boolean that reflects whether the display should be cleared or not after init
    ///
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

/// The MAX7219 Driver that initializes and communicates with a single MAX7219 IC.
pub struct MAX7219<SPI, CS> {
    spi: SPI,
    cs: CS,
}

///
/// Possible Errors that can be raised either
/// during communication with the MAX7219 chip over SPI
/// or controlling the chip select pin.
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
    ///
    /// Constructor method. Creates a new instance of the MAX7219 driver.
    ///
    /// # Arguments
    ///
    /// * `spi` - a SPI peripheral instance
    /// * `cs` - a OutputPin instance
    ///
    /// # Errors
    ///
    /// * `DriverError` - returned if there is an error during transfer or addressing device
    ///
    pub fn new(spi: SPI, cs: CS) -> Result<Self, DriverError> {
        let max7219 = MAX7219 { spi: spi, cs: cs };
        Ok(max7219)
    }

    ///
    /// Transmits raw data to the MAX7219 IC.
    ///
    /// # Arguments
    ///
    /// * `arr` - slice of data that needs to be transmitted
    ///
    /// # Errors
    ///
    /// * `DriverError` - returned if there is an error during transfer or addressing device
    ///
    pub fn transmit_raw_data(&mut self, arr: &[u8]) -> Result<(), DriverError> {
        self.cs.set_low().map_err(|_| DriverError::PinError)?;
        let transfer = self.spi.write(&arr).map_err(|_| DriverError::SpiError);
        self.cs.set_high().map_err(|_| DriverError::PinError)?;
        transfer
    }

    ///
    /// Configures the power mode of the MAX7219 IC.
    ///
    /// # Arguments
    ///
    /// * `mode` - one of the options in the `Shutdown` enum
    ///
    pub fn config_power_mode(&mut self, mode: Shutdown) -> () {
        let data: u8 = match mode {
            Shutdown::NormalOperation => 0x01,
            Shutdown::ShutDownMode => 0x00,
        };

        let send_array: [u8; 2] = [Command::Shutdown as u8, data];
        // Transmit Data
        self.transmit_raw_data(&send_array).unwrap();
    }

    ///
    /// Configures the decode mode on the input sent to the MAX7219 IC.
    ///
    /// # Arguments
    ///
    /// * `mode` - one of the options in the `DecodeMode` enum
    ///
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

    ///
    /// Configures the intensity of the LEDs on the display connected to the MAX7219 IC.
    ///
    /// # Arguments
    ///
    /// * `mode` - one of the options in the `Intensity` enum
    ///
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

    ///
    /// Configures the scanlimit for the MAX7219 IC.
    /// Applicable mostly to seven segment displays if certain digits (ex. on the left)
    /// need not to be shown.
    ///
    /// # Arguments
    ///
    /// * `mode` - one of the options in the `ScanLimit` enum
    ///
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

    ///
    /// Method to perform a visual test of the display.
    /// If performing a test, display needs to be put back in normal operation mode after done.
    ///
    /// # Arguments
    ///
    /// * `mode` - one of the options in the `DisplayTest` enum
    ///
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

    ///
    /// Method to draw a row if the MAX7219 is driving an 8x8 LED dot matrix display.
    /// Alternatively method will draw a digit in case the MAX7219 is driving a seven-segment display
    ///
    /// # Arguments
    ///
    /// * `digit_addr` - one of the options in the `DigitRowAddress` enum
    /// * `led_data` - the led row or seven segment digit activation data
    ///
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

    ///
    /// Method to clear the display.
    ///
    pub fn clear_display(&mut self) -> () {
        for i in 1..9 {
            self.transmit_raw_data(&[i, 0]).unwrap();
        }
    }

    ///
    /// Method to initialize the MAX7219 and the connected display.
    /// This method has to be called before doing any display operations otherwise the display will not operate properly.
    /// The method provides an option to leave the display uncleared after initalization.
    ///  
    /// # Arguments
    ///
    /// * `clr_display` - Boolean that reflects whether the display should be cleared or not after init
    ///
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

impl<SPI, CS, const BUFLEN: usize, const COUNT: usize> DrawTarget
    for MAX7219LedMat<SPI, CS, BUFLEN, COUNT>
{
    type Color = BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let bb = self.bounding_box();
        pixels
            .into_iter()
            .filter(|Pixel(pos, _color)| bb.contains(*pos))
            .for_each(|Pixel(pos, color)| {
                let index: u32 = pos.x as u32 + pos.y as u32 * 8;
                self.framebuffer[index as usize] = color.is_on() as u8;
            });
        Ok(())
    }
}

impl<SPI, CS, const BUFLEN: usize, const COUNT: usize> OriginDimensions
    for MAX7219LedMat<SPI, CS, BUFLEN, COUNT>
{
    fn size(&self) -> Size {
        Size::new(COUNT as u32 * 8, 8)
    }
}

/// Enumeration of commands in the register map of the MAX7219.
pub enum Command {
    NoOp = 0x00,
    DecodeMode = 0x09,
    Intensity = 0x0A,
    ScanLimit = 0x0B,
    Shutdown = 0x0C,
    DisplayTest = 0x0F,
}

/// Enumeration of the MAX7219 power modes.
pub enum Shutdown {
    ShutDownMode,
    NormalOperation,
}

/// Enumeration of the MAX7219 decode modes for BCD encoded input.
pub enum DecodeMode {
    NoDecode = 0x00,
    CodeB0 = 0x01,
    CodeB30 = 0x0F,
    CodeB70 = 0xFF,
}

/// Enumeration of the MAX7219 supported LED intensity values.
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

/// Enumeration of the MAX7219 display scan limits
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

/// Enumeration of the MAX7219 display test modes
pub enum DisplayTest {
    NormalOperationMode = 0x00,
    DisplayTestMode = 0x01,
}

/// Enumeration of the MAX7219 digit/row addresses
#[repr(u8)]
#[derive(Debug, TryFromPrimitive)]
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
