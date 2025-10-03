#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::ledc::{
    channel::{self, config::PinConfig, ChannelIFace},
    timer::{self, config::Duty, LSClockSource, TimerIFace},
    LSGlobalClkSource, Ledc, LowSpeed,
};
use esp_hal::main;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use panic_rtt_target as _;

extern crate alloc;

use core::f32::consts::PI;

esp_bootloader_esp_idf::esp_app_desc!();

fn calculate_sine_brightness(angle: f32) -> f32 {
    let sine_value = libm::sinf(angle);
    (sine_value + 1.0) / 2.0
}

#[main]
fn main() -> ! {

    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(timg0.timer0, esp_hal::rng::Rng::new(peripherals.RNG)).unwrap();

    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: Duty::Duty8Bit,
            clock_source: LSClockSource::APBClk,
            frequency: Rate::from_hz(1000),
        })
        .unwrap();

    let mut channels = [
        ledc.channel(channel::Number::Channel0, peripherals.GPIO1),
        ledc.channel(channel::Number::Channel1, peripherals.GPIO18),
        ledc.channel(channel::Number::Channel2, peripherals.GPIO9),
        ledc.channel(channel::Number::Channel3, peripherals.GPIO19),
        ledc.channel(channel::Number::Channel4, peripherals.GPIO20),
        ledc.channel(channel::Number::Channel5, peripherals.GPIO21),
    ];

    for channel in channels.iter_mut() {
        channel
            .configure(channel::config::Config {
                timer: &lstimer0,
                duty_pct: 0,
                pin_config: PinConfig::PushPull,
            })
            .unwrap();
    }

    let delay = Delay::new();

    let mut angle: f32 = 0.0;
    let cycle_duration_ms = 1000;
    let steps_per_cycle = (cycle_duration_ms / 20) as u32;
    let angle_step = (2.0 * PI) / steps_per_cycle as f32;

    info!("Starting smooth sine wave LED brightness control (1 second cycle)...");

    loop {
        for (i, channel) in channels.iter_mut().enumerate() {
            let phase_offset = (i as f32) * (2.0 * PI) / 6.0;
            let led_angle = angle + phase_offset;

            let brightness = calculate_sine_brightness(led_angle);
            let duty_pct = (brightness * 100.0) as u8;

            channel.set_duty(duty_pct).unwrap();

            if i == 0 {
                info!("Base angle: {}, LED 0 brightness: {}%", angle, duty_pct);
            }
        }

        delay.delay_millis(20);

        angle += angle_step;
        if angle >= 2.0 * PI {
            angle = 0.0;
        }
    }

}
