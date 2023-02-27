use std::{fs::{OpenOptions, File}, io::Write, fmt::{self, Display}};

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



