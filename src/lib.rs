use clap::{Arg, ArgAction, Command};
use colored::Colorize;
use regex::{Regex, RegexBuilder};
use std::error::Error;
use walkdir::{DirEntry, WalkDir};
use crate::cr_xlxs::open_xlsx;
use crate::cr_docx::open_docx;

pub mod cr_xlxs;
pub mod cr_docx;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    names: Vec<Regex>,
    non_formating: bool,
    pattern: Option<String>,
    insensitive: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("rfd")
        .version("0.0.1")
        .author("Mikhail Vasilchyk <mmishkin747@gmail.com>")
        .about("Rust find docx")
        .arg(
            Arg::new("files")
                .value_name("FILE / PATH")
                .help("Input file(s) or/and path(s)")
                .required(true)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("non_formating")
                .long("non-formating")
                .help("Non-formating text")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("pattern")
                .value_name("Pattertn")
                .short('p')
                .help("Find pattern"),
        )
        .arg(
            Arg::new("insensitive")
                .short('i')
                .help("Case-sensitive")
                .action(ArgAction::SetFalse),
        )
        .get_matches();

    let mut names: Vec<Regex> = Vec::new();
    names.push(Regex::new("\\.doc").unwrap());
    names.push(Regex::new("\\.xlsx").unwrap());

    let pattern = matches.get_one::<String>("pattern").map(|v| v.to_string());

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap_or_default()
            .map(|v| v.to_string())
            .collect::<Vec<_>>(),
        non_formating: matches.get_flag("non_formating"),
        names,
        pattern,
        insensitive: matches.get_flag("insensitive"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let files = walk(&config).unwrap();
    for filename in files {
        match open(&filename,  &config.non_formating) {
            Ok(file) => match &config.pattern {
                Some(pattern) => find_line(&file, pattern, config.insensitive),
                _ => print_file(&file)?,
            },

            _ => {}
        }
    }

    Ok(())
}

fn print_file(file: &String) -> MyResult<()> {
    let mut start_file:bool = true;
    for line in file.lines() {
        if start_file {
            println!("{}", line.bold().blue());
            start_file = false;
        }else{
            println!("{}", line);
        }
        
    }
    Ok(())
}

fn open(filename: &str, non_formating: &bool) -> MyResult<String> {
    match filename {
        s if s.ends_with(".docx") => open_docx(filename, non_formating),
        s if s.ends_with(".xlsx") => open_xlsx(filename),
        _ => Err("error type".into())
    }
}



fn find_line(file: &String, pattern: &String, insensitive: bool) {
    let pattern = RegexBuilder::new(pattern.as_str())
        .case_insensitive(insensitive)
        .build()
        .unwrap();
    
    for (count, line) in file.lines().enumerate() {
 
        if count !=0 && pattern.is_match(line) {
            print!("{}. ", count);
            for word in line.split_ascii_whitespace(){

                if pattern.is_match(word){
                    print!("{} ", word.on_yellow());
                }else{
                    print!("{} ", word);
                }
            }
            println!();
                
        }else{
            if  count == 0 {
                if  pattern.is_match(line){
                    println!("{}", line.bold().on_blue());
                }else{
                    println!("{}", line.bold().blue());
                }
            } 
        }

    }
}

fn walk(config: &Config) -> MyResult<Vec<String>> {
    let name_file = |entry: &DirEntry| {
        config.names.is_empty()
            || config
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    let mut files: Vec<String> = Vec::new();
    for path in &config.files {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{} -- {}", path.red(), e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(name_file)
            .filter(|s| s.file_type().is_file())
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();
        if !entries.is_empty() {
            for path in entries {
                files.push(path);
            }
        }
    }

    Ok(files)
}
