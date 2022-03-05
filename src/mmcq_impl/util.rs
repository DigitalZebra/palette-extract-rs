
use super::config::SIGNAL_BITS;

pub fn color_index_from(red: u8, green: u8, blue: u8) -> u32 {
    ((red as u32) << (2 * SIGNAL_BITS)) + ((green as u32) << SIGNAL_BITS) + (blue as u32)
}