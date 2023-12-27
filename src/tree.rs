use std::fmt::{Debug, format};
use std::sync::{Arc, Mutex};
use walkdir::{DirEntry, WalkDir};

const  DELIMITER_UNIT: &str = "|——";

#[derive(Debug)]
pub struct Tree {
    root: Option<Arc<Mutex<Node>>>,
    len: usize,
}

#[derive(Debug)]
struct Node {
    data: DirEntry,
    // print prefix
    prefix: String,
    children: Vec<Option<Arc<Mutex<Node>>>>,
}

impl Node {
    fn new(entry: &DirEntry, prefix: &String) -> Self {
        Node {
            data: entry.clone(),
            prefix: prefix.clone(),
            children: Vec::new(),
        }
    }

    fn push(&mut self, node: Option<Arc<Mutex<Node>>>) {
        self.children.push(node);
    }

    fn set_prefix(&mut self, prefix: &String) -> &mut Self {
        self.prefix = prefix.clone();
        self
    }
}

impl Tree {
    pub fn new() -> Self {
        // let walk_dir = WalkDir::new(path.unwrap()).max_depth(1);
        // let Some(Ok(entry)) = walk_dir.into_iter().next() else { todo!() };

        Tree {
            root: None,
            len: 0,
        }
    }

    pub fn insert_root(&mut self, entry: DirEntry) -> &mut Self {
        self.root = Some(Arc::new(Mutex::new(Node::new(&entry, &String::new()))));
        self
    }

    // insert
    pub fn insert(&mut self, entry: DirEntry) -> &mut Self {
        // 解析文件路径
        let paths = self._parse_path(entry.clone());

        if paths.len() == 0 {
            return self;
        }

        let mut root = self.root.clone();
        let mut prefix: String = String::new();
        let mut flag = false;

        for path in paths.iter() {
            if let None = root { return self; }

            prefix.push_str(DELIMITER_UNIT);

            flag = false;
            for node in root.clone().unwrap().lock().unwrap().
                children.
                iter()
            {
                if node.clone().unwrap().lock().unwrap().data.file_name().to_str()
                    ==
                    Some(&path.to_string())
                {
                    root = node.clone();
                    flag = true;
                    break;
                }
            }

            if !flag {
                let node = Some(Arc::new(
                    Mutex::new(Node::new(&entry, &prefix.to_string())),
                ));

                root.unwrap().lock().unwrap().
                    push(node.clone());
                self.set_len(self.len + 1);

                root = node.clone();
            }
        }
        self
    }

    fn _parse_path(&self, entry: DirEntry) -> Vec<String> {
        let mut paths_entry = entry.path().to_str().unwrap();

        if paths_entry == "" {
            return Vec::new();
        }

        // 将根节点切出
        let root_len = self.root.clone().unwrap().
            lock().unwrap().data.path().to_str().unwrap().len();
        paths_entry = &paths_entry[root_len..];

        let paths_split = paths_entry.split("/").
            filter(|&s| -> bool {
                s != ""
            });

        let mut paths = Vec::new();
        for path in paths_split {
            paths.push(path.to_string());
        }

        paths
    }

    // dfs
    fn dfs(&self) -> Vec<Arc<Mutex<Node>>> {
        let mut nodes = Vec::new();

        self._dfs(
            self.root.clone().unwrap().lock().unwrap().
                children.clone(),
            &mut nodes,
        );

        nodes
    }

    fn _dfs(&self, children: Vec<Option<Arc<Mutex<Node>>>>, nodes: &mut Vec<Arc<Mutex<Node>>>) {
        // 若遍历结束，那么直接结束
        if children.len() == 0 {
            return ;
        }

        for node in children.iter() {
            if let Some(node) = node {
                nodes.push(node.clone());

                self._dfs(
                    node.lock().unwrap().children.clone(),
                    nodes,
                );
            }
        }
    }

    pub fn printer(&self) {
        let nodes = self.dfs();

        // 打印根节点
        if let Some(root) = &self.root {
            self._print_line(true, root);
        }

        // 打印子节点
        for node in nodes.iter() {
            self._print_line(false, node);
        }
    }

    // print line
    fn _print_line(&self, root: bool, node: &Arc<Mutex<Node>>) {
        let node = node.lock().unwrap();

        let mut line = format!("{}", node.prefix);

        if root {
            line = format!("{}{}", line, node.data.path().to_str().unwrap())
        } else {
            line = format!("{}{}", line, node.data.file_name().to_str().unwrap())
        }

        // print
        println!("{}", line);
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn set_len(&mut self, len: usize) -> &mut Self {
        self.len = len;
        self
    }
}

#[test]
fn tree_test() {
    let mut file_tree = Tree::new();
    let walk_dir = WalkDir::new("./").max_depth(1);

    for (i, entry) in walk_dir.into_iter().enumerate() {
        if i == 0 {
            if let Ok(entry) = entry {
                file_tree.insert_root(entry);
            }
            continue
        }
        if let Ok(entry) = entry {
            file_tree.insert(entry);
        }
    }

    file_tree.printer();
    println!("tree_len: {}", file_tree.len);
}
