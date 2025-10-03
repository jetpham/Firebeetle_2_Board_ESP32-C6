#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output, OutputConfig},
    main,
    time::{Duration, Instant},
    timer::timg::TimerGroup,
};
use panic_rtt_target as _;

extern crate alloc;

use firebeetle_2_board_esp32_c6::animations::{
    binary::binary_animation, ping::ping_animation, surge::surge_animation, wave::wave_animation,
};

esp_bootloader_esp_idf::esp_app_desc!();
struct SoftwarePWM {
    duty_cycles: [u16; 8],
    pwm_period: u16,
    current_step: u16,
}

impl SoftwarePWM {
    fn new() -> Self {
        Self {
            duty_cycles: [0; 8],
            pwm_period: 2000,
            current_step: 0,
        }
    }

    fn set_duty_cycles(&mut self, duty_cycles: [u8; 8]) {
        for (i, &duty) in duty_cycles.iter().enumerate() {
            self.duty_cycles[i] = (duty as u16 * 20).min(2000);
        }
    }

    fn should_be_high(&self, led_index: usize) -> bool {
        self.current_step < self.duty_cycles[led_index]
    }

    fn update_step(&mut self) {
        self.current_step = (self.current_step + 1) % self.pwm_period;
    }
}

#[main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(timg0.timer0, esp_hal::rng::Rng::new(peripherals.RNG)).unwrap();

    let mut leds = [
        Output::new(peripherals.GPIO1, Level::Low, OutputConfig::default()),
        Output::new(peripherals.GPIO18, Level::Low, OutputConfig::default()),
        Output::new(peripherals.GPIO9, Level::Low, OutputConfig::default()),
        Output::new(peripherals.GPIO19, Level::Low, OutputConfig::default()),
        Output::new(peripherals.GPIO20, Level::Low, OutputConfig::default()),
        Output::new(peripherals.GPIO21, Level::Low, OutputConfig::default()),
        Output::new(peripherals.GPIO22, Level::Low, OutputConfig::default()),
        Output::new(peripherals.GPIO23, Level::Low, OutputConfig::default()),
    ];

    let mut pwm = SoftwarePWM::new();
    let animations = [
        wave_animation,
        surge_animation,
        binary_animation,
        ping_animation,
    ];
    let mut current_animation_index = 0;
    let animation_switch_interval_ms = 3000;

    let mut animation_start = Instant::now();

    loop {
        let current_animation = animations[current_animation_index];

        let animation_elapsed_ms = animation_start.elapsed().as_millis() as u32;
        let led_values = current_animation(animation_elapsed_ms);

        pwm.set_duty_cycles(led_values);

        let pwm_cycles = 100;
        for _ in 0..pwm_cycles {
            for i in 0..8 {
                if pwm.should_be_high(i) {
                    leds[i].set_high();
                } else {
                    leds[i].set_low();
                }
            }

            pwm.update_step();

            let delay_start = Instant::now();
            while delay_start.elapsed() < Duration::from_micros(5) {}
        }

        if animation_elapsed_ms >= animation_switch_interval_ms {
            animation_start = Instant::now();
            current_animation_index = (current_animation_index + 1) % animations.len();
        }
    }
}
