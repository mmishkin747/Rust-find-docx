use clap::{Arg, ArgAction, Command};
use colored::Colorize;
use dotext::*;
use regex::{Regex, RegexBuilder};
use std::error::Error;
use std::io::Read;
use walkdir::{DirEntry, WalkDir};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    names: Vec<Regex>,
    excel: bool,
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
            Arg::new("excel")
                .short('e')
                .help("Add find excel files, (not implemented)")
                .action(ArgAction::SetFalse),
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
                .help("Case-insensitive")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let mut names: Vec<Regex> = Vec::new();
    names.push(Regex::new("\\.doc").unwrap());
    let excel = matches.get_flag("excel");
    if excel {
        names.push(Regex::new("\\.xlsx").unwrap());
    }

    let pattern = matches.get_one::<String>("pattern").map(|v| v.to_string());

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap_or_default()
            .map(|v| v.to_string())
            .collect::<Vec<_>>(),
        excel,
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
        match open(&filename) {
            Ok(file) => match &config.pattern {
                Some(pattern) => find_line(&file, &filename, pattern, config.insensitive),
                _ => print_file(&file, &filename, &config.non_formating)?,
            },

            _ => {}
        }
    }

    Ok(())
}

fn print_file(file: &String, filename: &str, non_formating: &bool) -> MyResult<()> {
    println!("{:-^30}", filename.green());
    for line in file.lines() {
        if *non_formating || line.len() > 0 {
            println!("{}", line);
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<String> {
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

fn find_line(file: &String, filename: &str, pattern: &String, insensitive: bool) {
    let pattern = RegexBuilder::new(pattern.as_str())
        .case_insensitive(insensitive)
        .build()
        .unwrap();
    let mut count_match = 0;
    for (count, line) in file.lines().enumerate() {
        if pattern.is_match(line) {
            if count_match == 0 {
                println!("{:-^30}", filename.green())
            }
            println!("{}. {}", count, line);
            count_match += 1;
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
                    eprintln!("{}", e);
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
