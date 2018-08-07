use std::cell::{Cell, RefCell};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use super::fs::{exist_path, is_dir, is_file};
use super::size::{get_file_human_size, get_file_size};

pub static NODE_TYPE_DIR: &'static str = "dir";
pub static NODE_TYPE_FILE: &'static str = "file";

#[derive(Debug)]
pub struct FileNode {
    pub name: String,
    pub extension: String,
    pub abs_path: String,
    pub rel_path: String,
    pub node_type: String,

    pub size: Cell<f64>,
    pub total_size: Cell<f64>,
    pub human_size: RefCell<String>,

    pub parent: Option<Arc<FileNode>>,
    pub dirs: RefCell<Vec<Arc<FileNode>>>,
    pub files: RefCell<Vec<Arc<FileNode>>>,

    pub images: RefCell<Vec<Arc<FileNode>>>,
    pub children: RefCell<Vec<Arc<FileNode>>>,
}

pub fn new_file_node(
    path_str: &String,
    root: &String,
    parent: Option<Arc<FileNode>>,
) -> Option<Arc<FileNode>> {
    if !exist_path(path_str) {
        return None;
    }

    let abs_path_str = Path::new(&path_str)
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let path = Path::new(&abs_path_str);

    let size = get_file_size(&abs_path_str);
    let human_size = get_file_human_size(&path_str);
    let rv = FileNode {
        name: path.file_name().unwrap().to_str().unwrap().to_owned(),
        extension: path
            .extension()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .unwrap()
            .to_owned(),
        abs_path: String::from(&*abs_path_str),
        rel_path: path
            .strip_prefix(root)
            .unwrap_or(Path::new(""))
            .to_owned()
            .to_str()
            .unwrap()
            .to_owned(),
        node_type: if path.is_dir() {
            String::from(NODE_TYPE_DIR)
        } else {
            String::from(NODE_TYPE_FILE)
        },

        size: Cell::new(size),
        total_size: Cell::new(0f64),
        human_size: RefCell::new(human_size),

        // TODO: ignore parent
        parent: None,
        dirs: RefCell::new(vec![]),
        files: RefCell::new(vec![]),

        images: RefCell::new(vec![]),
        children: RefCell::new(vec![]),
    };

    let rv_rc = Arc::new(rv);

    if path.is_dir() {
        // iginore dir metadata space
        rv_rc.size.set(0f64);

        let files = fs::read_dir(&abs_path_str).unwrap();
        for f in files {
            let abs_path = path
                .join(f.unwrap().file_name())
                .to_str()
                .unwrap()
                .to_owned();

            let child_node =
                new_file_node(&abs_path, &abs_path_str, Some(Arc::clone(&rv_rc))).unwrap();
            let child_node_arc = Arc::new(child_node);

            if is_dir(&abs_path) {
                rv_rc
                    .children
                    .borrow_mut()
                    .push(Arc::clone(&child_node_arc));
                rv_rc.dirs.borrow_mut().push(Arc::clone(&child_node_arc));
            } else if is_file(&abs_path) {
                rv_rc
                    .children
                    .borrow_mut()
                    .push(Arc::clone(&child_node_arc));
                rv_rc.files.borrow_mut().push(Arc::clone(&child_node_arc));

                match &*child_node_arc.extension {
                    "jpg" | "jpeg" | "png" | "gif" | "bmp" => {
                        rv_rc.images.borrow_mut().push(Arc::clone(&child_node_arc))
                    }
                    _ => {}
                }
            }
        }
    }

    rv_rc.total_size.set(rv_rc.get_total_size());
    rv_rc
        .children
        .borrow_mut()
        .sort_by(|a, b| a.total_size.partial_cmp(&b.total_size).unwrap());
    Some(rv_rc)
}

impl FileNode {
    fn get_total_size(&self) -> f64 {
        if self.node_type == NODE_TYPE_FILE {
            self.size.get()
        } else {
            let mut rv = 0f64;
            for f in self.files.borrow().iter() {
                rv += f.get_total_size();
            }
            rv
        }
    }
}
