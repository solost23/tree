use std::{env, path};
use clap::Parser;
use walkdir::{DirEntry, WalkDir};
use crate::Tree;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(short, long, default_value_t = String::from("."))]
    path: String,
    #[clap(short, long, default_value_t = false)]
    dir: bool,
    #[clap(short, long, default_value_t = false)]
    link: bool,
    #[clap(short, long, default_value_t = 18_446_744_073_709_551_615_usize)]
    max: usize,
}

impl Cli {
    pub fn new() -> Self{
        Cli::parse()
    }

    // core login
    pub fn run(&self) {
        let mut current_dir = path::PathBuf::from(self.path.clone());

        let walk_dir = WalkDir::new(current_dir.clone()).
            // 查询深度
            max_depth(self.max).
            // 文件link
            follow_links(self.link);

        // 是否仅查询文件夹
        let walk_iter = walk_dir.into_iter();
        let mut entries = Vec::new();
        if self.dir {
            let filter_func = |d: &DirEntry| -> bool {
                d.file_type().is_dir()
            };
            for entry in walk_iter.filter_entry(filter_func) {
                if let Ok(entry) = entry {
                    entries.push(entry);
                }
            }
        } else {
            for entry in walk_iter {
                if let Ok(entry) = entry {
                    entries.push(entry);
                }
            }
        }

        // 收集文件||文件夹
        let mut file_tree = Tree::new();
        for (i, entry) in entries.iter().enumerate() {
            // 第一个节点就是根节点
            if i == 0 {
                file_tree.insert_root(entry.clone());
            }
            file_tree.insert(entry.clone());
        }

        file_tree.printer();
    }
}