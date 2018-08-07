use std::fs;
use std::path::Path;

const node_type_dir: String = String::from("dir");
const node_type_file: String = String::from("file");

#[derive(Debug)]
pub struct FileNode<'a> {
    pub name: String,
    pub extension: String,
    pub abs_path: String,
    pub rel_path: String,
    pub node_type: String,

    pub size: f64,
    pub total_size: f64,
    pub human_size: String,

    pub parent: Option<&'a FileNode<'a>>,
    pub dirs: Vec<&'a FileNode<'a>>,
    pub files: Vec<&'a FileNode<'a>>,

    pub images: Vec<&'a FileNode<'a>>,
    pub children: Vec<&'a FileNode<'a>>,
}

pub fn new_file_node<'a>(
    path_str: String,
    root: String,
    parent: Option<&FileNode>,
) -> FileNode<'a> {
    path_str = Path::new(&path_str)
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let path = Path::new(&path_str);

    let mut rv = FileNode {
        name: path.file_name().unwrap().to_str().unwrap().to_owned(),
        extension: path.extension().unwrap().to_str().unwrap().to_owned(),
        abs_path: path_str,
        rel_path: path
            .strip_prefix(root)
            .unwrap()
            .to_owned()
            .to_str()
            .unwrap()
            .to_owned(),
        node_type: if path.is_dir() {
            node_type_dir
        } else {
            node_type_file
        },

        size: get_file_size(&path_str),
        human_size: get_file_human_size(&path_str),
        total_size: 0f64,

        parent: parent,
        dirs: vec![],
        files: vec![],

        images: vec![],
        children: vec![],
    };

    if path.is_dir() {
        rv.size = 0f64;

        let files = fs::read_dir(path_str).unwrap();
        for f in files {
            let abs_path = path.join(f.unwrap().file_name()).to_str().unwrap();

            let child_file_node = new_file_node(String::from(abs_path), path_str, Some(&rv));

            if is_dir(abs_path) {
                rv.children.append(&mut child_file_node);
                rv.dirs.append(child_file_node);
            } else if is_file(abs_path) {
                rv.children.append(child_file_node);
                rv.files.append(child_file_node);

                match &*child_file_node.extension {
                    "jpg" | "jpeg" | "png" | "gif" | "bmp" => rv.images.append(child_file_node),
                    _ => {}
                }
            }
        }
    }

    rv.total_size = rv.get_total_size();
    rv.children
        .sort_by(|a, b| a.total_size.partial_cmp(&b.total_size).unwrap());
    rv
}

impl<'a> FileNode<'a> {
    fn get_total_size(&self) -> f64 {
        if self.node_type == node_type_file {
            self.size
        } else {
            let rv = 0f64;
            for f in self.files {
                rv += f.get_total_size();
            }
            rv
        }
    }
}

pub fn get_file_size(path: &str) -> f64 {
    let fi = fs::metadata(path).unwrap();
    fi.len() as f64
}

pub fn get_file_human_size(path: &str) -> String {
    let size = get_file_size(path);
    human_size(size, 1024f64)
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
    format!("{}{}", s, unit)
}

fn is_dir(path: &str) -> bool {
    let p = Path::new(path);
    p.is_dir()
}

fn is_file(path: &str) -> bool {
    let p = Path::new(path);
    p.is_file()
}
