use std::sync::{Arc, Mutex};
use walkdir::{DirEntry, WalkDir};

const DELIMITER_UNIT: &str = "|——";

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
    fn new(entry: &DirEntry) -> Self {
        Node {
            data: entry.clone(),
            prefix: "".to_string(),
            children: Vec::new(),
        }
    }

    fn push(&mut self, node: Option<Arc<Mutex<Node>>>) {
        self.children.push(node);
    }

    fn set_prefix(&mut self, prefix: String) -> &mut Self {
        self.prefix = prefix.clone();
        self
    }
}

impl Tree {
    pub fn new() -> Self {
        // 随便创建一个root, 总之不用
        let walk_dir = WalkDir::new("/").max_depth(1);
        let Some(Ok(entry)) = walk_dir.into_iter().next() else { todo!() };

        Tree {
            root: Some(Arc::new(Mutex::new(
                Node::new(&entry),
            ))),
            len: 0,
        }
    }

    // insert
    pub fn insert(&mut self, entry: DirEntry) {
        // 解析文件路径
        let paths = self._parse_path(entry.clone());

        if paths.len() == 0 {
            return ;
        }

        let mut root = self.root.clone();
        let mut flag = false;

        for path in paths.iter() {
            if let None = root { return ; }

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
                    Mutex::new(Node::new(&entry)),
                ));

                root.unwrap().lock().unwrap().
                    push(node.clone());
                self.set_len(self.len + 1);

                root = node.clone();
            }
        }
    }

    fn _parse_path(&self, entry: DirEntry) -> Vec<String> {
        let paths_entry = entry.path().to_str().unwrap();

        if paths_entry == "" {
            return Vec::new();
        }

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
            &mut nodes, &"".to_string(),
        );

        nodes
    }

    fn _dfs(&self, children: Vec<Option<Arc<Mutex<Node>>>>, nodes: &mut Vec<Arc<Mutex<Node>>>, prefix: &String) {
        // 若遍历结束，那么直接结束
        if children.len() == 0 {
            return ;
        }

        for node in children.iter() {
            if let Some(node) = node {

                node.lock().unwrap().set_prefix((*prefix).clone());
                nodes.push(node.clone());

                self._dfs(
                    node.lock().unwrap().children.clone(),
                    nodes,
                    &format!("{}{}", DELIMITER_UNIT, prefix),
                );
            }
        }
    }

    pub fn printer(&self) {
        let nodes = self.dfs();

        for node in nodes.iter() {
            let node = node.lock().unwrap();

            let mut line = format!("{}", node.prefix);

            if let Some(name) = node.data.file_name().to_str() {
                line = format!("{}{}", line, name);
            }

            // print
            println!("{}", line);
        }
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
    let walk_dir = WalkDir::new("/var");

    for entry in walk_dir {
        if let Ok(entry) = entry {
            file_tree.insert(entry);
        }
    }

    file_tree.printer();
    println!("tree_len: {}", file_tree.len);
}
