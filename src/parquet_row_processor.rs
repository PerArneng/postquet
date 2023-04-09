use std::error::Error;
use tokio_postgres::{Column, Row};
use crate::logging::init;
use crate::postquet_engine::RowProcessor;

pub struct ParquetRowProcessor {

}

impl ParquetRowProcessor {
    pub fn new() -> Self {
        ParquetRowProcessor {}
    }
}

impl ParquetRowProcessor  {
    fn init(&self, columns: &[Column]) -> Result<(), Box<dyn Error>> {
        println!("init {:?}", columns);
        Ok(())
    }
}

impl RowProcessor for ParquetRowProcessor  {

    fn process_row(&self, row: &Row) -> Result<(), Box<dyn Error>> {
        self.init(&row.columns())?;
        println!("process_row {:?}", row);
        Ok(())
    }
}