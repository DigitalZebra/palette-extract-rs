pub static SIGNAL_BITS: u8 = 5;
pub static RIGHT_SHIFT: u8 = 8 - SIGNAL_BITS;
pub static MULTIPLIER: u8 = 1 << RIGHT_SHIFT;
pub static HISTOGRAM_SIZE: u16 = 1 << (3 * SIGNAL_BITS);
pub static VBOX_LENGTH: u8 = 1 << SIGNAL_BITS;
pub static FRACTION_BY_POPULATION: f32 = 0.75;
pub static MAX_ITERATIONS: u16 = 1000;
