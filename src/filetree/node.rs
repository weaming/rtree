use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::sync::Arc;

static NODE_TYPE_DIR: &'static str = "dir";
static NODE_TYPE_FILE: &'static str = "file";

#[derive(Debug)]
pub struct FileNode {
    pub name: String,
    pub extension: String,
    pub abs_path: String,
    pub rel_path: String,
    pub node_type: String,

    pub size: f64,
    pub total_size: f64,
    pub human_size: String,

    pub parent: Option<Arc<FileNode>>,
    pub dirs: RefCell<Vec<Arc<FileNode>>>,
    pub files: RefCell<Vec<Arc<FileNode>>>,

    pub images: RefCell<Vec<Arc<FileNode>>>,
    pub children: RefCell<Vec<Arc<FileNode>>>,
}

pub fn new_file_node<'a>(
    path_str: &String,
    root: &String,
    parent: Option<Arc<FileNode>>,
) -> Arc<FileNode> {
    let abs_path_str = Path::new(&path_str)
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let path = Path::new(&abs_path_str);

    let mut rv = FileNode {
        name: path.file_name().unwrap().to_str().unwrap().to_owned(),
        extension: path.extension().unwrap().to_str().unwrap().to_owned(),
        abs_path: String::from(&*abs_path_str),
        rel_path: path
            .strip_prefix(root)
            .unwrap()
            .to_owned()
            .to_str()
            .unwrap()
            .to_owned(),
        node_type: if path.is_dir() {
            String::from(NODE_TYPE_DIR)
        } else {
            String::from(NODE_TYPE_FILE)
        },

        size: get_file_size(&abs_path_str),
        human_size: get_file_human_size(&path_str),
        total_size: 0f64,

        parent: parent,
        dirs: RefCell::new(vec![]),
        files: RefCell::new(vec![]),

        images: RefCell::new(vec![]),
        children: RefCell::new(vec![]),
    };

    let rv_rc = Arc::new(rv);

    if path.is_dir() {
        rv.size = 0f64;

        let files = fs::read_dir(&abs_path_str).unwrap();
        for f in files {
            let abs_path = path
                .join(f.unwrap().file_name())
                .to_str()
                .unwrap()
                .to_owned();

            let child_file_node = Arc::new(new_file_node(
                &abs_path,
                &abs_path_str,
                Some(Arc::clone(&rv_rc)),
            ));

            if is_dir(&abs_path) {
                rv_rc
                    .children
                    .borrow_mut()
                    .push(Arc::clone(&child_file_node));
                rv_rc.dirs.borrow_mut().push(Arc::clone(&child_file_node));
            } else if is_file(&abs_path) {
                rv_rc
                    .children
                    .borrow_mut()
                    .push(Arc::clone(&child_file_node));
                rv_rc.files.borrow_mut().push(Arc::clone(&child_file_node));

                match &*child_file_node.extension {
                    "jpg" | "jpeg" | "png" | "gif" | "bmp" => {
                        rv_rc.images.borrow_mut().push(Arc::clone(&child_file_node))
                    }
                    _ => {}
                }
            }
        }
    }

    rv.total_size = rv_rc.get_total_size();
    rv_rc
        .children
        .borrow_mut()
        .sort_by(|a, b| a.total_size.partial_cmp(&b.total_size).unwrap());
    rv_rc
}

impl FileNode {
    fn get_total_size(&self) -> f64 {
        if self.node_type == NODE_TYPE_FILE {
            self.size
        } else {
            let mut rv = 0f64;
            for f in self.files.borrow().iter() {
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

fn is_dir(path: &String) -> bool {
    let p = Path::new(&path);
    p.is_dir()
}

fn is_file(path: &String) -> bool {
    let p = Path::new(&path);
    p.is_file()
}
