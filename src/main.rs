extern crate filetree;

use filetree::node;

fn main() {
    let n = node::new_file_node(
        &"src/filetree".to_owned(),
        &"/Users/weaming".to_owned(),
        None,
    ).unwrap();
    println!("{:?}", n.abs_path);
    println!("{:?}", n.rel_path);
}
