use libsodium_sys as libsodium;
use rosenpass_to::{with_destination, To};
use std::ffi::c_ulonglong;
use std::ptr::null;
use thiserror::Error;
use log::{error, info};

// A custom error type
#[derive(Error, Debug)]
pub enum HashError {
    #[error("Failed to hash data")]
    HashFailed,
}

pub const KEY_MIN: usize = libsodium::crypto_generichash_blake2b_KEYBYTES_MIN as usize;
pub const KEY_MAX: usize = libsodium::crypto_generichash_blake2b_KEYBYTES_MAX as usize;
pub const OUT_MIN: usize = libsodium::crypto_generichash_blake2b_BYTES_MIN as usize;
pub const OUT_MAX: usize = libsodium::crypto_generichash_blake2b_BYTES_MAX as usize;

#[inline]
pub fn hash<'a>(key: &'a [u8], data: &'a [u8]) -> impl To<[u8], Result<(), HashError>> + 'a {
    with_destination(|out: &mut [u8]| {
        assert!(key.is_empty() || (KEY_MIN <= key.len() && key.len() <= KEY_MAX));
        assert!(OUT_MIN <= out.len() && out.len() <= OUT_MAX);
        let kptr = match key.len() {
            // NULL key
            0 => null(),
            _ => key.as_ptr(),
        };
        
        // Replaced the anyhow::Result with Result and log errors
        if let Err(err) = sodium_call!(
            crypto_generichash_blake2b,
            out.as_mut_ptr(),
            out.len(),
            data.as_ptr(),
            data.len() as c_ulonglong,
            kptr,
            key.len()
        ) {
            error!("Hashing failed: {:?}", err);
            return Err(HashError::HashFailed);
        }

        // Log successful hash
        info!("Data hashed successfully");
        Ok(())
    })
}
