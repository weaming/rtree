use super::node::FileNode;
use super::node::NODE_TYPE_DIR;
use std::iter::repeat;

static SPACE_ONE: &'static str = " ";
static SPACE_THREE: &'static str = "   ";
static SPACE_FOUR: &'static str = "    ";
static HORIZONTAL_LINE: &'static str = "─";
static VERTICAL_LINE: &'static str = "│";
static T_PREFIX: &'static str = "├";
static END_PREFIX: &'static str = "└";

pub fn print_file_node_tree(
    node: &FileNode,
    prefix: &mut Vec<&str>,
    depth: u64,
    level: u64,
    human: bool,
) {
    let n_children = node.children.borrow().len();
    for (i, x) in node.children.borrow().iter().enumerate() {
        // first
        if i == 0 {
            prefix.push(T_PREFIX);
        }

        // last
        if i + 1 == n_children {
            prefix.pop();
            prefix.push(END_PREFIX);
        }

        println!(
            "{}{} {} {}",
            prefix.join(""),
            repeat(HORIZONTAL_LINE).take(2).collect::<String>(),
            x.name,
            get_size_text(x.total_size.get(), human)
        );

        if x.node_type == NODE_TYPE_DIR && depth < level {
            if x.name.chars().next().unwrap() == '.' {
                continue;
            }

            prefix.pop();
            // last
            if i + 1 == n_children {
                prefix.push(SPACE_ONE);
            } else {
                prefix.push(VERTICAL_LINE);
            }

            prefix.push(SPACE_THREE);
            print_file_node_tree(x, &mut prefix.to_vec(), depth + 1, level, human);

            prefix.pop();
            prefix.pop();
            prefix.push(T_PREFIX);
        }
    }
}

fn print_node(node: &FileNode, human: bool, depth: u64) {
    println!(
        "{}{} {}",
        repeat(SPACE_FOUR)
            .take((depth - 1) as usize)
            .collect::<String>(),
        node.rel_path,
        get_size_text(node.total_size.get(), human)
    );
}

pub fn print_file_node_simplely(node: &FileNode, depth: u64, level: u64, human: bool) {
    for x in node.children.borrow().iter() {
        print_node(x, human, depth);

        if x.node_type == NODE_TYPE_DIR && depth < level {
            if x.name.chars().next().unwrap() == '.' {
                continue;
            }

            print_file_node_simplely(x, depth + 1, level, human);
        }
    }
}

pub fn get_size_text(size: f64, human: bool) -> String {
    if human {
        return super::size::human_size(size, 1000f64);
    }
    return format!("{}", size);
}
