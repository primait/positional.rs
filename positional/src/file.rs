use crate::row::ToPositionalRow;
use crate::{FromPositionalRow, PositionalError};
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::ops::ControlFlow;
use std::str::FromStr;

pub mod reader;

/// a positional file writer
pub struct Writer<T: ToPositionalRow> {
    rows: Vec<T>,
}

impl<T: ToPositionalRow> Writer<T> {
    pub fn new(rows: impl IntoIterator<Item = T>) -> Self {
        Self {
            rows: rows.into_iter().collect(),
        }
    }
}

impl<T: ToPositionalRow> Display for Writer<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let output: String = self.rows.iter().map(T::to_positional_row).join("\n");
        write!(f, "{}", output)
    }
}

/// a positional file reader
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
