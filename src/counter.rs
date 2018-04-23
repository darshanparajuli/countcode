use std::path::PathBuf;
use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::fmt;

#[derive(Clone)]
pub struct Sloc {
    pub lang: Lang,
    pub files: u64,
    pub lines: u64,
    pub comments: u64,
    pub blanks: u64,
}

pub struct SlocStr {
    pub lang: String,
    pub files: String,
    pub lines: String,
    pub code: String,
    pub comments: String,
    pub blanks: String,
}

#[derive(Clone)]
pub struct CommentInfo {
    pub single_line: HashSet<&'static str>,
    pub multi_line_start: Vec<&'static str>,
    pub multi_line_end: Vec<&'static str>,
}

impl Sloc {
    pub fn new(lang: Lang) -> Self {
        Self {
            lang,
            files: 0,
            lines: 0,
            comments: 0,
            blanks: 0,
        }
    }
}

pub struct Counter {
    path: PathBuf,
    lang: Lang,
    comment_info: CommentInfo,
}

impl Counter {
    pub fn new(path: PathBuf, lang: Lang, comment_info: CommentInfo) -> Self {
        Self {
            path,
            lang,
            comment_info,
        }
    }

    pub fn count(&self) -> Option<Sloc> {
        match File::open(&self.path) {
            Ok(f) => {
                let mut reader = BufReader::new(f);
                let mut sloc = Sloc::new(self.lang.clone());
                sloc.files = 1;
                for line in reader.lines() {
                    if let Ok(line) = line {
                        sloc.lines += 1;
                        if line.is_empty() {
                            sloc.blanks += 1;
                        }
                    }
                }
                Some(sloc)
            }
            Err(_) => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Lang {
    Cpp,
    Rust,
    Toml,
    Markdown,

    Total,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Lang::*;
        match *self {
            Cpp => write!(f, "C++"),
            Rust => write!(f, "Rust"),
            Toml => write!(f, "TOML"),
            Markdown => write!(f, "Markdown"),
            Total => write!(f, "TOTAL"),
        }
    }
}
