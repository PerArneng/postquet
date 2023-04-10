use std::error::Error;
use tokio_postgres::{Column, Row};
use crate::logging::init;
use crate::postquet_engine::RowProcessor;
use std::sync::{Mutex, Once};
use arrow::datatypes::{DataType, Field, Schema, TimeUnit};
use chrono::{DateTime, Local, NaiveDateTime, NaiveTime, TimeZone, Utc};
use futures_util::FutureExt;
use log::error;
use tokio_postgres::types::{FromSql, Type};

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
        let schema = create_parquet_schema(&columns)?;
        println!("schema: {:?}", schema);
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
        for col_index in 0..row.len() {
            let column = row.columns().get(col_index).ok_or("column not found")?;
            let col_type = column.type_();

            if col_type == &Type::INT8 {
                let value:Option<i64> = row.try_get(col_index)?;
                println!("value {}::'{}' i64: {:?}", col_type, column.name() ,value);
            } else if col_type == &Type::INT4 {
                let value:Option<i32> = row.try_get(col_index)?;
                println!("value {}::'{}' i32: {:?}", col_type, column.name() ,value);
            } else if col_type == &Type::VARCHAR {
                let value: Option<String> = row.try_get(col_index)?;
                println!("value {}::'{}' String: {:?}", col_type, column.name(), value);
            } else if col_type == &Type::FLOAT8 {
                    let value:Option<f64> = row.try_get(col_index)?;
                    println!("value {}::'{}' f64: {:?}", col_type, column.name() ,value);
            } else if col_type == &Type::TIMESTAMP {
                let value:Option<NaiveDateTime> = row.try_get(col_index)?;
                println!("value {}::'{}' NaiveDateTime: {:?}", col_type, column.name() ,value);
            } else {
                return Err(format!("unsupported type '{}' for column '{}'", col_type, column.name()).into());
            }

            //let value: Result<String, _> = row.get(x);
            //let value: dyn FromSql = row.try_get(x)
                                   // .map(|v| v.to_string())?;
            //println!("column: {:?}, value: {}", column, &value);
        }
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

pub fn create_parquet_schema(columns: &[Column]) -> Result<Schema, Box<dyn std::error::Error>> {
    let mut fields = Vec::new();

    for column in columns {
        let arrow_type= to_arrow_type(&column.type_())
            .ok_or(format!("unsupported postgresql type {:?} for column {}",
                           column.type_(), column.name()))?;
        let field = Field::new(column.name(), arrow_type, true);
        fields.push(field);
    }

    let schema = Schema::new(fields);
    Ok(schema)
}