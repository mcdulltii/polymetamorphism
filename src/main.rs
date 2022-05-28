#![allow(warnings)]
#![feature(asm)]

use std::arch::asm;
use std::env;
use std::ptr::{copy, read_volatile};
use std::mem::transmute;

use mmap::{MapOption, MemoryMap};

// modules
mod metamorphic;
mod polymorphic;

// key section
#[link_section = ".unixb"]
#[used]
static mut KEY: [u8; polymorphic::CRYPTED_FUNC_SIZE] = [0; polymorphic::CRYPTED_FUNC_SIZE];

// Payload function section
// #[link_section = ".reloc"]
// fn payload() {
//     unsafe {
//         asm!(
//             ".code32",
//             "xor eax, eax",
//             "mov al, 4",
//             "xor ebx, ebx",
//             "mov bl, 1",
//             "xor edx, edx",
//             "push edx",
//             "push 0x0a646c72",
//             "push 0x6f57206f",
//             "push 0x6c6c6548",
//             "mov ecx, esp",
//             "mov dl, 12",
//             "int 0x80",
//         );
//     }
// }

#[link_section = ".reloc"]
#[used]
// static FUNC: fn() = payload;
static FUNC: [u8; polymorphic::CRYPTED_FUNC_SIZE] = *b"\x6a\x01\x58\x50\x5f\xbe\x76\x69\x6c\x0a\x48\xc1\xc6\x08\x48\x83\xf6\x45\x56\x54\x5e\x6a\x05\x5a\x0f\x05\x6a\x3c\x58\x48\x31\xff\x0f\x05";
// static FUNC: [u8; polymorphic::CRYPTED_FUNC_SIZE] = *b"\x6a\x01\x58\x50\x5f\xbe\x76\x69\x6c\x0a\x48\xc1\xc6\x08\x48\x83\xf6\x45\x56\x54\x5e\x6a\x05\x5a\x0f\x05";

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[0];
    let mut code = Vec::new();

    // Read argv[0] into code variable
    metamorphic::read_binary_file(filename, &mut code).ok();

    // Check for ASM locations and randomize junk bytes
    metamorphic::metamorph(&mut code);

    // Initialize linked sections
    let mut key = unsafe { read_volatile(&KEY) };
    let func = unsafe { read_volatile(&FUNC) };

    // Decrypt payload function section
    let decrypted_func = polymorphic::decrypt_func(&mut code, &mut key).ok().unwrap();

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
        ).unwrap();

        // Copy decrypted payload function into memory region
        copy(decrypted_func.as_ptr(), decrypted_func_map.data(), decrypted_func.len());
        let decrypted_func_ptr: extern "C" fn() -> ! = transmute(decrypted_func_map.data());

        // Run decrypted payload function
        decrypted_func_ptr();
    }

    // Re-encrypt payload function section with new random key
    polymorphic::encrypt_func(&mut code, decrypted_func, &mut key).ok();

    // Rewrite binary file
    metamorphic::write_binary_file(filename, &mut code).ok();
}
