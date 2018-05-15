use std::collections::HashMap;
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
    Asm,
    C,
    CHeader,
    Cpp,
    CSharp,
    CppHeader,
    Css,
    D,
    FSharp,
    Go,
    Haskell,
    Html,
    Java,
    JavaScript,
    Json,
    Kotlin,
    Lua,
    Makefile,
    Markdown,
    Perl,
    PlainText,
    Python,
    Ruby,
    Rust,
    Toml,
    TypeScript,
    Xml,
    Yaml,

    Total,
}

impl Lang {
    pub fn extensions() -> HashMap<&'static str, Lang> {
        let mut extensions = HashMap::new();

        extensions.insert("s", Lang::Asm);
        extensions.insert("S", Lang::Asm);
        extensions.insert("asm", Lang::Asm);
        extensions.insert("C", Lang::Cpp);
        extensions.insert("c", Lang::C);
        extensions.insert("c++", Lang::Cpp);
        extensions.insert("cc", Lang::Cpp);
        extensions.insert("cpp", Lang::Cpp);
        extensions.insert("cs", Lang::CSharp);
        extensions.insert("css", Lang::Css);
        extensions.insert("cxx", Lang::Cpp);
        extensions.insert("d", Lang::D);
        extensions.insert("di", Lang::D);
        extensions.insert("fs", Lang::FSharp);
        extensions.insert("go", Lang::Go);
        extensions.insert("h", Lang::CHeader);
        extensions.insert("h++", Lang::CppHeader);
        extensions.insert("hh", Lang::CppHeader);
        extensions.insert("hpp", Lang::CppHeader);
        extensions.insert("hs", Lang::Haskell);
        extensions.insert("htm", Lang::Html);
        extensions.insert("html", Lang::Html);
        extensions.insert("hxx", Lang::CppHeader);
        extensions.insert("java", Lang::Java);
        extensions.insert("js", Lang::JavaScript);
        extensions.insert("json", Lang::Json);
        extensions.insert("kt", Lang::Kotlin);
        extensions.insert("lua", Lang::Lua);
        extensions.insert("mk", Lang::Makefile);
        extensions.insert("md", Lang::Markdown);
        extensions.insert("pl", Lang::Perl);
        extensions.insert("py", Lang::Python);
        extensions.insert("py3", Lang::Python);
        extensions.insert("rb", Lang::Ruby);
        extensions.insert("rs", Lang::Rust);
        extensions.insert("toml", Lang::Toml);
        extensions.insert("ts", Lang::TypeScript);
        extensions.insert("txt", Lang::PlainText);
        extensions.insert("xml", Lang::Xml);
        extensions.insert("yaml", Lang::Yaml);
        extensions.insert("yml", Lang::Yaml);

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
        comment_info.insert(Lang::CSharp, cpp_style_comment.clone());
        comment_info.insert(Lang::Cpp, cpp_style_comment.clone());
        comment_info.insert(Lang::CppHeader, cpp_style_comment.clone());
        comment_info.insert(Lang::Css, cpp_style_comment.clone());
        comment_info.insert(Lang::D, cpp_style_comment.clone());
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
        comment_info.insert(Lang::Html, html_style_comment.clone());
        comment_info.insert(Lang::Xml, html_style_comment);

        let ruby_style_comment = {
            let single_line = Arc::new(["#"]);
            let multi_line_start = Arc::new(["=begin"]);
            let multi_line_end = Arc::new(["=end"]);
            CommentInfo {
                single_line,
                multi_line_start,
                multi_line_end,
            }
        };
        comment_info.insert(Lang::Ruby, ruby_style_comment);

        let perl_style_comment = {
            let single_line = Arc::new(["#"]);
            let multi_line_start = Arc::new(["=begin"]);
            let multi_line_end = Arc::new(["=cut"]);
            CommentInfo {
                single_line,
                multi_line_start,
                multi_line_end,
            }
        };
        comment_info.insert(Lang::Perl, perl_style_comment);

        let yaml_style_comment = {
            let single_line = Arc::new(["#"]);
            let multi_line_start = Arc::new([]);
            let multi_line_end = Arc::new([]);
            CommentInfo {
                single_line,
                multi_line_start,
                multi_line_end,
            }
        };
        comment_info.insert(Lang::Yaml, yaml_style_comment.clone());
        comment_info.insert(Lang::Makefile, yaml_style_comment);

        let haskell_style_comment = {
            let single_line = Arc::new(["--"]);
            let multi_line_start = Arc::new(["{-"]);
            let multi_line_end = Arc::new(["-}"]);
            CommentInfo {
                single_line,
                multi_line_start,
                multi_line_end,
            }
        };
        comment_info.insert(Lang::Haskell, haskell_style_comment);

        let lua_style_comment = {
            let single_line = Arc::new(["--"]);
            let multi_line_start = Arc::new(["--[["]);
            let multi_line_end = Arc::new(["--]]"]);
            CommentInfo {
                single_line,
                multi_line_start,
                multi_line_end,
            }
        };
        comment_info.insert(Lang::Lua, lua_style_comment);

        let json_style_comment = {
            let single_line = Arc::new([]);
            let multi_line_start = Arc::new([]);
            let multi_line_end = Arc::new([]);
            CommentInfo {
                single_line,
                multi_line_start,
                multi_line_end,
            }
        };
        comment_info.insert(Lang::Json, json_style_comment);

        let plain_txt_style_comment = {
            let single_line = Arc::new([]);
            let multi_line_start = Arc::new([]);
            let multi_line_end = Arc::new([]);
            CommentInfo {
                single_line,
                multi_line_start,
                multi_line_end,
            }
        };
        comment_info.insert(Lang::PlainText, plain_txt_style_comment);

        let fs_style_comment = {
            let single_line = Arc::new(["//"]);
            let multi_line_start = Arc::new(["(*"]);
            let multi_line_end = Arc::new(["*)"]);
            CommentInfo {
                single_line,
                multi_line_start,
                multi_line_end,
            }
        };
        comment_info.insert(Lang::FSharp, fs_style_comment);

        let asm_style_comment = {
            let single_line = Arc::new([";", "#", "@", "//"]);
            let multi_line_start = Arc::new(["/*"]);
            let multi_line_end = Arc::new(["*/"]);
            CommentInfo {
                single_line,
                multi_line_start,
                multi_line_end,
            }
        };
        comment_info.insert(Lang::Asm, asm_style_comment);

        comment_info
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Lang::*;
        match *self {
            Asm => write!(f, "Assembly"),
            C => write!(f, "C"),
            CHeader => write!(f, "C Header"),
            CSharp => write!(f, "C#"),
            Cpp => write!(f, "C++"),
            CppHeader => write!(f, "C++ Header"),
            Css => write!(f, "CSS"),
            D => write!(f, "D"),
            FSharp => write!(f, "F#"),
            Go => write!(f, "Go"),
            Haskell => write!(f, "Haskell"),
            Html => write!(f, "HTML"),
            Java => write!(f, "Java"),
            JavaScript => write!(f, "JavaScript"),
            Json => write!(f, "JSON"),
            Kotlin => write!(f, "Kotlin"),
            Lua => write!(f, "Lua"),
            Makefile => write!(f, "Makefile"),
            Markdown => write!(f, "Markdown"),
            Perl => write!(f, "Perl"),
            PlainText => write!(f, "Plain Text"),
            Python => write!(f, "Python"),
            Ruby => write!(f, "Ruby"),
            Rust => write!(f, "Rust"),
            Toml => write!(f, "TOML"),
            TypeScript => write!(f, "TypeScript"),
            Xml => write!(f, "XML"),
            Yaml => write!(f, "YAML"),

            Total => write!(f, "TOTAL"),
        }
    }
}
