mod error;
pub use error::{AppError, AppResult};
pub mod logger;

use clap::{Parser, Subcommand};

pub fn get_cli() -> Args {
    Args::parse()
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Окружение
    #[command(subcommand)]
    env: Option<Environment>,
    /// Имя файла конфигурации
    #[arg(short, long, value_name = "FILE")]
    config: Option<std::path::PathBuf>,
    /// Уровень логгирования
    #[arg(short, long, value_name = "LOG LEVEL")]
    level: Option<tracing::Level>,
}
impl Args {
    pub fn env(&self) -> Environment {
        self.env.clone().unwrap_or_default()
    }
    pub fn level(&self) -> tracing::Level {
        self.level.unwrap_or(tracing::Level::INFO)
    }
    pub fn config(&self) -> std::path::PathBuf {
        self.config
            .clone()
            .unwrap_or(std::path::PathBuf::from("settings.toml"))
    }
}

#[derive(Subcommand, PartialEq, Default, Clone)]
pub enum Environment {
    /// Окружение разработки
    Dev,
    /// Окружение релиза
    #[default]
    Prod,
}
