extern crate filetree;

use std::env;

use filetree::{node, print};

fn main() {
    let path = ".";
    let root = "/User";
    let level = 100;
    let human = true;
    let root_node = node::new_file_node(&String::from(path), &String::from(root)).unwrap();

    match env::var("INDENT_MODE") {
        Ok(_) => print::print_file_node_simplely(&root_node, 1, level, human),
        Err(_) => print::print_file_node_tree(&root_node, &mut vec![], 1, level, human),
    }
}
