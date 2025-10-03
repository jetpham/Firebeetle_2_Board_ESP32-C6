pub type AnimationFunction = fn(u32) -> [u8; 6];

pub mod wave;
pub mod surge;
pub mod ping;
pub mod binary;
pub mod tyler;

pub use wave::wave_animation;
pub use surge::surge_animation;
pub use ping::ping_animation;
pub use binary::binary_animation;
pub use tyler::tyler_animation;
