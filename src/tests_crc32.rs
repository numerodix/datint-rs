pub use super::crc32::compute_crc4_remainder_demo;
pub use super::crc32::compute_crc32_remainder;


#[test]
fn test_compute_crc32() {
    let crc4_rem = compute_crc4_remainder_demo();
    println!("crc4 remainder: {:b}", crc4_rem);

    let crc32_rem = compute_crc32_remainder(1);
    println!("crc32 remainder: {:x}", crc32_rem);
}
