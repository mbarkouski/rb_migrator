use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rb_migrator", version = "1.0", author = "rlapenok@gmail.com")]
pub struct Cli {
    /// Arg for path to config with connection to ClickHouse
    #[arg(long, short = 'c', value_name = "PATH")]
    pub config_path: PathBuf,

    /// Arg for path to folder with migrations
    #[arg(long, short = 'm', value_name = "PATH")]
    pub migration_path: PathBuf,
}
