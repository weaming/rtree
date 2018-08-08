use std::fs;

pub fn get_file_size(path: &str) -> f64 {
    let fi = fs::metadata(path).unwrap();
    fi.len() as f64
}

pub fn get_file_human_size(path: &str) -> String {
    let size = get_file_size(path);
    // TODO: using 1000
    human_size(size, 1000f64)
}

pub fn human_size(mut s: f64, factor: f64) -> String {
    let mut unit = "B";
    if s > factor {
        s /= factor;
        unit = "KB";
    }
    if s > factor {
        s /= factor;
        unit = "MB";
    }
    if s > factor {
        s /= factor;
        unit = "GB";
    }
    if s > factor {
        s /= factor;
        unit = "TB";
    }
    format!("{:.2}{}", s, unit)
}
