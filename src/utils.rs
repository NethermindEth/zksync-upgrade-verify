// convert bytes to hex string
pub fn bytes_to_hex_string(bytes: &[u8]) -> String {
    let hex_string: String = bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    hex_string
}

// add one to big number
// no overflow checks
pub fn add_one_to_big_number(number: &mut [u8]) {
    let mut carry = 1;

    for byte in number.iter_mut().rev() {
        let sum = *byte as u16 + carry;
        *byte = sum as u8;
        carry = sum / 256;

        if carry == 0 {
            break;
        }
    }
}

#[test]
fn test_bytes_to_hex_string() {
    // Test empty input
    assert_eq!(bytes_to_hex_string(&[]), "");
    // Test single byte
    assert_eq!(bytes_to_hex_string(&[0x12]), "12");
    // Test multiple bytes
    assert_eq!(bytes_to_hex_string(&[0xAB, 0xCD, 0xEF]), "abcdef");
    // Test zero padding
    assert_eq!(bytes_to_hex_string(&[0x00, 0x01, 0x02]), "000102");
}
#[test]
fn test_add_one_to_big_number() {
    // Test overflow
    let mut number = [0xFF];
    add_one_to_big_number(&mut number);
    assert_eq!(number, [0x00]);
    // Test adding 1 to multiple byte number
    let mut number = [0x12, 0x34, 0x56];
    add_one_to_big_number(&mut number);
    assert_eq!(number, [0x12, 0x34, 0x57]);
    // Test switch
    let mut number = [0xFE, 0xFF];
    add_one_to_big_number(&mut number);
    assert_eq!(number, [0xFF, 0x00]);
}