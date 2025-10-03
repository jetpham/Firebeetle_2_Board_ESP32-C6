pub fn ping_animation(time_ms: u32) -> [u8; 8] {
    let cycle_duration_ms = 1000;
    let cycle_position = (time_ms % cycle_duration_ms) as f32 / cycle_duration_ms as f32;
    
    let mut leds = [0u8; 8];
    
    let led_index = if cycle_position < 8.0 / 14.0 {
        (cycle_position * 14.0) as usize
    } else {
        (14.0 - (cycle_position * 14.0)) as usize
    };
    
    if led_index < 8 {
        leds[led_index] = 100;
    }
    
    leds
}
