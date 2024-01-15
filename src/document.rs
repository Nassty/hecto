use crate::editor::Position;
use crate::highlight::Highlight;
use crate::row::Row;
use derivative::Derivative;
use std::cmp::Ordering;
use std::fs::read_to_string;

#[derive(Derivative)]
#[derivative(Default)]
pub struct Document {
    rows: Vec<Row>,
    #[derivative(Default(value = "Highlight::new(\"\".to_string())"))]
    highlight: Highlight,
    pub filename: Option<String>,
}

impl Document {
    pub fn open(fname: &str) -> Self {
        let fcontents = read_to_string(fname).expect("File to exist");
        let mut s = Self {
            rows: fcontents.lines().map(Into::<Row>::into).collect(),
            highlight: Highlight::new(fcontents.to_string()),
            filename: Some(fname.to_owned()),
        };
        s.highlight();
        s
    }
    pub fn render(&self, start: usize, end: usize, lineno: usize) -> String {
        self.highlight.render(start, end, lineno)
    }
    pub fn row(&self, index: usize, highlight: bool) -> Option<Row> {
        if !highlight {
            if let Some(x) = self.rows.get(index) {
                return Some(x.clone());
            } else {
                return None;
            }
        }
        if let Some(x) = self.highlight.row(index) {
            Some(x.into())
        } else {
            None
        }
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    pub fn row_len(&self, index: usize) -> usize {
        self.highlight.len(index)
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    pub fn insert(&mut self, at: &Position, c: char) {
        match at.y.cmp(&self.len()) {
            Ordering::Equal => self.rows.push(c.into()),
            Ordering::Less => self.rows.get_mut(at.y).unwrap().insert(at.x, c),
            Ordering::Greater => unreachable!(),
        }
        self.highlight();
    }
    pub fn insert_nl(&mut self, at: &Position) {
        if at.y > self.len() {
            return;
        }
        if at.y == self.len() {
            self.rows.push(Row::default());
            return;
        }
        let current_row = &mut self.rows[at.y];
        let new_row = current_row.split(at.x);
        self.highlight();
        self.rows.insert(at.y + 1, new_row);
    }
    pub fn delete(&mut self, at: &Position) {
        if at.y >= self.len() {
            return;
        }
        let row = self.rows.get_mut(at.y).unwrap();
        if !row.is_empty() {
            row.delete(at.x);
            self.highlight();
            return;
        }
        self.rows.remove(at.y);
    }
    pub fn highlight(&mut self) {
        let contents = self
            .rows
            .iter()
            .map(|x| x.string.clone())
            .collect::<Vec<String>>();
        self.highlight.parse(contents.join("\n"));
    }
}
