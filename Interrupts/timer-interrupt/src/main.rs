#![no_std]
#![no_main]

// Imports
use core::cell::{Cell, RefCell};
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    gpio::{self, Edge, Input, Output, PushPull},
    pac::{self, interrupt, TIM2},
    prelude::*,
    timer::{CounterMs, Event},
};

// Create aliases for pins PC13 and PA5
type ButtonPin = gpio::PC13<Input>;
type LedPin = gpio::PA5<Output<PushPull>>;

// Global Variable Definitions
// Global variables are wrapped in safe abstractions.
// If you notice peripherals are wrapped in a different manner than regular global mutable data.
// In the case of peripherals we must be sure only one refrence exists at a time.
// Refer to Chapter 6 of the Embedded Rust Book for more detail.

// Create a Global Variable for the Button GPIO Peripheral that I'm going to pass around.
static G_BUTTON: Mutex<RefCell<Option<ButtonPin>>> = Mutex::new(RefCell::new(None));
// Create a Global Variable for the Timer Peripheral that I'm going to pass around.
static G_TIM: Mutex<RefCell<Option<CounterMs<TIM2>>>> = Mutex::new(RefCell::new(None));
// Create a Global Variable for the LED GPIO Peripheral that I'm going to pass around.
static G_LED: Mutex<RefCell<Option<LedPin>>> = Mutex::new(RefCell::new(None));
// Create a Global Variable for the delay value that I'm going to use to manage the delay.
static G_DELAYMS: Mutex<Cell<u32>> = Mutex::new(Cell::new(2000));

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let mut dp = pac::Peripherals::take().unwrap();

    // Configure the LED pin as a push pull ouput and obtain handle
    // On the Nucleo FR401 theres an on-board LED connected to pin PA5
    // 1) Promote the GPIOA PAC struct
    let gpioa = dp.GPIOA.split();
    // 2) Configure Pin and Obtain Handle
    let led = gpioa.pa5.into_push_pull_output();

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

    // Configure and obtain handle for delay abstraction
    // 1) Promote RCC structure to HAL to be able to configure clocks
    let rcc = dp.RCC.constrain();
    // 2) Configure the system clocks
    // 8 MHz must be used for HSE on the Nucleo-F401RE board according to manual
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
    // 3) Create delay handle
    //let mut delay = dp.TIM1.delay_ms(&clocks);
    let mut timer = dp.TIM2.counter_ms(&clocks);

    // Kick off the timer with 2 seconds timeout first
    // It probably would be better to use the global variable here but I did not to avoid the clutter of having to create a crtical section
    timer.start(2000.millis()).unwrap();

    // Set up to generate interrupt when timer expires
    timer.listen(Event::Update);

    // Enable the external interrupt in the NVIC for all peripherals by passing the interrupt numbers
    unsafe {
        cortex_m::peripheral::NVIC::unmask(interrupt::TIM2);
        cortex_m::peripheral::NVIC::unmask(button.interrupt());
    }

    // Now that all peripherals are configured, move them into global context
    cortex_m::interrupt::free(|cs| {
        G_TIM.borrow(cs).replace(Some(timer));
        G_BUTTON.borrow(cs).replace(Some(button));
        G_LED.borrow(cs).replace(Some(led));
    });

    // Application Loop
    loop {
        // Go to sleep
        cortex_m::asm::wfi();
    }
}

// Button Interrupt
#[interrupt]
fn EXTI15_10() {
    // When Button interrupt happens three things need to be done
    // 1) Adjust Global Delay Variable
    // 2) Update Timer with new Global Delay value
    // 3) Clear Button Pending Interrupt

    // Start a Critical Section
    cortex_m::interrupt::free(|cs| {
        // Obtain Access to Delay Global Data and Adjust Delay
        G_DELAYMS
            .borrow(cs)
            .set(G_DELAYMS.borrow(cs).get() - 500_u32);

        // Reset delay value if it drops below 500 milliseconds
        if G_DELAYMS.borrow(cs).get() < 500_u32 {
            G_DELAYMS.borrow(cs).set(2000_u32);
        }

        // Obtain access to global timer
        let mut timer = G_TIM.borrow(cs).borrow_mut();

        // Adjust and start timer with updated delay value
        timer
            .as_mut()
            .unwrap()
            .start(G_DELAYMS.borrow(cs).get().millis())
            .unwrap();

        // Obtain access to Global Button Peripheral and Clear Interrupt Pending Flag
        let mut button = G_BUTTON.borrow(cs).borrow_mut();
        button.as_mut().unwrap().clear_interrupt_pending_bit();
    });
}

// Timer Interrupt
#[interrupt]
fn TIM2() {
    // When Timer Interrupt Happens Two Things Need to be Done
    // 1) Toggle the LED
    // 2) Clear Timer Pending Interrupt

    // Start a Critical Section
    cortex_m::interrupt::free(|cs| {
        // Obtain Access to Delay Global Data and Adjust Delay
        let mut led = G_LED.borrow(cs).borrow_mut();
        led.as_mut().unwrap().toggle();

        // Obtain access to Global Timer Peripheral and Clear Interrupt Pending Flag
        let mut timer = G_TIM.borrow(cs).borrow_mut();
        timer.as_mut().unwrap().clear_interrupt(Event::Update);
    });
}
