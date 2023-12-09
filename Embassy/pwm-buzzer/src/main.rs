#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use cortex_m_rt::entry;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
// use embassy_stm32::pwm::Channel;
use embassy_stm32::time::hz;
use embassy_time::Delay;
use embedded_hal::blocking::delay::DelayMs;

use panic_halt as _;

#[entry]
fn main() -> ! {
    // Initialize and create handle for devicer peripherals
    let p = embassy_stm32::init(Default::default());

    // Configure the Buzzer pin as an alternate and obtain handler.
    // I will use PA9 that connects to Grove shield connector D8
    // On the Nucleo FR401 PA9 connects to timer TIM1

    // Instantiate PWM pin and connect to channel
    let buzz_pin = PwmPin::new_ch2(p.PA9);

    // Instantiate and Configure Timer 1 for PWM
    let mut pwm = SimplePwm::new(p.TIM1, None, Some(buzz_pin), None, None, hz(2000));

    // Get Maximum Duty
    let max_duty = pwm.get_max_duty();

    // Configure the Duty Cycle to 50%
    pwm.set_duty(Channel::Ch2, max_duty / 2);

    // Enable PWM
    pwm.enable(Channel::Ch2);

    // // Configure and create a handle for a second timer using TIM2 for delay puposes
    // let mut delay = Delay.delay_ms(100);

    // Define the notes and their frequencies
    let tones = [
        ('c', hz(261)),
        ('d', hz(294)),
        ('e', hz(329)),
        ('f', hz(349)),
        ('g', hz(392)),
        ('a', hz(440)),
        ('b', hz(493)),
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
                    pwm.set_freq(tone.1);
                    // 3.2 Enable the channel to generate desired PWM
                    pwm.enable(Channel::Ch2);
                    // 3.3 Keep the output on for as long as required
                    Delay.delay_ms(note.1 * tempo);
                } else if note.0 == ' ' {
                    // 2.2 if ' ' tone is found disable output for one beat
                    pwm.disable(Channel::Ch2);
                    Delay.delay_ms(tempo);
                }
            }
            // 4. Silence for half a beat between notes
            // 4.1 Disable the PWM output (silence)
            pwm.disable(Channel::Ch2);
            // 4.2 Keep the output off for half a beat between notes
            Delay.delay_ms(tempo / 2);
            // 5. Go back to 1.
        }
    }
}
