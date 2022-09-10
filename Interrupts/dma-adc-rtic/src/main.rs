#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [])]
mod app {

    use core::fmt::Write;
    use stm32f4xx_hal::{
        adc::{
            config::{AdcConfig, Clock, Dma, Resolution, SampleTime, Scan, Sequence},
            Adc,
        },
        dma::{config::DmaConfig, PeripheralToMemory, Stream0, StreamsTuple, Transfer},
        pac::{self, ADC1, DMA2, TIM2, USART2},
        prelude::*,
        serial::{config::Config, Tx},
        timer::{CounterHz, Event},
    };

    type DMATransfer =
        Transfer<Stream0<DMA2>, 0, Adc<ADC1>, PeripheralToMemory, &'static mut [u16; 2]>;

    #[shared]
    struct Shared {
        transfer: DMATransfer,
    }

    #[local]
    struct Local {
        tx: Tx<USART2>,
        buffer: Option<&'static mut [u16; 2]>,
        timer: CounterHz<TIM2>,
    }

    #[init(local = [first_buffer: [u16; 2] = [0; 2],second_buffer: [u16; 2] = [0; 2]])]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let dp: pac::Peripherals = cx.device;

        // Clock Configuration
        let rcc = dp.RCC.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(84.MHz())
            .hclk(84.MHz())
            .require_pll48clk()
            .pclk2(21.MHz())
            .freeze();

        // ADC Configuration
        // Configure the first microphone pin into analog and obtain handler.
        // Split port A
        let gpioa = dp.GPIOA.split();
        // PA0 maps to A0 on Grove Base Shield
        // PA4 maps to A2 on Grove Base Shield
        let mic1 = gpioa.pa0.into_analog();
        let mic2 = gpioa.pa4.into_analog();
        // Create Handler for adc peripheral (PA0 and PA4 are connected to ADC1)
        // Configure ADC for sequence conversion with interrtups
        let adc_config = AdcConfig::default()
            .dma(Dma::Continuous)
            //Scan mode is also required to convert a sequence
            .scan(Scan::Enabled)
            .resolution(Resolution::Ten)
            .clock(Clock::Pclk2_div_8);

        let mut adc = Adc::adc1(dp.ADC1, true, adc_config);
        adc.configure_channel(&mic1, Sequence::One, SampleTime::Cycles_480);
        adc.configure_channel(&mic2, Sequence::Two, SampleTime::Cycles_480);

        // DMA Configuration
        let dma = StreamsTuple::new(dp.DMA2);
        let dma_config = DmaConfig::default()
            .transfer_complete_interrupt(true)
            .memory_increment(true)
            .double_buffer(false);

        let transfer = Transfer::init_peripheral_to_memory(
            dma.0,
            adc,
            cx.local.first_buffer,
            None,
            dma_config,
        );

        // Serial Configuration
        // Configure/Define TX pin
        // Use PA2 as it is connected to the host serial interface
        // let gpioa = dp.GPIOA.split();
        let tx_pin = gpioa.pa2.into_alternate();
        // Configure Serial perihperal channel
        // We're going to use USART2 since its pins are the ones connected to the USART host interface
        let tx = dp
            .USART2
            .tx(
                tx_pin,
                Config::default()
                    .baudrate(115200.bps())
                    .wordlength_8()
                    .parity_none(),
                &clocks,
            )
            .unwrap();

        let mut timer = dp.TIM2.counter_hz(&clocks);
        timer.listen(Event::Update);
        timer.start(1000.Hz()).unwrap();

        (
            Shared { transfer },
            Local {
                tx,
                buffer: Some(cx.local.second_buffer),
                timer,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = TIM2, shared = [transfer], local = [timer])]
    fn adcstart(mut cx: adcstart::Context) {
        cx.shared.transfer.lock(|transfer| {
            transfer.start(|adc| {
                adc.start_conversion();
            });
        });
        cx.local.timer.clear_interrupt(Event::Update);
    }

    #[task(binds = DMA2_STREAM0, shared = [transfer], local = [tx, buffer])]
    fn dma(ctx: dma::Context) {
        // Destructure dma::Context to make only the shared resources mutable
        //let dma::Context { mut shared, local } = cx;

        // Also Equivalent to
        let mut shared = ctx.shared;
        let local = ctx.local;

        let buffer = shared.transfer.lock(|transfer| {
            let (buffer, _) = transfer
                .next_transfer(local.buffer.take().unwrap())
                .unwrap();
            buffer
        });

        // Read out values from buffer
        let mic1 = buffer[0];
        let mic2 = buffer[1];

        // Return buffer to resources pool for next transfer
        *local.buffer = Some(buffer);

        // Send over data to UART
        writeln!(local.tx, "/*{:01},{:02}*/\r", mic1, mic2).unwrap();
    }
}
