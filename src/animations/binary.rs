pub fn binary_animation(time_ms: u32) -> [u8; 8] {
    let increment_interval_ms = 3000 / 256;
    let count = (time_ms / increment_interval_ms) % 256;

    let mut leds = [0u8; 8];
    (0..8).for_each(|i| {
        if count & (1 << i) != 0 {
            leds[7-i] = 100;
        }
    });
    
    leds
}
