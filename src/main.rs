use memmap2::MmapOptions;
use object::{File, Object, ObjectSection};
use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};

#[link_section = "count"]
#[used]
static mut RUN_COUNT: u32 = 0;

fn get_section(file: &File, name: &str) -> Option<(u64, u64)> {
    for section in file.sections() {
        match section.name() {
            Ok(n) if n == name => {
                return section.file_range();
            }
            _ => {}
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let run_count = unsafe { RUN_COUNT };
    println!("Previous run count: {}", run_count);
    let exe = env::current_exe()?;
    let tmp = exe.with_extension("tmp");
    fs::copy(&exe, &tmp)?;

    let file = OpenOptions::new().read(true).write(true).open(&tmp)?;
    let mut buf = unsafe { MmapOptions::new().map_mut(&file) }?;
    let file = File::parse(&*buf)?;

    if let Some(range) = get_section(&file, "count") {
        assert_eq!(range.1, 4);
        let base = range.0 as usize;
        buf[base..(base + 4)].copy_from_slice(&(run_count + 1).to_ne_bytes());

        let perms = fs::metadata(&exe)?.permissions();
        fs::set_permissions(&tmp, perms)?;
        fs::rename(&tmp, &exe)?;
    } else {
        fs::remove_file(&tmp)?;
    }

    Ok(())
}
