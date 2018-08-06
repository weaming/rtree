extern crate filetree;

use filetree::node;

fn main() {
    let n = node::new_file_node(".", None);
    println!("{}", n);
}
