use jobpool::JobPool;
use num_cpus;
use std::sync::mpsc;
use std::collections::HashSet;
use std::collections::hash_map::{Entry, HashMap};
use counter::{CommentInfo, Counter, Lang, Sloc, SlocStr};
use walkdir::WalkDir;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

pub struct Scanner {
    jobpool: JobPool,
    extensions: HashMap<&'static str, Lang>,
    comment_info: HashMap<Lang, CommentInfo>,
    ignore_files: HashSet<String>,
}

impl Scanner {
    pub fn new() -> Self {
        let jobpool = JobPool::new(num_cpus::get());
        let mut extensions = HashMap::new();

        extensions.insert("c", Lang::C);
        extensions.insert("c++", Lang::Cpp);
        extensions.insert("cc", Lang::Cpp);
        extensions.insert("cpp", Lang::Cpp);
        extensions.insert("cxx", Lang::Cpp);
        extensions.insert("h", Lang::CHeader);
        extensions.insert("h++", Lang::CppHeader);
        extensions.insert("hh", Lang::CppHeader);
        extensions.insert("hpp", Lang::CppHeader);
        extensions.insert("hxx", Lang::CppHeader);
        extensions.insert("md", Lang::Markdown);
        extensions.insert("rs", Lang::Rust);
        extensions.insert("toml", Lang::Toml);

        let mut comment_info = HashMap::new();

        let cpp_style_comment = {
            let single_line = Arc::new(vec!["//"]);
            let multi_line_start = Arc::new(vec!["/*"]);
            let multi_line_end = Arc::new(vec!["*/"]);
            CommentInfo {
                single_line,
                multi_line_start,
                multi_line_end,
            }
        };

        comment_info.insert(Lang::C, cpp_style_comment.clone());
        comment_info.insert(Lang::CHeader, cpp_style_comment.clone());
        comment_info.insert(Lang::Cpp, cpp_style_comment.clone());
        comment_info.insert(Lang::CppHeader, cpp_style_comment.clone());
        comment_info.insert(Lang::Markdown, cpp_style_comment.clone());
        comment_info.insert(Lang::Rust, cpp_style_comment.clone());
        comment_info.insert(Lang::Toml, cpp_style_comment.clone());

        Self {
            jobpool,
            extensions,
            ignore_files: HashSet::new(),
            comment_info,
        }
    }

    pub fn scan(&mut self, args: HashSet<String>) -> Vec<SlocStr> {
        let (tx, rx): (Sender<Sloc>, Receiver<Sloc>) = mpsc::channel();
        let mut count = 0;
        for a in args.iter() {
            for entry in WalkDir::new(a)
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if let Some(ext) = ext.to_str() {
                        if let Some(lang) = self.extensions.get(&ext) {
                            count += 1;
                            let tx = tx.clone();
                            let comment_info = self.comment_info.get(lang).unwrap();
                            let counter = Counter::new(
                                path.to_path_buf(),
                                lang.clone(),
                                comment_info.clone(),
                            );
                            self.jobpool.queue(move || {
                                if let Some(sloc) = counter.count() {
                                    tx.send(sloc).unwrap();
                                }
                            });
                        }
                    }
                }
            }
        }

        let mut sloc_map: HashMap<Lang, Sloc> = HashMap::new();
        for _ in 0..count {
            let sloc = rx.recv().unwrap();
            match sloc_map.entry(sloc.lang.clone()) {
                Entry::Occupied(ref mut e) => {
                    let mut s = e.get_mut();
                    s.files += sloc.files;
                    s.lines += sloc.lines;
                    s.comments += sloc.comments;
                    s.blanks += sloc.blanks;
                }
                Entry::Vacant(e) => {
                    e.insert(sloc);
                }
            }
        }

        let mut sloc: Vec<_> = sloc_map.iter().map(|(_, v)| v.clone()).collect();
        sloc.sort_by(|a, b| a.lines.cmp(&b.lines).reverse());
        let mut total = Sloc::new(Lang::Total);
        for s in &sloc {
            total.files += s.files;
            total.lines += s.lines;
            total.comments += s.comments;
            total.blanks += s.blanks;
        }
        sloc.push(total);

        sloc.iter()
            .map(|s| SlocStr {
                lang: format!("{}", s.lang),
                files: format!("{}", s.files),
                lines: format!("{}", s.lines),
                code: format!("{}", (s.lines - s.comments - s.blanks)),
                comments: format!("{}", s.comments),
                blanks: format!("{}", s.blanks),
            })
            .collect()
    }

    pub fn ignore_file(&mut self, file_name: &str) {
        self.ignore_files.insert(file_name.into());
    }
}
