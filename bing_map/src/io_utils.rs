use std::path::{Path, PathBuf};
use::std::io;
use std::collections::HashMap;

pub fn get_files(tile_dir: &Path) -> Result<HashMap<PathBuf, Vec<PathBuf>>, io::Error> {
    let mut tile_files = HashMap::new();
    for entry in tile_dir.read_dir().expect("read_dir fail") {
        if let Ok(entry) = entry {
            let cur_path = entry.path();
            if cur_path.is_dir() {
                let mut entries = cur_path.read_dir()?
                    .map(|res| res.map(|e| e.path()))
                    .collect::<Result<Vec<_>, io::Error>>()?;
                entries.sort();
                tile_files.insert(cur_path, entries);
            }
        }
    }
    Ok(tile_files)
}