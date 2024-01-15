use derivative::Derivative;
use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Derivative)]
#[derivative(Default)]
#[derivative(Debug)]
#[derivative(Clone)]
pub struct Row {
    pub string: String,
    len: usize,
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .map(|x| {
                if matches!(x.chars().next().unwrap(), '\t') {
                    "    ".into()
                } else {
                    x.to_string()
                }
            })
            .collect()
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count();
    }
    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
        } else {
            let mut before: String = self.string[..].graphemes(true).take(at).collect();
            let after: String = self.string[..].graphemes(true).skip(at).collect();
            before.push(c);
            before.push_str(&after);
            self.string = before;
        }
        self.update_len();
    }
    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        }
        let mut before: String = self.string[..].graphemes(true).take(at).collect();
        let after: String = self.string[..].graphemes(true).skip(at + 1).collect();
        before.push_str(&after);
        self.string = before;
        self.update_len();
    }
    pub fn split(&mut self, at: usize) -> Self {
        let reminder = self.string[..].graphemes(true).take(at).collect();
        let new_line: String = self.string[..].graphemes(true).skip(at).collect();
        self.string = reminder;

        Self::from(&new_line[..])
    }
}

impl From<char> for Row {
    fn from(input: char) -> Self {
        input.to_string()[..].into()
    }
}

impl From<String> for Row {
    fn from(input: String) -> Self {
        let mut r: Row = Self {
            string: input,
            len: 0,
        };
        r.update_len();
        r
    }
}

impl From<&str> for Row {
    fn from(input: &str) -> Self {
        let mut r = Self {
            string: input.into(),
            len: 0,
        };
        r.update_len();
        r
    }
}
