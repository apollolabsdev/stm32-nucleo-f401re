#![no_std]
#![no_main]

// Imports
use core::cell::{Cell, RefCell};
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    gpio::{self, Edge, Input},
    pac::{self, interrupt},
    prelude::*,
};

// Create an alias for pin PC13
type ButtonPin = gpio::PC13<Input>;

// Global Variable Definitions
// Global variables are wrapped in safe abstractions.
// Peripherals are wrapped in a different manner than regular global mutable data.
// In the case of peripherals we must be sure only one refrence exists at a time.
// Refer to Chapter 6 of the Embedded Rust Book for more detail.

// Create a Global Variable for the GPIO Peripheral that I'm going to pass around.
static G_BUTTON: Mutex<RefCell<Option<ButtonPin>>> = Mutex::new(RefCell::new(None));
// Create a Global Variable for the delay value that I'm going to pass around for delay.
static G_DELAYMS: Mutex<Cell<u32>> = Mutex::new(Cell::new(2000_u32));

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let mut dp = pac::Peripherals::take().unwrap();

    // Configure and obtain handle for delay abstraction
    // 1) Promote RCC structure to HAL to be able to configure clocks
    let rcc = dp.RCC.constrain();
    // 2) Configure the system clocks
    // 8 MHz must be used for HSE on the Nucleo-F401RE board according to manual
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
    // 3) Create delay handle
    let mut delay = dp.TIM1.delay_ms(&clocks);

    // Configure the LED pin as a push pull ouput and obtain handle
    // On the Nucleo FR401 theres an on-board LED connected to pin PA5
    // 1) Promote the GPIOA PAC struct
    let gpioa = dp.GPIOA.split();
    // 2) Configure Pin and Obtain Handle
    let mut led = gpioa.pa5.into_push_pull_output();

    // Configure the button pin as input and obtain handle
    // On the Nucleo FR401 there is a button connected to pin PC13
    // 1) Promote the GPIOC PAC struct
    let gpioc = dp.GPIOC.split();
    // 2) Configure Pin and Obtain Handle
    let mut button = gpioc.pc13;

    // Configure Button Pin for Interrupts
    // 1) Promote SYSCFG structure to HAL to be able to configure interrupts
    let mut syscfg = dp.SYSCFG.constrain();
    // 2) Make button an interrupt source
    button.make_interrupt_source(&mut syscfg);
    // 3) Make button an interrupt source
    button.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
    // 4) Enable gpio interrupt for button
    button.enable_interrupt(&mut dp.EXTI);

    // Enable the external interrupt in the NVIC by passing the button interrupt number
    unsafe {
        cortex_m::peripheral::NVIC::unmask(button.interrupt());
    }

    // Now that button is configured, move button into global context
    cortex_m::interrupt::free(|cs| {
        G_BUTTON.borrow(cs).replace(Some(button));
    });

    // Application Loop
    loop {
        // Turn On LED
        led.set_high();
        // Obtain G_DELAYMS and delay
        delay.delay_ms(cortex_m::interrupt::free(|cs| G_DELAYMS.borrow(cs).get()));
        // Turn off LED
        led.set_low();
        // Obtain G_DELAYMS and delay
        delay.delay_ms(cortex_m::interrupt::free(|cs| G_DELAYMS.borrow(cs).get()));
    }
}

#[interrupt]
fn EXTI15_10() {
    // Start a Critical Section
    cortex_m::interrupt::free(|cs| {
        // Obtain Access to Delay Global Data and Adjust Delay
        G_DELAYMS
            .borrow(cs)
            .set(G_DELAYMS.borrow(cs).get() - 500_u32);
        if G_DELAYMS.borrow(cs).get() < 500_u32 {
            G_DELAYMS.borrow(cs).set(2000_u32);
        }
        // Obtain access to Global Button Peripheral and Clear Interrupt Pending Flag
        let mut button = G_BUTTON.borrow(cs).borrow_mut();
        button.as_mut().unwrap().clear_interrupt_pending_bit();
    });
}
