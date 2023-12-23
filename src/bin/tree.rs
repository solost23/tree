//! A simple tree command

use tree::Tree;

fn main() {
    let paths = vec![
        "/root/ect/a.txt",
        "/root/ect/b.txt",
        "/root/ect/c.txt",
    ];

    let mut file_tree = Tree::new();

    for path in paths.iter() {
        file_tree.insert(&path.to_string());
    }

    // 打印文件树
    file_tree.printer();
}
