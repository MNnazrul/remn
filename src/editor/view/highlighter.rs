use crossterm::style::Color;

#[derive(Clone, Copy, PartialEq)]
pub enum TokenType {
    Keyword,
    Type,
    String,
    Comment,
    Number,
    Function,
    Macro,
}

impl TokenType {
    pub const fn color(self) -> Color {
        match self {
            Self::Keyword => Color::Magenta,
            Self::Type => Color::Cyan,
            Self::String => Color::Green,
            Self::Comment => Color::DarkGrey,
            Self::Number => Color::Yellow,
            Self::Function => Color::Blue,
            Self::Macro => Color::Red,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    C,
    Go,
    Plain,
}

impl Language {
    pub fn from_extension(ext: &str) -> Self {
        match ext {
            "rs" => Self::Rust,
            "py" => Self::Python,
            "js" | "ts" | "jsx" | "tsx" => Self::JavaScript,
            "c" | "h" | "cpp" | "hpp" | "cc" | "cxx" => Self::C,
            "go" => Self::Go,
            _ => Self::Plain,
        }
    }

    fn keywords(self) -> &'static [&'static str] {
        match self {
            Self::Rust => &[
                "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else",
                "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop",
                "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static",
                "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while",
                "yield",
            ],
            Self::Python => &[
                "and", "as", "assert", "async", "await", "break", "class", "continue", "def",
                "del", "elif", "else", "except", "False", "finally", "for", "from", "global",
                "if", "import", "in", "is", "lambda", "None", "not", "or", "pass", "raise",
                "return", "True", "try", "while", "with", "yield",
            ],
            Self::JavaScript => &[
                "async", "await", "break", "case", "catch", "class", "const", "continue",
                "debugger", "default", "delete", "do", "else", "export", "extends", "false",
                "finally", "for", "function", "if", "import", "in", "instanceof", "let", "new",
                "null", "of", "return", "switch", "this", "throw", "true", "try", "typeof",
                "undefined", "var", "void", "while", "yield",
            ],
            Self::C => &[
                "auto", "break", "case", "char", "const", "continue", "default", "do", "double",
                "else", "enum", "extern", "float", "for", "goto", "if", "inline", "int", "long",
                "register", "return", "short", "signed", "sizeof", "static", "struct", "switch",
                "typedef", "union", "unsigned", "void", "volatile", "while",
                "#include", "#define", "#ifdef", "#ifndef", "#endif", "#pragma",
            ],
            Self::Go => &[
                "break", "case", "chan", "const", "continue", "default", "defer", "else",
                "fallthrough", "for", "func", "go", "goto", "if", "import", "interface", "map",
                "package", "range", "return", "select", "struct", "switch", "type", "var",
            ],
            Self::Plain => &[],
        }
    }

    fn types(self) -> &'static [&'static str] {
        match self {
            Self::Rust => &[
                "bool", "char", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize",
                "str", "u8", "u16", "u32", "u64", "u128", "usize", "String", "Vec", "Box",
                "Option", "Result", "Some", "None", "Ok", "Err", "HashMap", "HashSet",
            ],
            Self::Python => &[
                "int", "float", "str", "bool", "list", "dict", "set", "tuple", "bytes",
                "None", "object", "type",
            ],
            Self::JavaScript => &[
                "Array", "Boolean", "Date", "Error", "Function", "Map", "Number", "Object",
                "Promise", "RegExp", "Set", "String", "Symbol",
            ],
            Self::C => &[
                "int8_t", "int16_t", "int32_t", "int64_t", "uint8_t", "uint16_t", "uint32_t",
                "uint64_t", "size_t", "ssize_t", "bool", "FILE", "NULL",
            ],
            Self::Go => &[
                "bool", "byte", "complex64", "complex128", "error", "float32", "float64",
                "int", "int8", "int16", "int32", "int64", "rune", "string",
                "uint", "uint8", "uint16", "uint32", "uint64", "uintptr", "nil", "true", "false",
            ],
            Self::Plain => &[],
        }
    }

    fn line_comment(self) -> &'static str {
        match self {
            Self::Rust | Self::JavaScript | Self::C | Self::Go => "//",
            Self::Python => "#",
            Self::Plain => "",
        }
    }
}

pub struct Highlighter {
    language: Language,
}

impl Highlighter {
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    pub fn highlight_line(&self, line: &str) -> String {
        if self.language == Language::Plain {
            return line.to_string();
        }

        let comment_prefix = self.language.line_comment();

        // Check for line comment
        let trimmed = line.trim_start();
        if !comment_prefix.is_empty() && trimmed.starts_with(comment_prefix) {
            return format!(
                "\x1b[{}m{}\x1b[0m",
                color_to_ansi(TokenType::Comment.color()),
                line
            );
        }

        let mut result = String::with_capacity(line.len() * 2);
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            let ch = chars[i];

            // Strings (double and single quotes)
            if ch == '"' || ch == '\'' {
                let quote = ch;
                let start = i;
                i += 1;
                while i < len && chars[i] != quote {
                    if chars[i] == '\\' {
                        i += 1; // skip escaped char
                    }
                    i += 1;
                }
                if i < len {
                    i += 1; // closing quote
                }
                let s: String = chars[start..i].iter().collect();
                result.push_str(&format!(
                    "\x1b[{}m{}\x1b[0m",
                    color_to_ansi(TokenType::String.color()),
                    s
                ));
                continue;
            }

            // Inline comment
            if !comment_prefix.is_empty() && i + comment_prefix.len() <= len {
                let slice: String = chars[i..i + comment_prefix.len()].iter().collect();
                if slice == comment_prefix {
                    let rest: String = chars[i..].iter().collect();
                    result.push_str(&format!(
                        "\x1b[{}m{}\x1b[0m",
                        color_to_ansi(TokenType::Comment.color()),
                        rest
                    ));
                    return result;
                }
            }

            // Numbers
            if ch.is_ascii_digit() && (i == 0 || !chars[i - 1].is_alphanumeric()) {
                let start = i;
                while i < len && (chars[i].is_ascii_digit() || chars[i] == '.' || chars[i] == '_'
                    || chars[i] == 'x' || chars[i] == 'b' || chars[i] == 'o'
                    || (chars[i].is_ascii_hexdigit() && start < i))
                {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                result.push_str(&format!(
                    "\x1b[{}m{}\x1b[0m",
                    color_to_ansi(TokenType::Number.color()),
                    s
                ));
                continue;
            }

            // Words (identifiers, keywords, types)
            if ch.is_alphabetic() || ch == '_' || ch == '#' {
                let start = i;
                i += 1;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();

                // Check for macro (word followed by !)
                if self.language == Language::Rust && i < len && chars[i] == '!' {
                    i += 1;
                    result.push_str(&format!(
                        "\x1b[{}m{}!\x1b[0m",
                        color_to_ansi(TokenType::Macro.color()),
                        word
                    ));
                    continue;
                }

                // Check for function call (word followed by '(')
                if i < len && chars[i] == '(' {
                    let token = if self.language.keywords().contains(&word.as_str()) {
                        TokenType::Keyword
                    } else {
                        TokenType::Function
                    };
                    result.push_str(&format!(
                        "\x1b[{}m{}\x1b[0m",
                        color_to_ansi(token.color()),
                        word
                    ));
                    continue;
                }

                if self.language.keywords().contains(&word.as_str()) {
                    result.push_str(&format!(
                        "\x1b[{}m{}\x1b[0m",
                        color_to_ansi(TokenType::Keyword.color()),
                        word
                    ));
                } else if self.language.types().contains(&word.as_str()) {
                    result.push_str(&format!(
                        "\x1b[{}m{}\x1b[0m",
                        color_to_ansi(TokenType::Type.color()),
                        word
                    ));
                } else {
                    result.push_str(&word);
                }
                continue;
            }

            result.push(ch);
            i += 1;
        }

        result
    }
}

fn color_to_ansi(color: Color) -> String {
    match color {
        Color::Reset => "0".to_string(),
        Color::Red => "31".to_string(),
        Color::Green => "32".to_string(),
        Color::Yellow => "33".to_string(),
        Color::Blue => "34".to_string(),
        Color::Magenta => "35".to_string(),
        Color::Cyan => "36".to_string(),
        Color::DarkGrey => "90".to_string(),
        _ => "0".to_string(),
    }
}
