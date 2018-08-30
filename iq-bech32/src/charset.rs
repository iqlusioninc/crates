/// Default separator character
pub const DEFAULT_SEPARATOR: char = '1';

/// bech32 encoding character set
pub const CHARSET: [char; 32] = [
    'q', 'p', 'z', 'r', 'y', '9', 'x', '8', 'g', 'f', '2', 't', 'v', 'd', 'w', '0', 's', '3', 'j',
    'n', '5', '4', 'k', 'h', 'c', 'e', '6', 'm', 'u', 'a', '7', 'l',
];

lazy_static! {
    pub(crate) static ref CHARSET_INVERSE: [Option<u8>; 128] = {
        let mut inverse = [None; 128];

        for (i, char) in CHARSET.iter().enumerate() {
            let mut byte = [0u8];

            {
                let inv_str = char.encode_utf8(byte.as_mut());
                inverse[inv_str.to_uppercase().as_bytes()[0] as usize] = Some(i as u8);
            }

            inverse[byte[0] as usize] = Some(i as u8);
        }

        inverse
    };
}
