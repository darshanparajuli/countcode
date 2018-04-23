use std::path::PathBuf;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::sync::Arc;
use lang::Lang;

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
    pub single_line: Arc<Vec<&'static str>>,
    pub multi_line_start: Arc<Vec<&'static str>>,
    pub multi_line_end: Arc<Vec<&'static str>>,
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
                let mut multi_line_comment = false;
                let mut multi_line_comment_index = 0;
                for line in reader.lines() {
                    if let Ok(line) = line {
                        let line = line.trim();
                        sloc.lines += 1;

                        if line.is_empty() {
                            sloc.blanks += 1;
                            continue;
                        }

                        if multi_line_comment {
                            if line.ends_with(
                                self.comment_info.multi_line_end[multi_line_comment_index],
                            ) {
                                multi_line_comment = false;
                                multi_line_comment_index = 0;
                            }
                            sloc.comments += 1;
                        } else {
                            let is_comment = self.comment_info
                                .single_line
                                .iter()
                                .filter(|a| line.starts_with(*a))
                                .count() >= 1;

                            if is_comment {
                                sloc.comments += 1;
                            } else {
                                for i in 0..self.comment_info.multi_line_start.len() {
                                    if line.starts_with(self.comment_info.multi_line_start[i]) {
                                        multi_line_comment = true;
                                        multi_line_comment_index = i;
                                        sloc.comments += 1;
                                    }

                                    if multi_line_comment {
                                        if line.ends_with(
                                            self.comment_info.multi_line_end
                                                [multi_line_comment_index],
                                        ) {
                                            multi_line_comment = false;
                                        }

                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                Some(sloc)
            }
            Err(_) => None,
        }
    }
}
