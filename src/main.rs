use std::{env, path::Path, process};

use copy_checker::{FileCheckRecorder, copy_check};

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];
    if args.len() < 3 || args.len() > 4 {
        eprintln!("Usage: {program} <source> <dest> [record_path]");
        process::exit(1);
    }

    let source_root_path = Path::new(&args[1]);
    let dest_root_path = Path::new(&args[2]);

    if !Path::exists(source_root_path) {
        eprintln!("Source path no exist!");
        process::exit(1);
    } else if !Path::is_dir(source_root_path) {
        eprintln!("Source path no a directory!");
        process::exit(2);
    }

    if !Path::exists(dest_root_path) {
        eprintln!("Destination path no exist!");
        process::exit(3);
    } else if !Path::is_dir(dest_root_path) {
        eprintln!("Destination path no a directory!");
        process::exit(4);
    }

    let record_file_path = match &args.get(3) {
        Some(value) => value,
        None => "./result.csv",
    };

    let mut recorder = FileCheckRecorder::new(record_file_path)
        .expect(&format!("Cannot open record file: {record_file_path}")[..]);

    copy_check(source_root_path, dest_root_path, &mut recorder);
    
}

