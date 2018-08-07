use super::node::FileNode;
use super::node::NODE_TYPE_DIR;
use std::iter::repeat;

static SPACE_ONE: &'static str = " ";
static SPACE_MULTI: &'static str = "   ";
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
    for (i, x) in node.children.borrow().iter().enumerate() {
        // first
        if i == 0 {
            prefix.push(T_PREFIX);
        }

        // last
        if i + 1 == node.children.borrow().len() {
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
            if x.name[0..1] == *"." {
                continue;
            }

            prefix.pop();
            // last
            if i + 1 == node.children.borrow().len() {
                prefix.push(SPACE_ONE);
            } else {
                prefix.push(VERTICAL_LINE);
            }

            prefix.push(SPACE_MULTI);
            print_file_node_tree(node, prefix, depth + 1, level, human);

            prefix.pop();
            prefix.pop();
            prefix.push(T_PREFIX);
        }
    }
}

pub fn get_size_text(size: f64, human: bool) -> String {
    if human {
        return super::size::human_size(size, 1000f64);
    }
    return format!("{}", size);
}
