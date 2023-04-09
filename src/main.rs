mod args;
mod postquet_engine;

use args::parse_cli_args;
use env_logger::{Env, fmt::{Color, Formatter}};
use std::io::Write;
use log::{debug, error, log_enabled, info, Level};
use args::CliArgs;


fn cli_args_to_connection_info(args: &CliArgs) -> postquet_engine::ConnectionInfo {
    postquet_engine::ConnectionInfo {
        hostname: args.hostname.clone(),
        username: args.username.clone(),
        password: args.password.clone(),
        port: args.port.clone(),
        database: args.database.clone(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let env = Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S.%3fZ"),
                record.level(),
                record.module_path().unwrap_or_default(),
                record.args()
            )
        })
        .init();

    let cli_args = parse_cli_args();

    let connection_info = cli_args_to_connection_info(&cli_args);

    info!("hostname: {}", &connection_info.hostname);
    info!("username: {}", &connection_info.username);
    info!("password: {}", "********");
    info!("port: {}", &connection_info.port);
    info!("database: {}", &connection_info.database);
    info!("table: {}", &cli_args.table);


    let query = format!("SELECT * FROM {} ORDER BY id DESC LIMIT 10", &cli_args.table);

    postquet_engine::stream_rows(&connection_info, &query).await?;

    Ok(())
}
