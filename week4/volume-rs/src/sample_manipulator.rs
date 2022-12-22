pub fn apply_factor_to_8_bit_sample(sample: u8, factor: f64) -> usize {
    (sample as f64 * factor) as usize
}

pub fn apply_factor_to_16_bit_sample(sample: i16, factor: f64) -> isize {
    (sample as f64 * factor) as isize
}

pub fn truncate_8_bit_sample(sample_value: usize) -> u8 {
    if sample_value > u8::MAX.into() {
        u8::MAX
    } else if sample_value < u8::MIN.into() {
        u8::MIN
    } else {
        sample_value as u8
    }
}

pub fn truncate_16_bit_sample(sample_value: isize) -> i16 {
    if sample_value > i16::MAX.into() {
        i16::MAX
    } else if sample_value < i16::MIN.into() {
        i16::MIN
    } else {
        sample_value as i16
    }
}
