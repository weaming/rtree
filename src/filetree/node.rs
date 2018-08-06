use std::fs;
use std::path::Path;

const node_type_dir: String = "dir".to_owned();
const node_type_file: String = "file".to_owned();

#[derive(Debug)]
pub struct FileNode<'a> {
    pub name: String,
    pub extension: String,
    pub abs_path: String,
    pub rel_path: String,
    pub node_type: String,

    pub size: i64,
    pub total_size: i64,
    pub human_size: i64,

    pub parent: Option<&'a FileNode<'a>>,
    pub dirs: Vec<&'a FileNode<'a>>,
    pub files: Vec<&'a FileNode<'a>>,

    pub images: Vec<&'a FileNode<'a>>,
    pub children: Vec<&'a FileNode<'a>>,
}

pub fn new_file_node<'a>(path_str: str, root: str, parent: Option<&FileNode>) -> FileNode<'a> {
    path_str = Path::new(path_str).canonicalize();
    let path = Path::new(path_str);

    let mut rv = FileNode {
        name: path.file_name(),
        extension: path.extension(),
        abs_path: path_str,
        rel_path: path.strip_prefix(root).unwrap(),
        node_type: |path| -> str {
            if path.is_dir() {
                node_type_dir
            } else {
                node_type_file
            }
        },

        size: get_file_size(path_str),
        human_size: get_file_human_size(path_str),

        parent: parent,
        dirs: vec![],
        files: vec![],

        images: vec![],
        children: vec![],
    };

    if path.is_dir() {
        rv.size = 0;

        let files = fs::read_dir(path_str).unwrap();
        for f in files {
            let abs_path = path.join(f).to_str().unwrap();

            let child_file_node = new_file_node(abs_path, path_str, Some(&rv));

            if is_dir(abs_path) {
                rv.children.append(child_file_node);
                rv.dirs.append(child_file_node);
            } else if is_file(abs_path) {
                rv.children.append(child_file_node);
                rv.files.append(child_file_node);

                match child_file_node.extension {
                    "jpg" | "jpeg" | "png" | "gif" | "bmp" => rv.images.append(child_file_node),
                    _ => {}
                }
            }
        }
    }

    rv.total_size = rv.get_total_size();
    rv.children
        .sort_by(|a, b| a.total_size.cmp(b.total_size).unwrap());
    rv
}

impl<'a> FileNode<'a> {
    fn get_total_size(&self) -> f64 {
        if self.node_type == node_type_file {
            self.size
        } else {
            let rv = 0;
            for f in self.files {
                rv += f.get_total_size();
            }
            rv
        }
    }
}

pub fn get_file_size(path: str) -> u64 {
    let fi = fs::metadata(path).unwrap();
    fi.len()
}

pub fn get_file_human_size(path: str) -> u64 {
    let size = get_file_size(path);
    human_size(size)
}

pub fn human_size(mut s: i64, factor: i64) -> str {
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
    format!("{}{}", s, unit)
}

fn is_dir(path: str) -> bool {
    let p = Path::new(path);
    p.is_dir()
}

fn is_file(path: str) -> bool {
    let p = Path::new(path);
    p.is_file()
}
