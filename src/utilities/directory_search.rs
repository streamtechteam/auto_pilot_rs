use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
use walkdir::WalkDir;

pub fn search_directory(dir: &Path, pattern: &str) -> std::io::Result<()> {
    let regex = Regex::new(pattern)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(file);
                for (_, line) in reader.lines().enumerate() {
                    if let Ok(line) = line {
                        if regex.is_match(&line) {
                            println!(
                                "This job file has this id '{}' : {}",
                                pattern,
                                path.display()
                            );
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
