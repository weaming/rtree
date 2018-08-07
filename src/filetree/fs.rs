use std::path::Path;

pub fn is_dir(path: &String) -> bool {
    let p = Path::new(&path);
    p.is_dir()
}

pub fn is_file(path: &String) -> bool {
    let p = Path::new(&path);
    p.is_file()
}

pub fn exist_path(path: &String) -> bool {
    Path::new(path).exists()
}
