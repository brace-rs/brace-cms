use futures::future::TryFutureExt;

use brace_data_store_postgres::{Config as PostgresConfig, Error as PostgresError, Postgres};
use brace_web_core::web::{self, Data, ServiceConfig};
use brace_web_core::{Error, HttpResponse};

use brace_config::Config;

pub async fn configure(config: &Config) -> impl Fn(&mut ServiceConfig) + Clone {
    let mut postgres_config = PostgresConfig::new();

    postgres_config.host(
        &config
            .get::<_, String>("postgres.host")
            .unwrap_or_else(|_| String::from("localhost")),
    );

    postgres_config.port(config.get::<_, u16>("postgres.port").unwrap_or(5432));

    postgres_config.user(
        &config
            .get::<_, String>("postgres.user")
            .unwrap_or_else(|_| String::from("postgres")),
    );

    postgres_config.password(
        &config
            .get::<_, String>("postgres.pass")
            .unwrap_or_else(|_| String::from("postgres")),
    );

    let postgres = Postgres::new(postgres_config).await.unwrap();

    move |cfg: &mut ServiceConfig| {
        cfg.data(postgres.clone());
        cfg.route("/postgres", web::get().to(postgres_info));
    }
}

async fn postgres_info(postgres: Data<Postgres>) -> Result<HttpResponse, Error> {
    let version = postgres_version(&postgres).map_err(|_| ()).await?;

    Ok(HttpResponse::Ok().body(format!("postgres version: {}", version)))
}

async fn postgres_version(postgres: &Postgres) -> Result<String, PostgresError> {
    let conn = postgres.connect().await?;
    let row = conn.query_one("SELECT version()", &[]).await?;
    let res = row.get::<_, String>(0);

    Ok(res)
}
