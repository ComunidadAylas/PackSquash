use super::*;

#[test]
fn decode_hex_works() {
	assert_eq!(decode_hex("48656c6c6f").unwrap(), b"Hello");
	assert_eq!(&decode_hex("").unwrap(), &[0_u8; 0]);
	assert_eq!(&decode_hex("00").unwrap(), &[0]);
	assert_eq!(&decode_hex("ff").unwrap(), &[255]);
	assert_eq!(&decode_hex("DeadBeef").unwrap(), &[222, 173, 190, 239]);

	assert!(decode_hex("hello").is_none());
	assert!(decode_hex("1").is_none());
}
