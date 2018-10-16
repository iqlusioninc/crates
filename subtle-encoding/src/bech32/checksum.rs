use zeroize::Zeroize;

use self::polymod::Polymod;
use super::Error;

/// Size of checksum in bytes
pub const CHECKSUM_SIZE: usize = 6;

/// Checksum value used to verify data integrity
pub(crate) struct Checksum([u8; CHECKSUM_SIZE]);

impl Checksum {
    /// Create a checksum for the given human-readable part (hrp) and binary data
    pub fn new(hrp: &[u8], data: &[u8]) -> Self {
        let mut p = Polymod::compute(hrp, data);
        let mut checksum = [0u8; CHECKSUM_SIZE];
        p.input_slice(&checksum);

        let value = p.finish() ^ 1;
        for (i, byte) in checksum.iter_mut().enumerate().take(CHECKSUM_SIZE) {
            *byte = ((value >> (5 * (5 - i))) & 0x1f) as u8;
        }

        Checksum(checksum)
    }

    /// Verify this checksum matches the given human-readable part (hrp) and binary data
    pub fn verify(hrp: &[u8], data: &[u8]) -> Result<(), Error> {
        if Polymod::compute(hrp, data).finish() == 1 {
            Ok(())
        } else {
            Err(Error::ChecksumInvalid)
        }
    }
}

impl AsRef<[u8]> for Checksum {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Drop for Checksum {
    fn drop(&mut self) {
        self.0.as_mut().zeroize()
    }
}

mod polymod {
    /// bech32 generator coefficients
    const COEFFICIENTS: [u32; 5] = [
        0x3b6a_57b2,
        0x2650_8e6d,
        0x1ea1_19fa,
        0x3d42_33dd,
        0x2a14_62b3,
    ];

    /// Perform polynomial calculation against the given generator coefficients
    pub(crate) struct Polymod(u32);

    impl Default for Polymod {
        fn default() -> Polymod {
            Polymod(1)
        }
    }

    impl Polymod {
        pub fn compute(hrp: &[u8], data: &[u8]) -> Self {
            let mut p = Polymod::default();

            for b in hrp {
                p.input_byte(*b >> 5);
            }

            p.input_byte(0);

            for b in hrp {
                p.input_byte(*b & 0x1f);
            }

            p.input_slice(data);
            p
        }

        pub fn input_slice(&mut self, slice: &[u8]) {
            for b in slice {
                self.input_byte(*b)
            }
        }

        pub fn input_byte(&mut self, byte: u8) {
            let b = (self.0 >> 25) as u8;
            self.0 = (self.0 & 0x1ff_ffff) << 5 ^ u32::from(byte);

            for (i, c) in COEFFICIENTS.iter().enumerate() {
                if (b >> i) & 1 == 1 {
                    self.0 ^= *c
                }
            }
        }

        pub fn finish(self) -> u32 {
            self.0
        }
    }

    impl Drop for Polymod {
        fn drop(&mut self) {
            // TODO: secure zeroize (integers not yet supported by `zeroize` crate)
            self.0 = 0;
        }
    }
}
