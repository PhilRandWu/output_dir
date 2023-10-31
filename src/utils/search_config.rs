pub struct SearchConfig {
    pub(crate) path: Option<String>, // current search path
    pub(crate) ignore_suffix: Option<Vec<String>>,
    pub(crate) ignore_name: Option<Vec<String>>,
    pub(crate) output_path: Option<String>,  // output path
}

impl SearchConfig {
    pub fn new() -> Self {
        Self { path: None, ignore_suffix: None, ignore_name: None, output_path: None }
    }

    pub fn with_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }

    pub fn with_ignore_suffix(mut self, ignore_suffix: Vec<String>) -> Self {
        self.ignore_suffix = Some(ignore_suffix);
        self
    }

    pub fn with_ignore_name(mut self,ignore_name: Vec<String>) -> Self {
        self.ignore_name = Some(ignore_name);
        self
    }

    pub fn with_output_path(mut self,output_path: String) -> Self {
        self.output_path = Some(output_path);
        self
    }
}