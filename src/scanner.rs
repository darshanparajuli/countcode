use counter::{Counter, Sloc, SlocStr};
use ignore::Walk;
use lang::{CommentInfo, Lang};
use rayon::prelude::*;
use std::collections::hash_map::{Entry, HashMap};
use std::collections::HashSet;

pub struct Scanner {
    extensions: HashMap<&'static str, Lang>,
    comment_info: HashMap<Lang, CommentInfo>,
}

impl Scanner {
    pub fn new() -> Self {
        let extensions = Lang::extensions();
        let comment_info = Lang::comment_info();
        Self {
            extensions,
            comment_info,
        }
    }

    pub fn scan(&mut self, args: HashSet<String>) -> Vec<SlocStr> {
        let mut paths = Vec::new();
        for a in args.iter() {
            for entry in Walk::new(a).into_iter().filter_map(|e| e.ok()) {
                paths.push(entry);
            }
        }

        let extensions = &self.extensions;
        let comment_info = &self.comment_info;
        let count_result: Vec<Sloc> = paths
            .par_iter()
            .filter_map(|entry| {
                let path = entry.path();
                let lang = {
                    let lang = path
                        .extension()
                        .and_then(|e| e.to_str())
                        .and_then(|e| extensions.get(e));

                    if let Some(lang) = lang {
                        Some(lang)
                    } else {
                        let is_mkfile = path
                            .file_name()
                            .and_then(|e| e.to_str())
                            .map_or(false, |e| e.to_lowercase() == "makefile");

                        if is_mkfile {
                            Some(&Lang::Makefile)
                        } else {
                            None
                        }
                    }
                };

                match lang {
                    Some(lang) => {
                        let comment_info = comment_info.get(lang).unwrap();
                        let mut counter = Counter::new(&path, lang.clone(), comment_info.clone());
                        counter.count()
                    }
                    None => None,
                }
            }).collect();

        let mut sloc_map: HashMap<Lang, Sloc> = HashMap::new();
        for sloc in count_result {
            match sloc_map.entry(sloc.lang.clone()) {
                Entry::Occupied(ref mut e) => {
                    e.get_mut().stats += sloc.stats;
                }
                Entry::Vacant(e) => {
                    e.insert(sloc);
                }
            }
        }

        let mut sloc: Vec<_> = sloc_map.iter().map(|(_, v)| v.clone()).collect();
        sloc.sort_by(|a, b| a.stats.lines.cmp(&b.stats.lines).reverse());
        let mut total = Sloc::new(Lang::Total);
        for s in &sloc {
            total.stats += &s.stats;
        }
        sloc.push(total);

        sloc.iter()
            .map(|s| SlocStr {
                lang: format!("{}", s.lang),
                files: format!("{}", s.stats.files),
                lines: format!("{}", s.stats.lines),
                code: format!("{}", s.stats.code),
                comments: format!("{}", s.stats.comments),
                blanks: format!("{}", s.stats.blanks),
            }).collect()
    }
}
