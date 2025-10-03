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

use firebeetle_2_board_esp32_c6::animations::{
    wave::wave_animation,
    surge::surge_animation,
    ping::ping_animation,
    binary::binary_animation,
    tyler::tyler_animation
};

esp_bootloader_esp_idf::esp_app_desc!();


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

    channels.iter_mut().for_each(|channel| {
        channel
            .configure(channel::config::Config {
                timer: &lstimer0,
                duty_pct: 0,
                pin_config: PinConfig::PushPull,
            })
            .unwrap();
    });

    let delay = Delay::new();
    let animations = [wave_animation, surge_animation, binary_animation, ping_animation, tyler_animation];
    let mut current_animation_index = 0;
    let mut animation_start_time = 0u32;
    let animation_switch_interval_ms = 3000;
    let frame_delay_ms = 10;

    loop {
        let current_animation = animations[current_animation_index];
        let led_values = current_animation(animation_start_time);
        info!("LED values: {:?}", led_values);
        
        channels.iter_mut()
            .zip(led_values.iter())
            .for_each(|(channel, &duty_pct)| {
                channel.set_duty(duty_pct).unwrap();
            });

        delay.delay_millis(frame_delay_ms);
        animation_start_time += frame_delay_ms;
        
        if animation_start_time >= animation_switch_interval_ms {
            animation_start_time = 0;
            current_animation_index = (current_animation_index + 1) % animations.len();
            info!("Switching to animation {}", current_animation_index);
        }
    }

}
