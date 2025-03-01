use static_assertions::const_assert;

pub mod subtle;

pub const KEY_LEN: usize = 32;
const_assert!(KEY_LEN == aead::KEY_LEN);
const_assert!(KEY_LEN == xaead::KEY_LEN);
const_assert!(KEY_LEN == hash::KEY_LEN);

/// Authenticated encryption with associated data
pub mod aead {
    pub use rosenpass_sodium::aead::chacha20poly1305_ietf::{
        decrypt, encrypt, KEY_LEN, NONCE_LEN, TAG_LEN,
    };
}

/// Authenticated encryption with associated data with a constant nonce
pub mod xaead {
    pub use rosenpass_sodium::aead::xchacha20poly1305_ietf::{
        decrypt, encrypt, KEY_LEN, NONCE_LEN, TAG_LEN,
    };
}

pub mod hash {
    pub use crate::subtle::incorrect_hmac_blake2b::{
        hash, KEY_LEN, KEY_MAX, KEY_MIN, OUT_MAX, OUT_MIN,
    };
}
