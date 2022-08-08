#![no_std]
#![no_main]

// Imports
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
    timer::Channel,
};

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Set up the clocks
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    // Configure the Buzzer pin as an alternate and obtain handler.
    // I will use PA9 that connects to Grove shield connector D8
    // On the Nucleo FR401 PA9 connects to timer TIM1
    let gpioa = dp.GPIOA.split();
    let buzz = gpioa.pa9.into_alternate();
    let mut buzz_pwm = dp.TIM1.pwm_hz(buzz, 2000.Hz(), &clocks);

    // Configure the duty cycle to 50%
    // If duty not configured, PWM will not operate properly (suggest comments)
    let max_duty = buzz_pwm.get_max_duty();
    buzz_pwm.set_duty(Channel::C2, max_duty / 2);

    // Configure and create a handle for a second timer using TIM2 for delay puposes
    let mut delay = dp.TIM2.delay_ms(&clocks);

    // Define the notes and their frequencies
    let tones = [
        ('c', 261.Hz()),
        ('d', 294.Hz()),
        ('e', 329.Hz()),
        ('f', 349.Hz()),
        ('g', 392.Hz()),
        ('a', 440.Hz()),
        ('b', 493.Hz()),
    ];

    // Define the notes to be played and the beats per note
    let tune = [
        ('c', 1),
        ('c', 1),
        ('g', 1),
        ('g', 1),
        ('a', 1),
        ('a', 1),
        ('g', 2),
        ('f', 1),
        ('f', 1),
        ('e', 1),
        ('e', 1),
        ('d', 1),
        ('d', 1),
        ('c', 2),
        (' ', 4),
    ];

    // Define the tempo
    let tempo = 300_u32;

    // Application Loop
    loop {
        // 1. Obtain a note in the tune
        for note in tune {
            // 2. Retrieve the freqeuncy and beat associated with the note
            for tone in tones {
                // 2.1 Find a note match in the tones array and update frequency and beat variables accordingly
                if tone.0 == note.0 {
                    // 3. Play the note for the desired duration (beats*tempo)
                    // 3.1 Adjust period of the PWM output to match the new frequency
                    buzz_pwm.set_period(tone.1);
                    // 3.2 Enable the channel to generate desired PWM
                    buzz_pwm.enable(Channel::C2);
                    // 3.3 Keep the output on for as long as required
                    delay.delay_ms(note.1 * tempo);
                } else if note.0 == ' ' {
                    // 2.2 if ' ' tone is found disable output for one beat
                    buzz_pwm.disable(Channel::C2);
                    delay.delay_ms(tempo);
                }
            }
            // 4. Silence for half a beat between notes
            // 4.1 Disable the PWM output (silence)
            buzz_pwm.disable(Channel::C2);
            // 4.2 Keep the output off for half a beat between notes
            delay.delay_ms(tempo / 2);
            // 5. Go back to 1.
        }
    }
}
