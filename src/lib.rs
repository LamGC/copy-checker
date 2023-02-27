use std::{fs::{OpenOptions, File}, io::{Write, stdout, Read}, fmt::{self, Display}, path::Path};

use sha2::{Sha256, Digest};
use walkdir::WalkDir;

pub enum FileCheckResult {
    OK,
    NotFound,
    NoEqual,
    Error
}

impl Display for FileCheckResult {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileCheckResult::OK => write!(f, "OK"),
            FileCheckResult::NotFound => write!(f, "NotFound"),
            FileCheckResult::NoEqual => write!(f, "NoEqual"),
            FileCheckResult::Error => write!(f, "Error"),
        }
    }

}

pub struct FileCheckRecorder {
    file: File
}

impl FileCheckRecorder {
    pub fn new(file_path: &str) -> Result<FileCheckRecorder, std::io::Error> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)?;

        Ok(FileCheckRecorder { file })
    }

    pub fn fail(&mut self, path: &str, result: FileCheckResult) {
        match self.file.write_all(format!("{result},\"{path}\",null,null\r\n").as_bytes()) {
            Ok(_) => {
                if let Err(e) = self.file.flush() {
                    eprintln!("Cannot flush record: {e}")
                }
            },
            Err(e) => {
                eprintln!("Cannot append record: {e}")
            },
        }
    }

    pub fn append(&mut self, path: &str, result: FileCheckResult, source_digest: String, dest_digest: String) {
        match self.file.write(format!("{result},\"{path}\",{source_digest},{dest_digest}\r\n").as_bytes()) {
            Ok(_) => {
                if let Err(e) = self.file.flush() {
                    eprintln!("Cannot flush record: {e}")
                }
            },
            Err(e) => {
                eprintln!("Cannot append record: {e}")
            }
        }
    }

}

pub fn copy_check(source_root_path: &Path, dest_root_path: &Path, recorder: &mut FileCheckRecorder) {
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
