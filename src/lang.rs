use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Lang {
    C,
    CHeader,
    Cpp,
    CppHeader,
    Go,
    Markdown,
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
            Markdown => write!(f, "Markdown"),
            Rust => write!(f, "Rust"),
            Toml => write!(f, "TOML"),

            Total => write!(f, "TOTAL"),
        }
    }
}