use colored::Colorize;
use dotext::*;
use std::io::Read;

use crate::MyResult;

pub fn open_docx(filename: &str) -> MyResult<String> {
    match Docx::open(filename) {
        Ok(mut file) => {
            let mut buf = String::new();
            let _ = file.read_to_string(&mut buf);
            Ok(buf)
        }
        Err(e) => {
            eprintln!("{:-^30} {}", filename.red(), e);
            Err(Box::new(e))
        }
    }
}