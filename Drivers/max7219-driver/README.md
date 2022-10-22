This crate provides a platform agnostic driver for the MAX7219 LED Driver IC.

This driver was built using the [embedded-hal](https://docs.rs/embedded-hal/0.2.7/embedded_hal/) traits.

# Usage
## Single MAX7219 Device

An updated version of the library should be available on crates.io. Add the following to your Cargo.toml to get is a dependency.
```rust
[dependencies]
max7219-driver = "*"
```

### Instantiating

Create an instance of the driver with the `new` method, by passing SPI and Output pin instances.

```rust
use max7219-driver::MAX7219;
let spi = // SPI instantiation code
let cs = // Output pin instantiation code
let mut max7219 = MAX7219::new(spi, cs).unwrap();
```

### Initializing

Initialize the driver instance with the `init_display` method. A boolean needs to be specified to indicate whether to clear the display after init or not.

```rust
 max7219.init_display(true);
```

## Cascading Multiple MAX7219 Devices
 
If you are cascading multiple devices that are driving 8x8 LED Dot Matrices instead use the `MAX7219LedMat` driver as follows:

### Instantiating
 
```rust
use max7219-driver::MAX7219LedMat;
let spi = // SPI instantiation code
let cs = // Output pin instantiation code
let mut max7219: MAX7219LedMat<_, _, BUFLEN, COUNT> = MAX7219LedMat::new(spi, cs).unwrap();
```

`BUFLEN` should be replaced with value equivalent to the total number of pixels/LED in the cascaded displays and `COUNT` with the number of displays

**Example:**

If four displays are cascaded then `BUFLEN` should be replaced with 256 (= 8 x 8 x 4) and `COUNT` replaced with 4 resulting in

```rust
let mut max7219: MAX7219LedMat<_, _, 256, 4> = MAX7219LedMat::new(spi, cs).unwrap();
```
 
### Initializing

Exactly the same as earlier, the new driver takes care of initializing all the cscaded displays.

```rust
max7219.init_display(true);
```