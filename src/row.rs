use derivative::Derivative;
use std::cmp;

#[derive(Derivative)]
#[derivative(Default)]
pub struct Row {
    string: String,
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        self.string.get(start..end).unwrap_or_default().to_string()
    }
}

impl From<&str> for Row {
    fn from(input: &str) -> Self {
        Self {
            string: input.into(),
        }
    }
}
