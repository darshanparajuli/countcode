extern crate ignore;
extern crate memmap;
extern crate rayon;

mod counter;
mod lang;
mod scanner;

use counter::SlocStr;
use scanner::Scanner;
use std::collections::HashSet;
use std::env;
use std::io::{self, StdoutLock, Write};

fn main() {
    let mut args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        let path = env::current_dir().unwrap();
        args.push(path.to_str().unwrap().into());
    }

    let args: HashSet<_> = args.drain(..).collect();

    let mut scanner = Scanner::new();
    let slocs = scanner.scan(args);
    pretty_print(slocs);
}

fn pretty_print(slocs: Vec<SlocStr>) {
    let mut lang_len = 10;
    let mut files_len = 10;
    let mut lines_len = 10;
    let mut code_len = 10;
    let mut comments_len = 10;
    let mut blanks_len = 10;

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

    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();

    let total_len = lang_len + files_len + lines_len + code_len + comments_len + blanks_len;
    for _ in 0..total_len + 7 {
        print_safe(&mut stdout_handle, "=");
    }
    print_safe(&mut stdout_handle, "\n");

    print_safe(
        &mut stdout_handle,
        &format!(
            " {:<w0$} {:>w1$} {:>w2$} {:>w3$} {:>w4$} {:>w5$} \n",
            "Language",
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
        ),
    );

    for _ in 0..total_len + 7 {
        print_safe(&mut stdout_handle, "-");
    }
    print_safe(&mut stdout_handle, "\n");

    let len = slocs.len();
    for sloc in slocs.iter().take(len - 1) {
        print_safe(
            &mut stdout_handle,
            &format!(
                " {:<w0$} {:>w1$} {:>w2$} {:>w3$} {:>w4$} {:>w5$} \n",
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
            ),
        );
    }

    for _ in 0..total_len + 7 {
        print_safe(&mut stdout_handle, "=");
    }
    print_safe(&mut stdout_handle, "\n");

    let sloc = slocs.get(len - 1).unwrap();
    print_safe(
        &mut stdout_handle,
        &format!(
            " {:<w0$} {:>w1$} {:>w2$} {:>w3$} {:>w4$} {:>w5$} \n",
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
        ),
    );

    for _ in 0..total_len + 7 {
        print_safe(&mut stdout_handle, "=");
    }
    print_safe(&mut stdout_handle, "\n");
}

fn print_safe(handle: &mut StdoutLock, s: &str) {
    match handle.write(s.as_bytes()) {
        Ok(_) => {}
        Err(_) => {
            std::process::exit(0);
        }
    }
}
