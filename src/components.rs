use std::{error::Error, fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub file_type: FileType,
}

#[derive(Debug, Clone)]
pub enum FileType {
    Dir,
    File,
}

pub fn get_current_dir() -> Result<PathBuf, Box<dyn Error>> {
    std::env::current_dir().map_err(|e| e.into())
}

/// NOTE: if current dir is the very root dir, then it will ruturn itslef
pub fn get_parent_dir(path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let path_buf = PathBuf::from(path);
    if let Some(parent_dir) = path_buf.parent() {
        Ok(parent_dir.to_path_buf())
    } else {
        Ok(path_buf.to_path_buf())
    }
}
pub fn get_files(path: &str) -> Result<Vec<File>, Box<dyn Error>> {
    let mut files = Vec::new();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                map_file(&mut files, file)?;
            }
        }
    }

    sort_files(&mut files);
    Ok(files)
}

pub fn filter_hidden_files(files: Vec<File>) -> Vec<File> {
    files
        .into_iter()
        .filter(|f| !f.name.starts_with('.'))
        .collect()
}

fn sort_files(files: &mut Vec<File>) {
    files.sort_by(|a, b| match (&a.file_type, &b.file_type) {
        (FileType::Dir, FileType::File) => std::cmp::Ordering::Less,
        (FileType::File, FileType::Dir) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });
}

fn map_file(files: &mut Vec<File>, file: fs::DirEntry) -> Result<(), Box<dyn Error>> {
    let metadata = fs::metadata(file.path())?;
    Ok(if metadata.is_dir() {
        files.push(File {
            name: file
                .file_name()
                .into_string()
                .unwrap_or("Unknown dir".to_string()),
            file_type: FileType::Dir,
        });
    } else {
        files.push(File {
            name: file
                .file_name()
                .into_string()
                .unwrap_or("Unknown file".to_string()),
            file_type: FileType::File,
        });
    })
}
