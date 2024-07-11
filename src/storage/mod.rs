use crate::{config::ClickHouseCfg, migrations::Migarion};
use clickhouse_rs::errors::Result;

use clickhouse_rs::{types::certificate_wrapper::ClickHouseCertificateWrapper, Options, Pool};

use std::{process::exit, time::Duration};
use url::Url;
pub struct ClickhouseClient(Pool);

impl ClickhouseClient {
    pub async fn connect(cfg: ClickHouseCfg) -> Self {
        //create url
        let url = Url::parse(
            format!(
                "tcp://{}:{}",
                cfg.host, cfg.port
            )
            .as_str(),
        ).unwrap_or_else(|err|{
            log::error!("Create Options for connection to ClickHouse Error:{}",err);
            exit(1);
        });
    //create certificate
        let certificate = ClickHouseCertificateWrapper::new(cfg.cert_path)
            .unwrap_or_else(|err|{
                log::error!("Error to create certificate for connection to ClickHouse:{}",err);
                exit(1);
            })
            .get_cert();
        //create connection options
        let mut opt = Options::new(url)
        .username(&cfg.user_name)
        .password(&cfg.password)
            .database(&cfg.db)
            .connection_timeout(Duration::from_secs(15))
            .ping_before_query(true);
        //if tls enable
        if let Some(cert)=certificate{
                opt=opt.secure(true).certificate(Some(cert));
        }
        let pool = Pool::new(opt);
        pool.get_handle().await.unwrap_or_else(|err|{
            log::error!("Connectig to Clickhouse Error: {}",err);
            exit(1);
        }).ping().await.unwrap_or_else(|err|{
            log::error!("Connectig to Clickhouse Error: {}",err);
            exit(1);
        });
        log::info!("Successful connection to ClickHouse");
        Self(pool)
    }
    pub async fn run_migrations(&self, migrations: Migarion)->Result<()> {
        log::info!("Starting to apply migrations");
        for m in migrations.0{
           self.0.get_handle().await?.execute(m).await? 
        }
        Ok(())

    }
}
