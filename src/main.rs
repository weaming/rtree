extern crate filetree;

use filetree::{node, print};

fn main() {
    let path = ".";
    let root = "/User";
    let level = 1024;
    let human = true;
    let root_node = node::new_file_node(&String::from(path), &String::from(root), None).unwrap();

    print::print_file_node_tree(&root_node, &mut vec![], 1, level, human)
}
