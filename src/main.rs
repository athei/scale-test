use parity_scale_codec::Encode;

fn main() {
    let data_array = {
        let data: ([u8; 4], u128) = (
            [0x84, 0xa1, 0x5d, 0xa1],
            5000,
        );
        data.encode()
    };
    let data_slice = {
        let data: (&[u8], u128) = (
            &[0x84, 0xa1, 0x5d, 0xa1],
            5000,
        );
        data.encode()
    };
    let data_manual = {
        let mut arg_enc: Vec<u8> = 5000u128.encode();
        let mut data = Vec::new();
        data.extend_from_slice(&[0x84u8, 0xa1, 0x5d, 0xa1]);
        data.append(&mut arg_enc);
        data
    };

    // array encoding is identity
    assert_eq!(data_array, data_manual);

    // slice will prepend its length
    assert_ne!(data_slice, data_manual)
}
