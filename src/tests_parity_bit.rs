pub use super::parity_bit::add_parity_bit;
pub use super::parity_bit::compute_parity_bit;
pub use super::parity_bit::has_parity;
pub use super::parity_bit::remove_parity_bit;


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


#[test]
#[should_panic(expected="Cannot use number exceeding 7 bits")]
fn test_add_parity_bit_out_of_range() {
    add_parity_bit(0b10000000);
}


#[test]
fn test_add_parity_bit() {
    assert_eq!(add_parity_bit(0b00000000), 0b00000000);
    assert_eq!(add_parity_bit(0b00000001), 0b00000011);
    assert_eq!(add_parity_bit(0b00000011), 0b00000110);
    assert_eq!(add_parity_bit(0b00000010), 0b00000101);
    assert_eq!(add_parity_bit(0b00000110), 0b00001100);
    assert_eq!(add_parity_bit(0b00001110), 0b00011101);
    assert_eq!(add_parity_bit(0b00011110), 0b00111100);
    assert_eq!(add_parity_bit(0b00111110), 0b01111101);
    assert_eq!(add_parity_bit(0b01111110), 0b11111100);
    assert_eq!(add_parity_bit(0b01111111), 0b11111111);
}


#[test]
fn test_remove_parity_bit() {
    assert_eq!(remove_parity_bit(0b10000000), 0b01000000);
}


#[test]
fn test_add_remove_roundtrip() {
    let mut num = 0;

    // Make sure 0-127 roundtrips correctly
    loop {
        // We're done here
        if num > 127 {
            break;
        }

        let num8 = add_parity_bit(num);
        let num7 = remove_parity_bit(num8);

        assert_eq!(num, num7);

        // Print the ascii char
        println!("id: {},  char: '{}'", num, (num as char));

        num = num + 1;
    }
}


#[test]
fn test_has_parity() {
    assert_eq!(has_parity(0b00000000), true);
    assert_eq!(has_parity(0b00000001), false);
    assert_eq!(has_parity(0b00000011), true);
    assert_eq!(has_parity(0b00000010), false);
    assert_eq!(has_parity(0b00000110), true);
    assert_eq!(has_parity(0b00001110), false);
    assert_eq!(has_parity(0b00011110), true);
    assert_eq!(has_parity(0b00111110), false);
    assert_eq!(has_parity(0b01111110), true);
    assert_eq!(has_parity(0b01111111), false);
}
