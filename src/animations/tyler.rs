use core::f32::consts::PI;
use defmt::info;

pub fn tyler_animation(time_ms: u32) -> [u8; 6] {
    let cycle_duration_ms = 3000;
    let angle = (time_ms as f32 / cycle_duration_ms as f32) * 2.0 * PI;
    
    let mut leds = [0u8; 6];
    leds.iter_mut().enumerate().for_each(|(i, led)| {
        let sine_value =  (1..i+2).fold(1.0, |acc, i| {
            let b = i as f32;
            acc * libm::sinf(b * angle + (b * 0.5)) - 0.1
        });
        info!("{:?}", sine_value);
        let brightness = sine_value.max(0.0);
        *led = (brightness * 100.0) as u8;
    });
    
    leds
}
