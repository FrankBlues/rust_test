use ::std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn get_files(tile_dir: &Path) -> Result<HashMap<PathBuf, Vec<PathBuf>>, io::Error> {
    let mut tile_files = HashMap::new();
    for entry in tile_dir.read_dir().expect("read_dir fail") {
        if let Ok(entry) = entry {
            let cur_path = entry.path();
            if cur_path.is_dir() {
                let mut entries = cur_path
                    .read_dir()?
                    .map(|res| res.map(|e| e.path()))
                    .collect::<Result<Vec<_>, io::Error>>()?;
                entries.sort();
                tile_files.insert(cur_path, entries);
            }
        }
    }
    Ok(tile_files)
}

pub fn write_string_to_text<T: AsRef<Path>>(file_name: &T, content: String) -> io::Result<()> {
    let f = File::create(file_name)?;
    {
        let mut writer = BufWriter::new(f);
        writer.write(content.as_bytes())?;
    }
    Ok(())
}
    
