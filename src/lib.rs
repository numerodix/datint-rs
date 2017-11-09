/// Computes an even parity bit from a 7bit number [0, 127] and returns
/// the bit.
fn compute_parity_bit(num: u8) -> i8 {
    if num > 127 {
        return -1;
    }

    let parity = (
        ((num & (1 << 6)) >> 6) ^
        ((num & (1 << 5)) >> 5) ^
        ((num & (1 << 4)) >> 4) ^
        ((num & (1 << 3)) >> 3) ^
        ((num & (1 << 2)) >> 2) ^
        ((num & (1 << 1)) >> 1) ^
        ((num & (1 << 0)) >> 0)
    );

    return parity as i8;
}


#[cfg(test)]
mod tests {
    use ::compute_parity_bit;

    #[test]
    fn test_compute_parity_bit() {
        // out of range
        assert_eq!(compute_parity_bit(0b10000000), -1);

        assert_eq!(compute_parity_bit(0b00000000), 0);
        assert_eq!(compute_parity_bit(0b00000001), 1);
        assert_eq!(compute_parity_bit(0b00000011), 0);
        assert_eq!(compute_parity_bit(0b00000010), 1);
        assert_eq!(compute_parity_bit(0b00000110), 0);
        assert_eq!(compute_parity_bit(0b00001110), 1);
        assert_eq!(compute_parity_bit(0b00011110), 0);
        assert_eq!(compute_parity_bit(0b00111110), 1);
        assert_eq!(compute_parity_bit(0b01111110), 0);
        assert_eq!(compute_parity_bit(0b01111111), 1);
    }
}
