extern crate filetree;

use filetree::node;

fn main() {
    let n = node::new_file_node(&".".to_owned(), &".".to_owned(), None);
    println!("{:?}", n);
}
