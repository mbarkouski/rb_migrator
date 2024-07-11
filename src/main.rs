mod args;
mod config;
mod migrations;
mod storage;
use clap::Parser;
use env_logger::Builder;
use storage::ClickhouseClient;

use args::Cli;
use config::ClickHouseCfg;
use migrations::Migarion;

#[tokio::main]
async fn main() {
    let mut builder=Builder::new();
    builder.filter(Some("clickhouse_rs"), log::LevelFilter::Off);
    builder.filter(None, log::LevelFilter::Info);
    builder.format_timestamp(Some(env_logger::TimestampPrecision::Seconds));
    builder.init();

    let args = Cli::parse();
    //get args from cli
    let config_path = args.config_path;
    let migrations_path = args.migration_path;
    //read config connection to ClickHouse from file
    let cfg = ClickHouseCfg::new(config_path);
    //read migration from file
    let migration = Migarion::from(migrations_path);
    //create client for connection to CliclHouse
    let client = ClickhouseClient::connect(cfg).await;
    //apply migartion
    client.run_migrations(migration).await.map_or_else(|err|{
      log::error!("Error to run migration:{}",err);
    }, |_|{
      log::info!("Migration applied successfully");
    });
}
