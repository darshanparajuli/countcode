use std::fmt;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Clone)]
pub struct CommentInfo {
    pub single_line: Arc<[&'static str]>,
    pub multi_line_start: Arc<[&'static str]>,
    pub multi_line_end: Arc<[&'static str]>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Lang {
    C,
    CHeader,
    Cpp,
    CppHeader,
    Css,
    Go,
    Html,
    Java,
    JavaScript,
    Kotlin,
    Markdown,
    Python,
    Rust,
    Toml,
    TypeScript,

    Total,
}

impl Lang {
    pub fn extensions() -> HashMap<&'static str, Lang> {
        let mut extensions = HashMap::new();

        extensions.insert("c", Lang::C);
        extensions.insert("c++", Lang::Cpp);
        extensions.insert("cc", Lang::Cpp);
        extensions.insert("cpp", Lang::Cpp);
        extensions.insert("css", Lang::Css);
        extensions.insert("cxx", Lang::Cpp);
        extensions.insert("go", Lang::Go);
        extensions.insert("h", Lang::CHeader);
        extensions.insert("h++", Lang::CppHeader);
        extensions.insert("hh", Lang::CppHeader);
        extensions.insert("hpp", Lang::CppHeader);
        extensions.insert("htm", Lang::Html);
        extensions.insert("html", Lang::Html);
        extensions.insert("hxx", Lang::CppHeader);
        extensions.insert("java", Lang::Java);
        extensions.insert("js", Lang::JavaScript);
        extensions.insert("kt", Lang::Kotlin);
        extensions.insert("md", Lang::Markdown);
        extensions.insert("py", Lang::Python);
        extensions.insert("py3", Lang::Python);
        extensions.insert("rs", Lang::Rust);
        extensions.insert("toml", Lang::Toml);
        extensions.insert("ts", Lang::TypeScript);

        extensions
    }

    pub fn comment_info() -> HashMap<Lang, CommentInfo> {
        let mut comment_info = HashMap::new();

        let cpp_style_comment = {
            let single_line = Arc::new(["//"]);
            let multi_line_start = Arc::new(["/*"]);
            let multi_line_end = Arc::new(["*/"]);
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
        comment_info.insert(Lang::Css, cpp_style_comment.clone());
        comment_info.insert(Lang::Go, cpp_style_comment.clone());
        comment_info.insert(Lang::Java, cpp_style_comment.clone());
        comment_info.insert(Lang::JavaScript, cpp_style_comment.clone());
        comment_info.insert(Lang::Kotlin, cpp_style_comment.clone());
        comment_info.insert(Lang::Markdown, cpp_style_comment.clone());
        comment_info.insert(Lang::Rust, cpp_style_comment.clone());
        comment_info.insert(Lang::Toml, cpp_style_comment.clone());
        comment_info.insert(Lang::TypeScript, cpp_style_comment.clone());

        let py_style_comment = {
            let single_line = Arc::new(["#"]);
            let multi_line_start = Arc::new(["\"\"\""]);
            let multi_line_end = Arc::new(["\"\"\""]);
            CommentInfo {
                single_line,
                multi_line_start,
                multi_line_end,
            }
        };
        comment_info.insert(Lang::Python, py_style_comment);

        let html_style_comment = {
            let single_line = Arc::new([]);
            let multi_line_start = Arc::new(["<!--"]);
            let multi_line_end = Arc::new(["-->"]);
            CommentInfo {
                single_line,
                multi_line_start,
                multi_line_end,
            }
        };
        comment_info.insert(Lang::Html, html_style_comment);

        comment_info
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Lang::*;
        match *self {
            C => write!(f, "C"),
            CHeader => write!(f, "C Header"),
            Cpp => write!(f, "C++"),
            CppHeader => write!(f, "Cpp Header"),
            Css => write!(f, "CSS"),
            Go => write!(f, "Go"),
            Html => write!(f, "HTML"),
            Java => write!(f, "Java"),
            JavaScript => write!(f, "JavaScript"),
            Kotlin => write!(f, "Kotlin"),
            Markdown => write!(f, "Markdown"),
            Python => write!(f, "Python"),
            Rust => write!(f, "Rust"),
            Toml => write!(f, "TOML"),
            TypeScript => write!(f, "TypeScript"),

            Total => write!(f, "TOTAL"),
        }
    }
}
