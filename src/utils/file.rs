use std::fs;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::utils::search_config::SearchConfig;

#[derive(Debug, Clone, PartialEq)]
pub struct FileInfo {
    name: String,
    r#type: String,
    children: Option<Vec<FileInfo>>,
}

impl FileInfo {
    pub fn new(name: String, r#type: String, children: Option<Vec<FileInfo>>) -> Self {
        Self { name, r#type, children }
    }
}

/**
 * 传入路径查询文档结构
 * @param init_path 初始查询路径
 */
pub fn search_dir(config: SearchConfig) -> Result<Vec<FileInfo>, io::Error> {
    let mut result = Vec::new();
    if let Ok(entries) = fs::read_dir(Path::new(config.path.as_deref().unwrap_or("."))) {
        for entry in entries.flatten() {
            let file_type = if entry.file_type().map_or(false, |t| t.is_dir()) {
                "dir".to_owned()
            } else {
                "file".to_owned()
            };
            let file_name = entry.file_name().to_string_lossy().to_string();
            let mut current_info = match file_type.as_str() {
                "dir" => {
                    if !config.ignore_name.clone().unwrap().contains(&file_name) {
                        let mut info = FileInfo::new(file_name.clone(), file_type.clone(), Some(vec![]));
                        let next_path_str = format!("{}/{}", config.path.clone().unwrap_or_default(), file_name);
                        let children_config = SearchConfig::new()
                            .with_path(next_path_str)
                            .with_ignore_suffix(config.ignore_suffix.clone().unwrap_or_default())
                            .with_ignore_name(config.ignore_name.clone().unwrap_or_default())
                            .with_output_path(config.output_path.clone().unwrap_or_default());
                        info.children = Some(search_dir(children_config)?);
                        info
                    } else {
                        continue
                    }
                }
                _ => {
                    if !config.ignore_suffix.clone().unwrap().contains(&get_suffix(&file_name))
                        && !config.ignore_name.clone().unwrap().contains(&file_name) {
                        FileInfo::new(file_name.clone(), file_type.clone(), None)
                    } else {
                        continue;
                    }
                }
            };
            result.push(current_info);
        }
    }
    Ok(result)
}

fn print_tree(output: &mut dyn Write, nodes: &Option<Vec<FileInfo>>, level: usize) -> io::Result<()> {
    if let Some(nodes) = nodes {
        for node in nodes {
            match node.r#type.as_str() {
                "dir" => write!(output, "{}└─{}\n", "│   ".repeat(level), node.name)?,
                _ => write!(output, "{}{}\n", "│   ".repeat(level), node.name)?
            }
            print_tree(output, &node.children, level + 1);
        }
    }
    Ok(())
}

/**
 * 将目录写入文件
 * @param output
 * @param nodes
 * @return
 */
pub fn write_tree(output: &mut dyn Write, nodes: &Option<Vec<FileInfo>>) {
    write!(output, "{}\n{}\n", "````", "```");
    print_tree(output, nodes, 0);
    write!(output, "{}\n{}\n", "```", "````");
}

pub fn get_suffix(file_name: &str) -> String {
    match file_name.split(".").last() {
        Some(suffix) => suffix.to_string(),
        None => "".to_string()
    }
}