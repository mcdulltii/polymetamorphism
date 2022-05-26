#![allow(warnings)]
#![feature(asm)]

use std::arch::asm;
use object::{File, Object, ObjectSection};
use std::env;
use std::error::Error;

mod engine;

#[link_section = ".reloc"]
#[used]
static mut RUN_COUNT: u8 = 0;

fn get_section(file: &File, name: &str) -> Option<(u64, u64)> {
    for section in file.sections() {
        engine::junk!();
        match section.name() {
            Ok(n) if n == name => {
                return section.file_range();
            }
            _ => {}
        }
        engine::junk!();
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let run_count = unsafe { RUN_COUNT }; engine::junk!();
    println!("Previous run count: {}", run_count);

    let args: Vec<String> = env::args().collect(); engine::junk!();
    let filename = &args[0];
    let mut code = Vec::new(); engine::junk!();

    engine::read_binary_file(filename, &mut code).ok(); engine::junk!();

    engine::metamorph(&mut code); engine::junk!();

    let file = File::parse(&*code)?; engine::junk!();

    if let Some(range) = get_section(&file, ".reloc") {
        assert_eq!(range.1, 1);
        let base = range.0 as usize;
        code[base..(base + 1)].copy_from_slice(&(run_count + 1).to_ne_bytes()); engine::junk!();
    }

    engine::write_binary_file(filename, &mut code).ok(); engine::junk!();

    Ok(())
}
