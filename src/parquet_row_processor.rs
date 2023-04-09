use std::error::Error;
use tokio_postgres::{Column, Row};
use crate::logging::init;
use crate::postquet_engine::RowProcessor;
use std::sync::{Mutex, Once};

pub struct ParquetRowProcessor {
    init_once: Once,
}

impl ParquetRowProcessor {
    pub fn new() -> Self {
        Self {
            init_once: Once::new(),
        }
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

        let mut init_result = Ok(());
        self.init_once.call_once(|| init_result = self.init(&row.columns()));
        if init_result.is_err() {
            return init_result;
        }

        println!("process_row {:?}", row);
        Ok(())
    }
}