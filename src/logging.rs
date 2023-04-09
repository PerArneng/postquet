use env_logger::{Builder, Env, fmt::{Color, Formatter}};
use std::io::Write;
use log::Level;
use colored::*;

pub fn init() {
    let env = Env::default().default_filter_or("info");
    Builder::from_env(env)
        .format(|buf, record| {
            let level_color = match record.level() {
                Level::Error => "red",
                Level::Warn => "yellow",
                Level::Info => "cyan",
                Level::Debug => "purple",
                Level::Trace => "white",
            };

            writeln!(
                buf,
                "{} {} [{}] {}",
                chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S.%3fZ"),
                record
                    .level()
                    .to_string()
                    .color(level_color)
                    .bold(),
                record
                    .module_path()
                    .unwrap_or_default()
                    .color(level_color),
                record.args()
            )
        })
        .init();
}