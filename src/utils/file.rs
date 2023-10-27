use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

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
pub fn search_dir(init_path: Option<&str>) -> Result<Vec<FileInfo>, io::Error> {
    let path = init_path.as_ref().map_or(Path::new("."), Path::new);
    let mut result = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_type = if entry.file_type().map_or(false, |t| t.is_dir()) {
                "dir".to_owned()
            } else {
                "file".to_owned()
            };
            let file_name = entry.file_name().to_string_lossy().to_string();
            let mut current_info = match file_type.as_str() {
                "dir" => {
                    let mut info = FileInfo::new(file_name.clone(), file_type.clone(), Some(vec![]));
                    info.children = Some(search_dir(
                        Some(path.join(&file_name).to_str().unwrap()),
                    )?);
                    info
                }
                _ => FileInfo::new(file_name.clone(), file_type.clone(), None),
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
    write!(output,"{}\n{}\n","````","```");
    print_tree(output, nodes, 0);
    write!(output,"{}\n{}\n","```","````");
}