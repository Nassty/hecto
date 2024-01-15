use std::cmp;
use synoptic::{languages::rust, Highlighter, Token};
use termion::color;
use unicode_segmentation::UnicodeSegmentation;

pub struct Highlight {
    engine: Highlighter,
    contents: Vec<Vec<Token>>,
    lines: Vec<String>,
}

impl Highlight {
    pub fn new(input: String) -> Self {
        let engine = rust();
        let mut s = Self {
            engine,
            contents: Vec::new(),
            lines: Vec::new(),
        };
        s.parse(input);
        s
    }
    pub fn parse(&mut self, input: String) {
        self.contents = self.engine.run(&input);
        self.lines = self.contents.iter().map(|x| self.parse_line(x)).collect();
    }
    pub fn row(&self, index: usize) -> Option<String> {
        self.lines.get(index).cloned()
    }
    pub fn len(&self, index: usize) -> usize {
        self.row(index)
            .map_or_else(|| 0, |x| x[..].graphemes(true).count())
    }
    pub fn render(&self, index: usize, start: usize, end: usize) -> String {
        let start = cmp::min(start, end);
        let string = self.row(index).unwrap();
        string[..]
            .graphemes(true)
            .skip(start)
            //.take(end - start)
            .map(|x| {
                if matches!(x.chars().next().unwrap(), '\t') {
                    "    ".into()
                } else {
                    x.to_string()
                }
            })
            .collect()
    }
    pub fn parse_line(&self, c: &[Token]) -> String {
        format!(
            "{}{}",
            c.iter()
                .map(|tok| {
                    match tok {
                        // Handle the start token (start foreground colour)
                        Token::Start(kind) => match kind.as_str() {
                            "string" => format!("{}", color::Fg(color::Green)),
                            "number" => format!("{}", color::Fg(color::Blue)),
                            "keyword" => format!("{}", color::Fg(color::Red)),
                            "boolean" => format!("{}", color::Fg(color::LightGreen)),
                            "function" => format!("{}", color::Fg(color::Yellow)),
                            "struct" => format!("{}", color::Fg(color::Cyan)),
                            "macro" => format!("{}", color::Fg(color::Cyan)),
                            "operator" => format!("{}", color::Fg(color::LightWhite)),
                            "namespace" => format!("{}", color::Fg(color::Blue)),
                            "character" => format!("{}", color::Fg(color::Cyan)),
                            "attribute" => format!("{}", color::Fg(color::Blue)),
                            "reference" => format!("{}", color::Fg(color::Magenta)),
                            _ => "".to_string(),
                        },
                        // Handle a text token (print out the contents)
                        Token::Text(txt) => txt.into(),
                        // Handle an end token (reset foreground colour)
                        Token::End(_) => format!("{}", color::Fg(color::Reset)),
                    }
                })
                .collect::<String>(),
            color::Fg(color::Reset)
        )
    }
}
