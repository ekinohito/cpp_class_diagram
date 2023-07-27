use std::{fs::File, io::Read};

use clang_ast::SourceRange;

pub struct Source {
    #[allow(dead_code)]
    file_name: String,
    text: String,
}

impl Source {
    pub fn new(file_name: &str) -> Self {
        let mut file = File::open(file_name).unwrap(); 
        let mut text = String::new();
        file.read_to_string(&mut text).unwrap();
        let file_name = file_name.to_owned();
        Self { file_name, text }
    }

    pub fn get(&self, range: &SourceRange) -> Option<&str> {
        let begin = range.begin.spelling_loc.as_ref()?;
        let end = range.end.spelling_loc.as_ref()?;
        Some(&self.text[begin.offset..end.offset+end.tok_len])
    }
}
