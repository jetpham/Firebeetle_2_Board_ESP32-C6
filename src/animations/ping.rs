pub fn ping_animation(time_ms: u32) -> [u8; 6] {
    let cycle_duration_ms = 1000;
    let cycle_position = (time_ms % cycle_duration_ms) as f32 / cycle_duration_ms as f32;
    
    let mut leds = [0u8; 6];
    
    let led_index = if cycle_position < 6.0 / 10.0 {
        (cycle_position * 10.0) as usize
    } else {
        (10.0 - (cycle_position * 10.0)) as usize
    };
    
    // Ensure we don't go out of bounds
    if led_index < 6 {
        leds[led_index] = 100;
    }
    
    leds
}
