pub fn binary_animation(time_ms: u32) -> [u8; 6] {
    let increment_interval_ms = 1000.0 / 64.0;
    let count = (time_ms as f32 / increment_interval_ms) as u32 % 64;
    
    let mut leds = [0u8; 6];
    (0..6).for_each(|i| {
        if count & (1 << i) != 0 {
            leds[5-i] = 100;
        }
    });
    
    leds
}
