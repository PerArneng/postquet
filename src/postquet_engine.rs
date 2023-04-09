use tokio_postgres::{NoTls, Row, RowStream, Statement};
use tokio_postgres::types::ToSql;
use futures_util::{pin_mut, TryStreamExt};
use tokio_postgres::types::Timestamp;
use chrono::NaiveDateTime;
use itertools::join;

pub struct ConnectionInfo {
    pub hostname: String,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub database: String,
}

pub trait TableProcessor {
    type ColumnIterator: Iterator<Item = &'static str>;

    fn get_table_name(&self) -> &str;
    fn get_columns(&self) -> Self::ColumnIterator;
    fn get_max_rows(&self) -> i64;
    fn process_row(&self, row: &Row) -> Result<(), Box<dyn std::error::Error>>;
}

pub async fn stream_rows(
    connection_info: &ConnectionInfo,
    query: &str)
    -> Result<(), Box<dyn std::error::Error>> {

    let connection_string =
        format!("host={} user={} password={} dbname={}",
                &connection_info.hostname, &connection_info.username, &connection_info.password, &connection_info.database);

    let (client, connection) = tokio_postgres::connect(
        connection_string.as_str(),
        NoTls,
    ).await?;

    // Spawn a background task to handle the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let params:Vec<String> = vec![];

    // Execute a query and stream the results
    let row_stream = client
        .query_raw(&query.to_string(), &params)
        .await?;


    pin_mut!(row_stream);
    while let Some(row) = row_stream.try_next().await? {
        println!("{:?}", row);
    }

    Ok(())
}
