use polib::po_file::po_file_parser::POParseOptions;
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
    let parse_options = POParseOptions {
        message_body_only: true,
        translated_only: true,
    };
    let catalog = po_file::parse(src, &parse_options)?;
    mo_file::write(&catalog, dst)?;
    Ok(())
}

struct Job {
    src: PathBuf,
    dst: PathBuf,
}

fn common_prefix(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).take_while(|(c, d)| c == d).count()
}

fn common_suffix(a: &str, b: &str) -> usize {
    a.chars()
        .rev()
        .zip(b.chars().rev())
        .take_while(|(c, d)| c == d)
        .count()
}

fn main() {
    let (mut input, mut output) = match (env::args().nth(1), env::args().nth(2)) {
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
        if cfg!(windows) {
            input = input.replace('/', "\\");
            if input.starts_with(".\\") {
                input = input.strip_prefix(".\\").unwrap().to_string();
            }
            output = output.replace('/', "\\");
        }
        for entry in glob::glob(&input.replace("<lang>", "*")).unwrap() {
            if let Ok(src) = entry {
                let src = src.to_str().unwrap();
                let lang = &src[common_prefix(src, &input)..src.len() - common_suffix(src, &input)];
                let dst = output.replace("<lang>", lang);
                jobs.push(Job {
                    src: PathBuf::from(src),
                    dst: PathBuf::from(dst),
                });
            } else {
                println!("Warning: {}", entry.err().unwrap());
            }
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
