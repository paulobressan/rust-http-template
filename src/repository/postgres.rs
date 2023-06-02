use std::env;
use std::error::Error;
use std::io::Read;

use deadpool_postgres::Pool;
use deadpool_postgres::{Manager, ManagerConfig, RecyclingMethod};
use rustls::{Certificate, RootCertStore};
use tokio_postgres_rustls::MakeRustlsConnect;

#[cfg(test)]
use crate::domain::error::DomainError;
#[cfg(test)]
use tokio_postgres::NoTls;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

struct PostgresConfig {
    host: String,
    user: String,
    password: String,
    name: String,
    pool_max: usize,
}
impl PostgresConfig {
    fn from_env() -> Self {
        Self {
            host: env::var("DATABASE_HOST").expect("DATABASE_HOST must be set"),
            user: env::var("DATABASE_USER").expect("DATABASE_USER must be set"),
            password: env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set"),
            name: env::var("DATABASE_NAME").expect("DATABASE_NAME must be set"),
            pool_max: env::var("DATABASE_POOL_MAX")
                .expect("DATABASE_POOL_MAX must be set")
                .parse::<usize>()
                .expect("DATABASE_POOL_MAX is usize"),
        }
    }
}
impl From<PostgresConfig> for tokio_postgres::Config {
    fn from(_: PostgresConfig) -> Self {
        let mut pg_config = tokio_postgres::Config::new();

        let postgres_config = PostgresConfig::from_env();
        pg_config.user(&postgres_config.user);
        pg_config.password(&postgres_config.password);
        pg_config.dbname(&postgres_config.name);
        pg_config.host(&postgres_config.host);

        pg_config
    }
}

pub fn init() -> Result<deadpool_postgres::Pool, Box<dyn Error>> {
    let postgres_config = PostgresConfig::from_env();
    let pool_max = postgres_config.pool_max;
    let pg_config: tokio_postgres::Config = postgres_config.into();

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };

    let mgr = Manager::from_config(pg_config, get_tls_connector()?, mgr_config);
    Ok(Pool::builder(mgr).max_size(pool_max).build()?)
}

pub async fn run_migrations() -> Result<(), Box<dyn Error>> {
    let pg_config: tokio_postgres::Config = PostgresConfig::from_env().into();

    let (mut client, connection) = pg_config.connect(get_tls_connector()?).await?;

    let handler = tokio::spawn(async move {
        connection.await.unwrap();
    });

    let migration_report = embedded::migrations::runner()
        .run_async(&mut client)
        .await?;

    for migration in migration_report.applied_migrations() {
        println!(
            "Migration Applied -  Name: {}, Version: {}",
            migration.name(),
            migration.version()
        );
    }

    handler.abort();
    Ok(())
}

pub fn get_tls_connector() -> Result<MakeRustlsConnect, Box<dyn Error>> {
    let mut cert_buffer = vec![];
    std::fs::File::open(env::current_dir()?.join("ca-certificates/us-east-1-bundle.pem"))?
        .read_to_end(&mut cert_buffer)?;
    let pems = pem::parse_many(cert_buffer)?;
    let mut root_cert_store = RootCertStore::empty();

    for pem in pems {
        let certificate = Certificate(pem.contents().to_vec());
        root_cert_store.add(&certificate)?;
    }

    let tls_config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    Ok(MakeRustlsConnect::new(tls_config))
}

#[cfg(test)]
pub async fn init_to_tests() -> Result<(), DomainError> {
    let postgres_config = PostgresConfig::from_env();

    let mut pg_config = tokio_postgres::Config::new();
    pg_config.user(&postgres_config.user);
    pg_config.password(&postgres_config.password);
    pg_config.host(&postgres_config.host);

    let (client, connection) = pg_config.connect(NoTls).await?;

    let handler = tokio::spawn(async move {
        connection.await.unwrap();
    });

    let stmt = client.prepare("select datname from pg_database;").await?;
    let result = client.query(&stmt, &[]).await?;
    let mut databases: Vec<String> = vec![];
    result
        .into_iter()
        .for_each(|row| databases.push(row.get("datname")));

    if databases.contains(&postgres_config.name) {
        let stmt = client
            .prepare(&format!("drop database {};", postgres_config.name))
            .await?;
        client.execute(&stmt, &[]).await?;
    }

    let stmt = client
        .prepare(&format!("create database {};", postgres_config.name))
        .await?;
    client.execute(&stmt, &[]).await?;

    handler.abort();

    run_migrations()
        .await
        .expect("Error to run migration to tests");

    Ok(())
}
