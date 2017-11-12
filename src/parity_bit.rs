/// Computes an even parity bit from a 7bit number [0, 127] and returns
/// the bit.
pub fn compute_parity_bit(num: u8) -> i8 {
    // Don't accept numbers that exceed 7 bits
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


/// Alternative implementation of compute_parity_bit that can be heavily
/// optimized on x86.
pub fn compute_parity_bit_opt(num: u8) -> i8 {
    // Don't accept numbers that exceed 7 bits
    if num > 127 {
        return -1;
    }

    // use the built-in function count_ones ones which can take advantage of
    // the dedicate x86 instruction popcnt:
    // https://users.rust-lang.org/t/what-is-the-implementation-of-count-ones/4923/3
    return (num.count_ones() % 2) as i8;
}


/// Accepts a 7bit number and adds a parity bit at the end (shifting the
/// bits forward), returning the new 8bit number.
pub fn add_parity_bit(num: u8) -> u8 {
    let parity_bit = compute_parity_bit(num);

    // Make sure the bit is not an error
    if parity_bit == -1 {
        panic!("Cannot use number exceeding 7 bits");
    }

    let result = (num << 1) | (parity_bit as u8);

    return result;
}


/// Accepts an 8bit number and removes the parity bit (8th bit),
/// returning the other 7 bits of the number.
pub fn remove_parity_bit(num: u8) -> u8 {
    return num >> 1;
}


/// Accepts an 8bit number where the last bit is a parity bit, and checks
/// the first 7 bits to see if the parity bit is correct (which means
/// data integrity is preserved). Returns a bool.
pub fn has_parity(num: u8) -> bool {
    // the transmitted parity bit (8th bit)
    let actual_parity_bit = num & 1;

    // the computed parity bit based on the first 7 bits
    let parity_bit = compute_parity_bit(num >> 1) as u8;

    return actual_parity_bit == parity_bit;
}
