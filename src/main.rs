mod utils;

use utils::{file, search_config};
use std::io;
use std::fs::File;
use std::path::Path;

fn main() -> io::Result<()> {
    let config = search_config::SearchConfig::new()
        .with_path(String::from("."))
        .with_ignore_suffix(vec!["txt".to_string()])
        .with_ignore_name(vec![String::from(".git"), String::from("math.rs"), String::from("target")]);


    let path = if let Some(ref path) = config.output_path {
        let path_str: &str = path;
        Path::new(path_str)
    } else {
        Path::new("output.md")
    };
    let mut output = match File::create(&path) {
        Err(e) => return Err(e),
        Ok(file) => file
    };
    if let Ok(files) = file::search_dir(config) {
        file::write_tree(&mut output, &Some(files));
    }
    Ok(())
}

// 读取文件，输出目录
// 添加初始路径
// 添加忽视后缀
// 添加忽视的目录与文件名
// 通过命令行参数配置
// -f 指定文件读取配置
