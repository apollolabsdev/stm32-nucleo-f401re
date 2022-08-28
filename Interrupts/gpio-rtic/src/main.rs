#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true)]
mod app {

    use stm32f4xx_hal::{
        gpio::{self, Edge, Input, Output, PushPull},
        pac::TIM1,
        prelude::*,
        timer,
    };

    // Resources shared between tasks
    #[shared]
    struct Shared {
        delayval: u32,
    }

    // Local resources to specific tasks (cannot be shared)
    #[local]
    struct Local {
        button: gpio::PC13<Input>,
        led: gpio::PA5<Output<PushPull>>,
        delay: timer::DelayMs<TIM1>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut dp = ctx.device;

        // Configure and obtain handle for delay abstraction
        // 1) Promote RCC structure to HAL to be able to configure clocks
        let rcc = dp.RCC.constrain();
        // 2) Configure the system clocks
        // 8 MHz must be used for HSE on the Nucleo-F401RE board according to manual
        let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
        // 3) Create delay handle
        let delay = dp.TIM1.delay_ms(&clocks);

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

        (
            // Initialization of shared resources
            Shared { delayval: 2000_u32 },
            // Initialization of task local resources
            Local { button, led, delay },
            // Move the monotonic timer to the RTIC run-time, this enables
            // scheduling
            init::Monotonics(),
        )
    }

    // Background task, runs whenever no other tasks are running
    #[idle(local = [led, delay], shared = [delayval])]
    fn idle(mut ctx: idle::Context) -> ! {
        let led = ctx.local.led;
        let delay = ctx.local.delay;
        loop {
            // Turn On LED
            led.set_high();
            // Obtain shared delay variable and delay
            delay.delay_ms(ctx.shared.delayval.lock(|del| *del));
            // Turn off LED
            led.set_low();
            // Obtain shared delay variable and delay
            delay.delay_ms(ctx.shared.delayval.lock(|del| *del));
        }
    }

    #[task(binds = EXTI15_10, local = [button], shared=[delayval])]
    fn button_pressed(mut ctx: button_pressed::Context) {
        ctx.shared.delayval.lock(|del| {
            *del = *del - 500_u32;
            if *del < 500_u32 {
                *del = 2000_u32;
            }
            *del
        });
        // Obtain access to Global Button Peripheral and Clear Interrupt Pending Flag
        ctx.local.button.clear_interrupt_pending_bit();
    }
}
