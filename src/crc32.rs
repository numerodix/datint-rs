/// Theory: http://chrisballance.com/wp-content/uploads/2015/10/CRC-Primer.html
/// Reference impl: http://www.sunshine2k.de/coding/javascript/crc/crc_js.html
///
/// CRC: polynomial division (xor in binary) of the input by the preselected
/// polynomial.


/// Computes CRC32 remainder, characteristics:
/// - input/output not reflected
/// - no initial value used
/// - no final value xor'ed with result
pub fn compute_crc32_remainder(bits: u32) -> u32 {
    let polynomial = 0x04C11DB7;

    let mut dividend = bits;

    for _i in 0..32 {
        if (dividend & (1 << 31)) > 0 {
            dividend = dividend ^ polynomial;
        }

        dividend = dividend << 1;
    }

    return dividend >> 1;
}


/// Computes CRC4 remainder to illustrate the method
pub fn compute_crc4_remainder_demo() -> u8 {
    let bits = 0b00001001;
    let polynomial = 0b00001011;

    let mut dividend = bits;

    for _i in 0..4 {
        // if the first bit is one we xor with polynomial
        if (dividend & (1 << 3)) > 0 {
            dividend = dividend ^ polynomial;
        }

        // then we shift left by 1
        dividend = dividend << 1;
    }

    // go back one because it's 3 bits long
    return dividend >> 1;

/*
 * Unrolled version
 *
    // first bit is 1
    dividend = dividend ^ poly;
    dividend = dividend << 1;

    // first bit is 0
    dividend = dividend ^ 0;
    dividend = dividend << 1;

    // first bit is 1
    dividend = dividend ^ poly;
    dividend = dividend << 1;

    // first bit is 0
    dividend = dividend ^ 0;
    dividend = dividend << 1;

    return dividend >> 1;
*/
}
