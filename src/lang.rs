use std::fmt;
use std::sync::Arc;

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
    Go,
    Java,
    Kotlin,
    Markdown,
    Python,
    Rust,
    Toml,

    Total,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Lang::*;
        match *self {
            C => write!(f, "C"),
            CHeader => write!(f, "C Header"),
            Cpp => write!(f, "C++"),
            CppHeader => write!(f, "Cpp Header"),
            Go => write!(f, "Go"),
            Java => write!(f, "Java"),
            Kotlin => write!(f, "Kotlin"),
            Markdown => write!(f, "Markdown"),
            Python => write!(f, "Python"),
            Rust => write!(f, "Rust"),
            Toml => write!(f, "TOML"),

            Total => write!(f, "TOTAL"),
        }
    }
}
