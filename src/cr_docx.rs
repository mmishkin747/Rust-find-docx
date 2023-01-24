use dotext::*;
use std::io::Read;

use crate::MyResult;


pub fn open_docx(filename: &str, non_formating: &bool) -> MyResult<String> {
    let docx = Docx::open(filename);
    let mut buf = String::new();
    if let Ok(mut file) = docx {
        let _ = file.read_to_string(&mut buf);
    }
    let mut res = filename.to_string();
    res.push_str("\n");

    for line in buf.lines(){

        if *non_formating || line.len() > 0 {
            res.push_str(line);
            res.push_str("\n");
        }
    }      

    Ok(res)
}