use polib::{mo_file, po_file};
use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let (input, output) = match (env::args().nth(1), env::args().nth(2)) {
        (Some(input), Some(output)) => (input, output),
        _ => {
            println!("Usage: compile-po2mo <input.po> <output.mo>");
            return Ok(());
        }
    };
    let mut directory = PathBuf::from(&output);
    directory.pop();
    std::fs::create_dir_all(directory)?;
    let catalog = po_file::parse(Path::new(&input))?;
    mo_file::write(&catalog, Path::new(&output))?;
    Ok(())
}
