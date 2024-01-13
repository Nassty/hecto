use crate::editor::Position;
use crate::row::Row;
use derivative::Derivative;
use std::cmp::Ordering;
use std::fs::read_to_string;

#[derive(Derivative)]
#[derivative(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub filename: Option<String>,
}

impl Document {
    pub fn open(fname: &str) -> Self {
        Self {
            rows: read_to_string(fname)
                .expect("File to exist")
                .lines()
                .map(|x| {
                    let mut row = Into::<Row>::into(x);
                    row.highlight();
                    row
                })
                .collect(),
            filename: Some(fname.to_owned()),
        }
    }
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    pub fn insert(&mut self, at: &Position, c: char) {
        match at.y.cmp(&self.len()) {
            Ordering::Equal => {
                let mut k = Row::from(c);
                k.highlight();
                self.rows.push(k);
            }
            Ordering::Less => {
                let row = self.rows.get_mut(at.y).unwrap();
                row.insert(at.x, c);
                row.highlight();
            }
            Ordering::Greater => unreachable!(),
        }
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
        let mut new_row = current_row.split(at.x);
        current_row.highlight();
        new_row.highlight();
        self.rows.insert(at.y + 1, new_row);
    }
    pub fn delete(&mut self, at: &Position) {
        if at.y >= self.len() {
            return;
        }
        let row = self.rows.get_mut(at.y).unwrap();
        if !row.is_empty() {
            row.delete(at.x);
            row.highlight();
            return;
        }
        self.rows.remove(at.y);
    }
}
