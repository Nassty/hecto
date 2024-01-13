use crate::row::Row;
use derivative::Derivative;
use std::fs::read_to_string;

#[derive(Derivative)]
#[derivative(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn open(filename: &str) -> Self {
        let rows = read_to_string(filename)
            .expect("File to exist")
            .lines()
            .map(|x| x.into())
            .collect();
        Self { rows }
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
}
