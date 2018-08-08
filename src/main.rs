extern crate clap;
extern crate filetree;

use clap::{App, Arg};

use std::env;

use filetree::{node, print};

fn main() {
    let matches = App::new("rtree")
        .version("1.0")
        .author("weaming <garden.yuen@gmail.com>")
        .about("simple tree implemented in Rust")
        .arg(
            Arg::with_name("raw")
                .long("raw")
                .help("print raw size in bytes"),
        ).arg(
            Arg::with_name("root")
                .value_name("ROOT")
                .short("r")
                .long("root")
                .help("the root to get relative path of tree data structure"),
        ).arg(
            Arg::with_name("path")
                .value_name("PATH")
                .help("the directory to be list"),
        ).get_matches();
    let path = matches.value_of("path").unwrap_or(".");
    let root = matches.value_of("root").unwrap_or(&path);
    let human = !matches.is_present("raw");
    let level = 100;

    let root_node = node::new_file_node(&String::from(path), &String::from(root)).unwrap();

    match env::var("INDENT_MODE") {
        Ok(_) => print::print_file_node_simplely(&root_node, 1, level, human),
        Err(_) => print::print_file_node_tree(&root_node, &mut vec![], 1, level, human),
    }
}
