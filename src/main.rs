extern crate jobpool;
extern crate num_cpus;

mod scanner;
mod counter;

use std::path::PathBuf;
use std::env;
use scanner::Scanner;
use counter::SlocStr;

fn main() {
    let mut paths: Vec<_> = env::args().skip(1).map(|a| PathBuf::from(a)).collect();
    if paths.is_empty() {
        let path = env::current_dir().unwrap();
        paths.push(path.to_str().unwrap().into());
    }

    let mut scanner = Scanner::new();
    scanner.ignore_file(".git");

    let slocs = scanner.scan(paths);
    pretty_print(slocs);
}

fn pretty_print(slocs: Vec<SlocStr>) {
    let mut lang_len = 8;
    let mut files_len = 8;
    let mut lines_len = 8;
    let mut code_len = 8;
    let mut comments_len = 10;
    let mut blanks_len = 6;

    for sloc in &slocs {
        if lang_len < sloc.lang.len() {
            lang_len = sloc.lang.len();
        }
        if files_len < sloc.files.len() {
            files_len = sloc.files.len();
        }
        if lines_len < sloc.lines.len() {
            lines_len = sloc.lines.len();
        }
        if code_len < sloc.code.len() {
            code_len = sloc.code.len();
        }
        if comments_len < sloc.comments.len() {
            comments_len = sloc.comments.len();
        }
        if blanks_len < sloc.blanks.len() {
            blanks_len = sloc.blanks.len();
        }
    }

    lang_len += 1;
    files_len += 1;
    lines_len += 1;
    code_len += 1;
    comments_len += 1;
    blanks_len += 1;

    let total_len = lang_len + files_len + lines_len + code_len + comments_len + blanks_len;
    for _ in 0..total_len + 7 {
        print!("=");
    }
    println!();

    println!(
        " {:<w0$} {:>w1$} {:>w2$} {:>w3$} {:>w4$} {:>w5$} ",
        "Lang",
        "Files",
        "Lines",
        "Code",
        "Comments",
        "Blanks",
        w0 = lang_len,
        w1 = files_len,
        w2 = lines_len,
        w3 = code_len,
        w4 = comments_len,
        w5 = blanks_len,
    );

    for _ in 0..total_len + 7 {
        print!("-");
    }
    println!();

    let len = slocs.len();
    for sloc in slocs.iter().take(len - 1) {
        println!(
            " {:<w0$} {:>w1$} {:>w2$} {:>w3$} {:>w4$} {:>w5$} ",
            sloc.lang,
            sloc.files,
            sloc.lines,
            sloc.code,
            sloc.comments,
            sloc.blanks,
            w0 = lang_len,
            w1 = files_len,
            w2 = lines_len,
            w3 = code_len,
            w4 = comments_len,
            w5 = blanks_len,
        );
    }

    for _ in 0..total_len + 7 {
        print!("-");
    }
    println!();

    let sloc = slocs.get(len - 1).unwrap();
    println!(
        " {:<w0$} {:>w1$} {:>w2$} {:>w3$} {:>w4$} {:>w5$} ",
        sloc.lang,
        sloc.files,
        sloc.lines,
        sloc.code,
        sloc.comments,
        sloc.blanks,
        w0 = lang_len,
        w1 = files_len,
        w2 = lines_len,
        w3 = code_len,
        w4 = comments_len,
        w5 = blanks_len,
    );

    for _ in 0..total_len + 7 {
        print!("=");
    }
    println!();
}
