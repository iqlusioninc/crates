use super::Error;

/// Encode binary data as bech32-flavored base32
pub fn encode(data: &[u8]) -> Result<Vec<u8>, Error> {
    convert(data, 8, 5)
}

/// Decode data from bech32-flavored base32
pub fn decode(data: &[u8]) -> Result<Vec<u8>, Error> {
    convert(data, 5, 8)
}

fn convert(data: &[u8], src_base: u32, dst_base: u32) -> Result<Vec<u8>, Error> {
    let mut acc = 0u32;
    let mut bits = 0u32;
    let mut result = vec![];
    let max = (1u32 << dst_base) - 1;

    for value in data {
        let v = u32::from(*value);

        if (v >> src_base) != 0 {
            return Err(Error::DataInvalid { byte: v as u8 });
        }

        acc = (acc << src_base) | v;
        bits += src_base;

        while bits >= dst_base {
            bits -= dst_base;
            result.push(((acc >> bits) & max) as u8);
        }
    }

    if src_base > dst_base {
        if bits > 0 {
            result.push(((acc << (dst_base - bits)) & max) as u8);
        }
    } else if bits >= src_base || ((acc << (dst_base - bits)) & max) != 0 {
        return Err(Error::PaddingInvalid);
    }

    Ok(result)
}
