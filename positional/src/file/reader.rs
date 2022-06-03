use crate::{FromPositionalRow, PositionalResult};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::marker::PhantomData;
use std::path::Path;

pub struct Reader<T> {
    buf_reader: Lines<BufReader<File>>,
    row_type: PhantomData<T>,
}

impl<T> Reader<T> {
    pub fn from_path<P: AsRef<Path>>(path: P) -> PositionalResult<Self> {
        let file = File::open(path.as_ref())?;
        let buf_reader = BufReader::new(file).lines();
        Ok(Self {
            buf_reader,
            row_type: PhantomData::default(),
        })
    }
}

impl<T: FromPositionalRow> Iterator for Reader<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buf_reader.next() {
            None => None,
            Some(line) => Some(T::parse(line.ok()?).ok()?),
        }
    }
}
