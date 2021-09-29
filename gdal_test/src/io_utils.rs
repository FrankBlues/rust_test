use std::io;
use std::path::{Path, PathBuf};
use std::fs::{File, create_dir_all};
use std::io::prelude::*;
use std::io::BufWriter;
extern crate glob;
use glob::{glob, Paths};

/// get all files with the extension(.tif, .img, ..etc)
pub fn get_files(in_dir: &str, extension: &str) -> Result<Paths, Box<dyn std::error::Error>> {
    let mut path = PathBuf::from(in_dir);
    path.push("**/*".to_string() + extension);

    Ok(glob(path.to_str().unwrap()).expect("Failed to read glob pattern"))
}

pub fn check_parent_dir<T: AsRef<Path>>(file_name: &T) -> io::Result<()> {
    let path = file_name.as_ref();
    match path.parent() {
        Some(p) => {
            if !p.is_dir() {
                println!("Parent dir not exist, create");
                create_dir_all(p).expect("Call create_dir_all error");
            }
        },
        _ => println!("Cannot get parent dir of the input filename.")
    }
    Ok(())
}

/// Write string to a text file.
pub fn write_vec_to_text<'a, T: AsRef<Path> + ?Sized, U: Iterator<Item=&'a String>>(file_name: &T, content: U) -> io::Result<()> {
    
    check_parent_dir(&file_name).unwrap();
    
    match File::create(file_name) {
        Ok(f) => {
            {
                let mut writer = BufWriter::new(f);
                for t in content {
                    writer.write((t.to_owned() + "\n").as_bytes())?;
                }
            }
        },
        Err(e) => panic!("Problem creating the file: {:?}", e),
    }
    Ok(())
}