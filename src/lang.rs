use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Lang {
    CHeader,
    C,
    CppHeader,
    Cpp,
    Rust,
    Toml,
    Markdown,

    Total,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Lang::*;
        match *self {
            CHeader => write!(f, "C Header"),
            C => write!(f, "C"),
            CppHeader => write!(f, "Cpp Header"),
            Cpp => write!(f, "C++"),
            Rust => write!(f, "Rust"),
            Toml => write!(f, "TOML"),
            Markdown => write!(f, "Markdown"),
            Total => write!(f, "TOTAL"),
        }
    }
}