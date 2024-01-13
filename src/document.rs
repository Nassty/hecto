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
    pub fn open(filename: &str) -> Self {
        let rows = read_to_string(filename)
            .expect("File to exist")
            .lines()
            .map(std::convert::Into::into)
            .collect();
        let filename = Some(filename.to_owned());
        Self { rows, filename }
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
                self.rows.push(Row::from(c));
            }
            Ordering::Less => {
                let row = self.rows.get_mut(at.y).unwrap();
                row.insert(at.x, c);
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
        let new_row = self.rows.get_mut(at.y).unwrap().split(at.x);
        self.rows.insert(at.y + 1, new_row);
    }
    pub fn delete(&mut self, at: &Position) {
        if at.y >= self.len() {
            return;
        }
        let row = self.rows.get_mut(at.y).unwrap();
        if !row.is_empty() {
            row.delete(at.x);
            return;
        }
        self.rows.remove(at.y);
    }
}
