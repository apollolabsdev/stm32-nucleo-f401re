This crate provides a platform agnostic driver for the MAX7219 LED Driver IC.

This driver was built using the [embedded-hal](https://docs.rs/embedded-hal/0.2.7/embedded_hal/) traits.

## Usage

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