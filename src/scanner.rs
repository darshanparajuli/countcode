use std::collections::HashSet;
use std::collections::hash_map::{Entry, HashMap};
use counter::{Counter, Sloc, SlocStr};
use lang::{CommentInfo, Lang};
use walkdir::WalkDir;
use rayon::prelude::*;

pub struct Scanner {
    extensions: HashMap<&'static str, Lang>,
    comment_info: HashMap<Lang, CommentInfo>,
    ignore_files: HashSet<String>,
}

impl Scanner {
    pub fn new() -> Self {
        let extensions = Lang::extensions();
        let comment_info = Lang::comment_info();
        Self {
            extensions,
            ignore_files: HashSet::new(),
            comment_info,
        }
    }

    pub fn scan(&mut self, args: HashSet<String>) -> Vec<SlocStr> {
        let mut paths = Vec::new();
        for a in args.iter() {
            for entry in WalkDir::new(a)
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                paths.push(entry);
            }
        }

        let extensions = &self.extensions;
        let comment_info = &self.comment_info;
        let count_result: Vec<Sloc> = paths
            .par_iter()
            .filter_map(|entry| {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if let Some(ext) = ext.to_str() {
                        if let Some(lang) = extensions.get(&ext) {
                            let comment_info = comment_info.get(lang).unwrap();
                            let counter = Counter::new(
                                path.to_path_buf(),
                                lang.clone(),
                                comment_info.clone(),
                            );
                            return counter.count();
                        }
                    }
                }

                None
            })
            .collect();

        let mut sloc_map: HashMap<Lang, Sloc> = HashMap::new();
        for sloc in count_result {
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
