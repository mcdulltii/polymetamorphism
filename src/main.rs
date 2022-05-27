#![allow(warnings)]
#![feature(asm)]

use std::arch::asm;
use std::env;
use std::mem::transmute;

// modules
mod metamorphic;
mod polymorphic;

// key section
#[link_section = ".unixb"]
#[used]
static mut KEY: [u8; polymorphic::CRYPTED_FUNC_SIZE] = [0; polymorphic::CRYPTED_FUNC_SIZE];

// Payload function section
#[link_section = ".reloc"]
pub fn payload() {
    println!("Hello World!");
}

#[link_section = ".reloc"]
#[used]
static FUNC: fn() = payload;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[0];
    let mut code = Vec::new();

    // Read argv[0] into code variable
    metamorphic::read_binary_file(filename, &mut code).ok();

    // Check for ASM locations and randomize junk bytes
    metamorphic::metamorph(&mut code);

    // Initialize linked sections
    let mut key = unsafe { KEY };
    let func = unsafe { FUNC };

    // Decrypt payload function section
    let decrypted_func = polymorphic::decrypt_func(&mut code, &mut key).ok().unwrap();
    let func_ptr = decrypted_func.as_ptr();
    let decrypted_func_ptr: fn() = unsafe { transmute(func_ptr) };

    // Run decrypted payload function
    // decrypted_func_ptr();

    // Re-encrypt payload function section with new random key
    polymorphic::encrypt_func(&mut code, decrypted_func, &mut key).ok();

    // Rewrite binary file
    metamorphic::write_binary_file(filename, &mut code).ok();
}
