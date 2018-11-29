use lang::{CommentInfo, Lang};
use memmap::Mmap;
use std::fs::File;
use std::ops::AddAssign;
use std::path::Path;
use std::str;

#[derive(Clone)]
pub struct Stats {
    pub files: u64,
    pub lines: u64,
    pub code: u64,
    pub comments: u64,
    pub blanks: u64,
}

impl AddAssign for Stats {
    fn add_assign(&mut self, rhs: Stats) {
        self.files += rhs.files;
        self.lines += rhs.lines;
        self.comments += rhs.comments;
        self.code += rhs.code;
        self.blanks += rhs.blanks;
    }
}

impl<'a> AddAssign<&'a Stats> for Stats {
    fn add_assign(&mut self, rhs: &'a Stats) {
        self.files += rhs.files;
        self.lines += rhs.lines;
        self.comments += rhs.comments;
        self.code += rhs.code;
        self.blanks += rhs.blanks;
    }
}

#[derive(Clone)]
pub struct Sloc {
    pub lang: Lang,
    pub stats: Stats,
}

pub struct SlocStr {
    pub lang: String,
    pub files: String,
    pub lines: String,
    pub code: String,
    pub comments: String,
    pub blanks: String,
}

impl Sloc {
    pub fn new(lang: Lang) -> Self {
        Self {
            lang,
            stats: Stats {
                files: 0,
                lines: 0,
                code: 0,
                comments: 0,
                blanks: 0,
            },
        }
    }
}

struct LineReader<'a> {
    mmap: &'a [u8],
    next_index: usize,
}

impl<'a> LineReader<'a> {
    fn new(mmap: &'a [u8]) -> Self {
        Self {
            mmap,
            next_index: 0,
        }
    }

    fn read_line(&mut self) -> Option<&'a str> {
        let starting_index = self.next_index;
        let mut end_index = self.next_index;

        let length = self.mmap.len();
        while end_index < length {
            match self.mmap[end_index] {
                b'\r' => {
                    self.next_index = end_index + 1;
                    if self.next_index < length {
                        if self.mmap[self.next_index] == b'\n' {
                            self.next_index += 1;
                        }
                    }

                    return str::from_utf8(&self.mmap[starting_index..end_index]).ok();
                }

                b'\n' => {
                    self.next_index = end_index + 1;
                    return str::from_utf8(&self.mmap[starting_index..end_index]).ok();
                }

                _ => {
                    end_index += 1;
                }
            }
        }

        None
    }
}

pub struct Counter<'a> {
    path: &'a Path,
    lang: Lang,
    comment_info: CommentInfo,
}

impl<'a> Counter<'a> {
    pub fn new(path: &'a Path, lang: Lang, comment_info: CommentInfo) -> Self {
        Self {
            path,
            lang,
            comment_info,
        }
    }

    pub fn count(&mut self) -> Option<Sloc> {
        match File::open(&self.path) {
            Ok(f) => {
                let mmap = match unsafe { Mmap::map(&f) } {
                    Ok(mmap) => mmap,
                    Err(_) => {
                        return None;
                    }
                };

                let mut line_reader = LineReader::new(&mmap);
                let mut sloc = Sloc::new(self.lang.clone());
                sloc.stats.files = 1;

                let mut multi_line_comment = false;
                let mut multi_line_comment_index = 0;

                loop {
                    let mut line = match line_reader.read_line() {
                        Some(line) => line,
                        None => break,
                    };

                    let line = line.trim();
                    sloc.stats.lines += 1;

                    if line.is_empty() {
                        sloc.stats.blanks += 1;
                        continue;
                    }

                    if multi_line_comment {
                        if line
                            .ends_with(self.comment_info.multi_line_end[multi_line_comment_index])
                        {
                            multi_line_comment = false;
                            multi_line_comment_index = 0;
                        }
                        sloc.stats.comments += 1;
                    } else {
                        let mut skip_single_line_check = false;
                        for i in 0..self.comment_info.multi_line_start.len() {
                            if line.starts_with(self.comment_info.multi_line_start[i]) {
                                multi_line_comment = true;
                                multi_line_comment_index = i;
                                sloc.stats.comments += 1;
                            }

                            if multi_line_comment {
                                if line.ends_with(
                                    self.comment_info.multi_line_end[multi_line_comment_index],
                                ) {
                                    multi_line_comment = false;
                                    skip_single_line_check = true;
                                }

                                break;
                            }
                        }

                        if !multi_line_comment && !skip_single_line_check {
                            let is_comment = {
                                if self.comment_info.single_line.is_empty() {
                                    false
                                } else {
                                    self.comment_info
                                        .single_line
                                        .iter()
                                        .filter(|a| line.starts_with(*a))
                                        .count()
                                        >= 1
                                }
                            };

                            if is_comment {
                                sloc.stats.comments += 1;
                            }
                        }
                    }
                }
                sloc.stats.code = sloc.stats.lines - sloc.stats.comments - sloc.stats.blanks;
                Some(sloc)
            }
            Err(_) => None,
        }
    }
}
