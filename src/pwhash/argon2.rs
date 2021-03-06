//! [argon2](https://password-hashing.net/).

use argon2rs::{ Argon2, Variant, ParamErr };
use seckey::Bytes;
use super::{ KeyDerive, KeyDerivationFail };


/// Interactive Opslimit. parameter from `libsodium`.
pub const OPSLIMIT_INTERACTIVE: u32 = 4;
/// Interactive Memlimit. parameter from `libsodium`.
pub const MEMLIMIT_INTERACTIVE: u32 = 33554432;
/// Moderate Opslimit. parameter from `libsodium`.
pub const OPSLIMIT_MODERATE: u32 = 6;
/// Moderate Memlimit. parameter from `libsodium`.
pub const MEMLIMIT_MODERATE: u32 = 134217728;
/// Sensitive Opslimit. parameter from `libsodium`.
pub const OPSLIMIT_SENSITIVE: u32 = 8;
/// Sensitive Memlimit. parameter from `libsodium`.
pub const MEMLIMIT_SENSITIVE: u32 = 536870912;

/// Argon2i.
///
/// # Example(keyderive)
/// ```
/// # extern crate rand;
/// # extern crate seckey;
/// # extern crate sarkara;
/// # fn main() {
/// use rand::{ Rng, thread_rng };
/// use seckey::Bytes;
/// use sarkara::pwhash::{ Argon2i, KeyDerive };
///
/// // ...
/// # let mut rng = thread_rng();
/// # let mut pass = [0; 8];
/// # let mut salt = [0; 8];
/// # rng.fill_bytes(&mut pass);
/// # rng.fill_bytes(&mut salt);
///
/// let key = Argon2i::default()
///     .derive::<Bytes>(&pass, &salt)
///     .unwrap();
/// # assert!(key != pass);
/// # }
/// ```
///
/// # Example(keyverify)
/// ```
/// # extern crate rand;
/// # extern crate seckey;
/// # extern crate sarkara;
/// # fn main() {
/// use rand::{ Rng, thread_rng };
/// use seckey::Bytes;
/// use sarkara::pwhash::{ Argon2i, KeyDerive, KeyVerify };
///
/// // ...
/// # let mut rng = thread_rng();
/// # let mut pass = [0; 8];
/// # let mut salt = [0; 8];
/// # rng.fill_bytes(&mut pass);
/// # rng.fill_bytes(&mut salt);
///
/// let key = Argon2i::default()
///     .derive::<Bytes>(&pass, &salt)
///     .unwrap();
///
/// assert!(Argon2i::default().verify(&pass, &salt, &key).unwrap());
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Argon2i {
    key: Bytes,
    aad: Bytes,
    outlen: usize,
    passes: u32,
    lanes: u32,
    kib: u32
}

impl Default for Argon2i {
    fn default() -> Argon2i {
        Argon2i {
            key: Bytes::empty(),
            aad: Bytes::empty(),
            outlen: 32,
            passes: OPSLIMIT_INTERACTIVE,
            lanes: 1,
            kib: MEMLIMIT_INTERACTIVE / 1024
        }
    }
}

impl KeyDerive for Argon2i {
    fn with_size(&mut self, len: usize) -> &mut Self {
        self.outlen = len;
        self
    }
    fn with_key(&mut self, key: &[u8]) -> &mut Self {
        self.key = Bytes::new(key);
        self
    }
    fn with_aad(&mut self, aad: &[u8]) -> &mut Self {
        self.aad = Bytes::new(aad);
        self
    }
    fn with_opslimit(&mut self, opslimit: u32) -> &mut Self {
        self.passes = opslimit;
        self
    }
    fn with_memlimit(&mut self, memlimit: u32) -> &mut Self {
        self.kib = memlimit / 1024;
        self
    }

    fn derive<K>(&self, password: &[u8], salt: &[u8])
        -> Result<K, KeyDerivationFail>
        where K: From<Vec<u8>>
    {
        if salt.len() < 8 { Err(KeyDerivationFail::SaltTooShort)? };
        if salt.len() > 0xffffffff { Err(KeyDerivationFail::SaltTooLong)? };
        if self.outlen < 4 { Err(KeyDerivationFail::OutLenTooShort)? };
        if self.outlen > 0xffffffff { Err(KeyDerivationFail::OutLenTooLong)? };

        let mut output = vec![0; self.outlen];
        Argon2::new(self.passes, self.lanes, self.kib, Variant::Argon2i)?
            .hash(&mut output, password, salt, &self.key, &self.aad);
        Ok(output.into())
    }
}

impl From<ParamErr> for KeyDerivationFail {
    fn from(err: ParamErr) -> KeyDerivationFail {
        use std::error::Error;
        KeyDerivationFail::ParameterError(err.description().to_string())
    }
}
