use std::fmt::Debug;
use std::string::ToString;
use std::sync::{
    Arc,
    Mutex,
};

const DELIMITER_UNIT: &str = "——";

#[derive(Default, Debug)]
pub struct Tree {
    root: Arc<Mutex<Node>>,
    len: usize,
}

#[derive(Default, Debug)]
struct Node {
    data: Entry,
    children: Vec<Arc<Mutex<Node>>>,
    // 结束
    end: bool,
}

#[derive(Default, Debug)]
struct Entry {
    name: String,
}

impl Node {
    fn new(name: &String, end: bool) -> Self {
        Node {
            data: Entry {
                name: name.clone(),
            },
            children: Vec::new(),
            end: end,
        }
    }

    fn push(&mut self, node: Arc<Mutex<Node>>) {
        self.children.push(node)
    }

    fn set_end(&mut self, end: bool) {
        self.end = end;
    }
}

impl Tree {
    pub fn new() -> Self {
        Tree::default()
    }

    // insert
    // @param path &String eg-"/root/ect/b.txt"
    pub fn insert(&mut self, path: &String) {
        // 解析文件路径
        let paths = self._parse_path(path);
        if paths.len() == 0 {
            return ;
        }

        let mut root = self.root.clone();
        let mut flag = false;

        for (idx, path) in paths.iter().enumerate() {
            flag = false;

            let children = root.lock().unwrap().
                children.
                clone();

            for node in children.iter() {
                let name = &node.
                    lock().
                    unwrap().
                    data.
                    name;

                if name == &path.to_string() {
                    root = node.clone();
                    flag = true;
                    break;
                }
            }

            // 判断是否循环到名字
            let mut end = false;
            if idx == paths.len() - 1 {
                end = true;
            }

            if !flag {
                let node = Arc::new(
                    Mutex::new(Node::new(&path.to_string(), end))
                );

                root.lock().unwrap().
                    push(node.clone());
                self.len += 1;

                root = node.clone();
            } else {
                root.lock().unwrap().
                    set_end(end);
            }
        }
    }

    fn _parse_path(&self, path: &String) -> Vec<String> {
        if *path == "" {
            return Vec::new();
        }

        let paths_split = path.split("/").
            filter(|&s| -> bool {
                s != ""
            });

        let mut paths = Vec::new();
        for path in paths_split {
            paths.push(path.to_string());
        }

        paths
    }

    // 遍历
    fn dfs(&self) -> Vec<String> {
        let mut names = Vec::new();

        // 递归遍历树
        let children = self.root.lock().unwrap().
            children.
            clone();

        self._dfs(children, &mut names, &DELIMITER_UNIT.to_string());

        names
    }

    fn _dfs(&self, children: Vec<Arc<Mutex<Node>>>, names: &mut Vec<String>, prefix: &String) {
        // 若遍历结束，那么直接结束
        if children.len() == 0 {
            return ;
        }

        for node in children.iter() {
            let node = node.lock().unwrap();

            names.push(format!("{}{}", prefix, node.data.name));

            self._dfs(
                node.children.clone(),
                names,
                &format!("{}{}", DELIMITER_UNIT, prefix),
            );
        }
    }

    pub fn printer(&self) {
        let names = self.dfs();

        for name in names.iter() {
            println!("|{}", name);
        }
    }
    // len
    pub fn len(&self) -> usize {
        self.len
    }
}

#[test]
fn tree_test() {
    let mut tree = Tree::new();
    tree.insert(&"/root/ect/a.txt".to_string());
    tree.insert(&"/root/ect/b.txt".to_string());
    tree.insert(&"/root/ect/c.txt".to_string());

    tree.insert(&"/users/ect/a.txt".to_string());
    tree.insert(&"/users/ect/b.txt".to_string());
    tree.insert(&"/users/ect/c.txt".to_string());

    tree.printer();

    // println!("{:#?}", tree);
    // println!("iter: {:#?}, len: {:?}", tree.dfs(), tree.len());
}


