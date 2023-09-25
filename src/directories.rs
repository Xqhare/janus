use std::os::unix::fs::{DirEntryExt2, MetadataExt};
use std::fs;
use std::path::PathBuf;
use std::env;

fn get_dir_entries(read_dir_path: &str) -> Result<Vec<fs::DirEntry>, std::io::Error> {
    let mut dir_entries = vec![];
    for dir_entry in fs::read_dir(read_dir_path)? {
        let dir_entry = dir_entry?;
        dir_entries.push(dir_entry);
    }
    Ok(dir_entries)
}

pub fn current_dir() -> std::io::Result<PathBuf> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());
    Ok(path)
}

pub fn print_everything(dir_path: &str) {
    let dir_entries = get_dir_entries(&dir_path);
        match dir_entries {
            Ok(dir_entries) => {
                for dir_entry in dir_entries {
                    // FILENAME
                    let dir_entry_file_name = dir_entry.file_name();
                    println!("FILE NAME: {:?}", dir_entry_file_name);
                    println!("FILE NAME REF: {:?}", dir_entry.file_name_ref());
                    // PATH
                    let dir_entry_path = dir_entry.path();
                    println!("PATH: {:?}", dir_entry_path);
                    // GET METADATA
                    let dir_entry_metadata = dir_entry.metadata().unwrap();
                    println!("METADATA: {:?}", dir_entry_metadata);
                    // GET  PERMISSIONS
                    let dir_entry_permissions = dir_entry_metadata.permissions();
                    println!("dir_entry_permissions: {:?}", dir_entry_permissions);
                    // GET IS_DIR BOOLEAN
                    let dir_entry_is_dir_bool = dir_entry_metadata.is_dir();
                    println!("dir_entry_is_dir_bool: {:?}", dir_entry_is_dir_bool);
                    // GET IS_SYMLINK BOOL
                    let dir_entry_is_symlink_bool = dir_entry_metadata.is_symlink();
                    println!("dir_entry_is_symlink_bool: {:?}", dir_entry_is_symlink_bool);
                    // GET IS_FILE BOOL
                    let dir_entry_is_file_bool = dir_entry_metadata.is_file();
                    println!("dir_entry_is_file_bool: {:?}", dir_entry_is_file_bool);
                    // GET METADATA FILETYPE
                    let dir_entry_meta_filetype = dir_entry_metadata.file_type();
                    println!("dir_entry_meta_filetype: {:?}", dir_entry_meta_filetype);
                    // GET Unix MODE
                    let dir_entry_mode = dir_entry_metadata.mode();
                    println!("dir_entry_mode: {:?}", dir_entry_mode);
                    // GET Byte LEN of file
                    let dir_entry_byte_len = dir_entry_metadata.len();
                    println!("dir_entry_len: {:?}", dir_entry_byte_len);
                    // GET ACCESSED TIME
                    let dir_entry_accessed = dir_entry_metadata.accessed();
                    println!("dir_entry_accessed: {:?}", dir_entry_accessed);
                    // GET CREATED TIME
                    let dir_entry_created = dir_entry_metadata.created();
                    println!("dir_entry_created: {:?}", dir_entry_created);
                    // GET MODIFIED TIME
                    let dir_entry_modified = dir_entry_metadata.modified();
                    println!("dir_entry_modified: {:?}", dir_entry_modified);


                    // GET FILETYPE
                    let dir_entry_filetype = dir_entry.file_type().unwrap();
                    println!("FILE TYPE: {:?}", dir_entry_filetype);
                }
            }
            // Error Handling
            _ => {}
        }
}