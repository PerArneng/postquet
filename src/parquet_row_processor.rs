use std::error::Error;
use tokio_postgres::{Column, Row};
use crate::logging::init;
use crate::postquet_engine::RowProcessor;
use std::sync::{Mutex, Once};
use arrow::datatypes::{DataType, TimeUnit};
use log::error;
use tokio_postgres::types::Type;

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
        //println!("init {:?}", columns);
        for column in columns {
            //println!("column {:?}", column);

            let arrow_type= to_arrow_type(&column.type_())
                .ok_or(format!("unsupported postgresql type {:?} for column {}. sorry!",
                               column.type_(), column.name()))?;

            //println!("type {:?}", column.type_());
            //println!("type name {:?}", column.type_().name());
            //println!("arrow type {:?}", arrow_type);
        }
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

fn to_arrow_type(t: &Type) -> Option<DataType> {
    match t {
        &Type::BOOL => Some(DataType::Boolean),
        &Type::CHAR => Some(DataType::Int8),
        &Type::INT2 => Some(DataType::Int16),
        &Type::INT4 => Some(DataType::Int32),
        &Type::INT8 => Some(DataType::Int64),
        &Type::FLOAT4 => Some(DataType::Float32),
        &Type::FLOAT8 => Some(DataType::Float64),
        &Type::TEXT | &Type::VARCHAR | &Type::BPCHAR => Some(DataType::Utf8),
        &Type::DATE => Some(DataType::Date32),
        &Type::TIMESTAMP => Some(DataType::Timestamp(TimeUnit::Microsecond, None)),
        &Type::TIMESTAMPTZ => Some(DataType::Timestamp(TimeUnit::Microsecond, Some("UTC".to_string()))),
        &Type::BYTEA => Some(DataType::Binary),
        _ => None, // You may add more conversions here or return None for unsupported types
    }
}