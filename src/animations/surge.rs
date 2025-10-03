use core::f32::consts::PI;

pub fn surge_animation(time_ms: u32) -> [u8; 6] {
    let cycle_duration_ms = 2000;
    let angle = (time_ms as f32 / cycle_duration_ms as f32) * 2.0 * PI;
    let sine_value = libm::sinf(angle);
    let brightness = (sine_value + 1.0) / 2.0;
    let duty = (brightness * 100.0) as u8;
    [duty; 6]
}
