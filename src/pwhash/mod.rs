mod argon2;

use argon2rs::ParamErr;
use ::utils::Bytes;
pub use self::argon2::{
    Argon2i,
    OPSLIMIT_INTERACTIVE, MEMLIMIT_INTERACTIVE,
    OPSLIMIT_MODERATE, MEMLIMIT_MODERATE,
    OPSLIMIT_SENSITIVE, MEMLIMIT_SENSITIVE
};


pub type Key = Bytes;

pub trait KeyDerive: Default {
    fn new() -> Self {
        Self::default()
    }

    fn with_key(mut self, key: &[u8]) -> Self;
    fn with_aad(mut self, aad: &[u8]) -> Self;
    fn with_opslimit(mut self, opslimit: u32) -> Self;
    fn with_memlimit(mut self, memlimit: u32) -> Self;
    fn derive(&self, password: &[u8], salt: &[u8], outlen: usize) -> Result<Key, ParamErr>;
}

pub trait KeyVerify: KeyDerive {
    fn verify(&self, password: &[u8], salt: &[u8], hash: &[u8]) -> Result<bool, ParamErr>;
}
