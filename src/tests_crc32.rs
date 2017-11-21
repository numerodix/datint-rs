pub use super::crc32::compute_crc4_remainder_demo;
pub use super::crc32::compute_crc32_remainder;


#[test]
fn test_compute_crc4_demo() {
    let crc4_rem = compute_crc4_remainder_demo();
    println!("crc4 remainder: {:b}", crc4_rem);
}


#[test]
fn test_compute_crc32() {
    assert_eq!(compute_crc32_remainder(0), 0);
    assert_eq!(compute_crc32_remainder(1), 0x04c11db7);
    assert_eq!(compute_crc32_remainder(2), 0x09823b6e);
    assert_eq!(compute_crc32_remainder(3), 0x0d4326d9);
    assert_eq!(compute_crc32_remainder(4), 0x130476dc);
    assert_eq!(compute_crc32_remainder(5), 0x17c56b6b);
    assert_eq!(compute_crc32_remainder(6), 0x1a864db2);
    assert_eq!(compute_crc32_remainder(7), 0x1e475005);

    assert_eq!(compute_crc32_remainder(8), 0x2608edb8);
    assert_eq!(compute_crc32_remainder(15), 0x384fbdbd);

    assert_eq!(compute_crc32_remainder(23), 0x52568b75);

    assert_eq!(compute_crc32_remainder(31), 0x745e66cd);
}
