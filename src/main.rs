use std::{env, fs::File, io::Read, path::Path, process, io::stdout, io::Write};

use copy_checker::{FileCheckRecorder, FileCheckResult};
use hex;
use pathdiff;
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

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
    let walkdir = WalkDir::new(source_root_path)
        .sort_by_file_name()
        .follow_links(true)
        .into_iter();

    println!("Scanning source path...");
    for entry in walkdir {
        match entry {
            Ok(entry) => {
                if !entry.path().is_file() {
                    continue;
                }
                let r_path = pathdiff::diff_paths(entry.path(), source_root_path)
                    .expect("Cannot get relative path.");
                print!("Checking `{}`......", r_path.display());
                stdout().flush().expect("Cannot flush stdout.");
                let dest_file_path = dest_root_path.join(&r_path);
                if !Path::exists(&dest_file_path) {
                    println!("Not exists!");
                    recorder.fail(
                        &r_path.to_str().expect("Cannot get path str."),
                        FileCheckResult::NotFound,
                    );
                    continue;
                }

                let source_digest =
                    get_file_digest(entry.path()).expect("Failed to calculate source file digest.");
                let dest_digest = get_file_digest(&dest_file_path)
                    .expect("Failed to calculate destination file digest.");

                let result = if source_digest == dest_digest {
                    println!("OK!");
                    FileCheckResult::OK
                } else {
                    println!("Not equal!");
                    FileCheckResult::NoEqual
                };
                stdout().flush().expect("Cannot flush stdout.");
                
                let source_digest = hex::encode(source_digest);
                let dest_digest = hex::encode(dest_digest);

                let r_path = &r_path.to_str().expect("Cannot get path str.");

                recorder.append(
                    &r_path,
                    result,
                    source_digest,
                    dest_digest,
                );
            }
            Err(e) => {
                eprintln!("Cannot access file: {e}");
            }
        }
    }
    println!("Checks complete!");
}

fn get_file_digest(path: &Path) -> Result<[u8; 32], std::io::Error> {
    let mut file = File::open(path)?;
    const BUFFER_SIZE: usize = 1024 * 100; // 10K
    let mut buffer = [0; BUFFER_SIZE];
    let mut digest = Sha256::new();
    loop {
        let read_size = file.read(&mut buffer)?;
        if read_size == 0 {
            break;
        }
        digest.update(&buffer[..read_size]);
        if read_size != BUFFER_SIZE {
            break;
        }
    }
    let mut digest_result: [u8; 32] = [0; 32];
    digest_result.copy_from_slice(digest.finalize().as_slice());
    Ok(digest_result)
}
