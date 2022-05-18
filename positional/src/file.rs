use crate::row::ToPositionalRow;
use crate::{FromPositionalRow, PositionalError};
use itertools::Itertools;
use std::ops::ControlFlow;
use std::str::FromStr;

/// a positional file is a collection of rows
pub struct Writer<T: ToPositionalRow> {
    rows: Vec<T>,
}

impl<T: ToPositionalRow> Writer<T> {
    pub fn new(rows: impl IntoIterator<Item = T>) -> Self {
        Self {
            rows: rows.into_iter().collect(),
        }
    }

    pub fn to_string(&self) -> String {
        let rows = self.rows.iter().map(|row| row.to_positional_row());
        Itertools::intersperse(rows, "\n".to_string()).collect()
    }
}

pub struct Reader<T: FromPositionalRow> {
    rows: Vec<T>,
}

impl<T: FromPositionalRow> FromStr for Reader<T> {
    type Err = PositionalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.lines().try_fold(vec![], |mut acc, line| {
            match FromPositionalRow::parse(line) {
                Ok(row) => {
                    acc.push(row);
                    ControlFlow::Continue(acc)
                }
                Err(error) => ControlFlow::Break(error),
            }
        });
        match rows {
            ControlFlow::Continue(rows) => Ok(Self { rows }),
            ControlFlow::Break(_) => Err(PositionalError::UnparsableFile),
        }
    }
}

impl<T: FromPositionalRow> IntoIterator for Reader<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}
