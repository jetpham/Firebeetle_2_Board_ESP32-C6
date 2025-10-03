use core::f32::consts::PI;

pub fn wave_animation(time_ms: u32) -> [u8; 8] {
    let cycle_duration_ms = 1000;
    let angle = (time_ms as f32 / cycle_duration_ms as f32) * 2.0 * PI;
    
    let mut leds = [0u8; 8];
    leds.iter_mut().enumerate().for_each(|(i, led)| {
        let phase_offset = (i as f32) * (2.0 * PI) / 8.0;
        let led_angle = angle + phase_offset;
        let sine_value = libm::sinf(led_angle);
        let brightness = (sine_value + 1.0) / 2.0;
        *led = (brightness * 100.0) as u8;
    });
    
    leds
}
