mod utils;

use utils::{file,search_config};
use std::io;
use std::fs::File;
use std::path::Path;

fn main() -> io::Result<()> {
    let config = search_config::SearchConfig::new(None,None,None,Some("output.md".to_string()));

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
    if let Ok(files) = file::search_dir(None) {
        file::write_tree(&mut output, &Some(files));
    }
    Ok(())
}