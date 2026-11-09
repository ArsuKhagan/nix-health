use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct BrokenSymlink {
    pub path: PathBuf,
    pub target: PathBuf,
}

pub fn scan_broken_symlinks<P: AsRef<Path>>(config_dir: P, _home_dir: &str) -> Vec<BrokenSymlink> {
    let mut broken = Vec::new();
    let config_path = config_dir.as_ref();

    if !config_path.exists() {
        return broken;
    }

    for entry in WalkDir::new(config_path)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        
        // Игнорируем файлы блокировки (lock), так как они ссылаются на сокеты/цепи процесса
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name == "lock" {
                continue;
            }
        }
        
        if path.is_symlink() {
            if let Ok(target) = std::fs::read_link(path) {
                if !target.exists() {
                    broken.push(BrokenSymlink {
                        path: path.to_path_buf(),
                        target,
                    });
                }
            }
        }
    }

    broken
}

pub fn sanitize_path(path: &Path, home: &str) -> String {
    let path_str = path.to_string_lossy();
    if path_str.starts_with(home) {
        path_str.replacen(home, "~", 1)
    } else {
        path_str.into_owned()
    }
}