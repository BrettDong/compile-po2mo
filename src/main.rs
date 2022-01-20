use polib::{mo_file, po_file};
use rayon::prelude::*;
use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

fn parent_directory(path: &Path) -> PathBuf {
    let mut path = path.to_path_buf();
    path.pop();
    path
}

fn ensure_dir_exist(path: &Path) -> Result<(), std::io::Error> {
    if path.exists() && path.is_file() {
        std::fs::create_dir_all(parent_directory(path))
    } else {
        std::fs::create_dir_all(path)
    }
}

fn compile(src: &Path, dst: &Path) -> Result<(), Box<dyn Error>> {
    ensure_dir_exist(&parent_directory(dst))?;
    let catalog = po_file::parse(src)?;
    mo_file::write(&catalog, dst)?;
    Ok(())
}

struct Job {
    src: PathBuf,
    dst: PathBuf,
}

fn main() {
    let (input, output) = match (env::args().nth(1), env::args().nth(2)) {
        (Some(input), Some(output)) => (input, output),
        _ => {
            println!("Usage: compile-po2mo <input.po> <output.mo>");
            std::process::exit(1);
        }
    };
    if input.contains('*') || output.contains('*') || input.contains('?') || output.contains('?') {
        println!("Path patterns (*, ?, etc.) are not allowed!");
        std::process::exit(1);
    }
    let mut jobs = vec![];
    if input.contains("<lang>") && output.contains("<lang>") {
        if let Some((a, b)) = input.split_once("<lang>") {
            let pattern = format!("{}*{}", &a, &b);
            println!(
                "Ensure {:?}",
                &PathBuf::from(output.split_once("<lang>").unwrap().0)
            );
            if let Err(err) =
                ensure_dir_exist(&PathBuf::from(output.split_once("<lang>").unwrap().0))
            {
                println!("Cannot create target directory: {}", err);
                std::process::exit(1);
            }
            for entry in glob::glob(&pattern).unwrap() {
                if let Ok(src) = entry {
                    let mut lang = src.to_str().unwrap();
                    lang = lang.strip_prefix(a).unwrap();
                    lang = lang.strip_suffix(b).unwrap();
                    let dst = PathBuf::from(output.replace("<lang>", lang));
                    jobs.push(Job { src, dst });
                } else if entry.is_err() {
                    let err = entry.err().unwrap();
                    println!(
                        "Warning: {} in accessing {}",
                        err.error(),
                        err.path().display()
                    );
                }
            }
        } else {
            println!("Invalid po path glob pattern!");
            std::process::exit(1);
        }
    } else if !input.contains("<lang>") && !output.contains("<lang>") {
        jobs.push(Job {
            src: PathBuf::from(input),
            dst: PathBuf::from(output),
        });
    } else {
        println!("Invalid po path glob pattern!");
        std::process::exit(1);
    }
    let start = std::time::Instant::now();
    let errors: Vec<_> = jobs
        .par_iter()
        .map(|job| {
            println!("Compiling: {} => {}", job.src.display(), job.dst.display());
            let result = compile(&job.src, &job.dst);
            if result.is_err() {
                Err(format!(
                    "Error: {} in compiling {}",
                    result.err().unwrap(),
                    job.src.display()
                ))
            } else {
                Ok(())
            }
        })
        .filter_map(|ret| ret.err())
        .collect();
    if errors.is_empty() {
        println!(
            "Successfully compiled {} files in {:05.3} seconds.",
            jobs.len(),
            start.elapsed().as_secs_f32()
        );
        std::process::exit(0);
    } else {
        for err in errors {
            println!("{}", err);
        }
        std::process::exit(1);
    }
}
