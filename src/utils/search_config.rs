pub struct SearchConfig {
    path: Option<String>,
    ignore_suffix: Option<Vec<String>>,
    ignore_name: Option<Vec<String>>,
    pub(crate) output_path: Option<String>,
}

impl SearchConfig {
    pub fn new(path: Option<String>, ignore_suffix: Option<Vec<String>>, ignore_name: Option<Vec<String>>, output_path: Option<String>) -> Self {
        Self { path, ignore_suffix, ignore_name, output_path: output_path }
    }
}