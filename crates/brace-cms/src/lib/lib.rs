use std::io;
use std::net::Ipv4Addr;

use futures::future::TryFutureExt;

use brace_config::Config;
use brace_data_store_postgres::{Config as PostgresConfig, Error as PostgresError, Postgres};
use brace_web::core::middleware::Logger;
use brace_web::core::{web, App, Error, HttpResponse, HttpServer};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world")
}

async fn postgres_info(postgres: web::Data<Postgres>) -> Result<HttpResponse, Error> {
    let version = postgres_version(&postgres).map_err(|_| ()).await?;

    Ok(HttpResponse::Ok().body(format!("postgres version: {}", version)))
}

async fn postgres_version(postgres: &Postgres) -> Result<String, PostgresError> {
    let conn = postgres.connect().await?;
    let row = conn.query_one("SELECT version()", &[]).await?;
    let res = row.get::<_, String>(0);

    Ok(res)
}

pub async fn server(config: Config) -> io::Result<()> {
    brace_cms_log::init(&config).unwrap();

    let host = config
        .get::<_, Ipv4Addr>("server.host")
        .unwrap_or_else(|_| Ipv4Addr::new(127, 0, 0, 1));
    let port = config.get::<_, u16>("server.port").unwrap_or(8080);
    let addr = format!("{}:{}", host, port);
    let format = r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#;

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

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(format))
            .data(postgres.clone())
            .route("/", web::get().to(index))
            .route("/postgres", web::get().to(postgres_info))
    });

    #[cfg(feature = "dev")]
    {
        use listenfd::ListenFd;

        let mut listenfd = ListenFd::from_env();

        server = match listenfd.take_tcp_listener(0)? {
            Some(lst) => server.listen(lst)?,
            None => server.bind(addr)?,
        };
    }

    #[cfg(not(feature = "dev"))]
    {
        server = server.bind(addr)?;
    }

    server.run().await
}

#[cfg(test)]
mod tests {
    use brace_web::core::test::{call_service, init_service, TestRequest};
    use brace_web::core::{web, App};

    use super::index;

    #[actix_rt::test]
    async fn test_server_index_get() {
        let mut app = init_service(App::new().route("/", web::get().to(index))).await;
        let req = TestRequest::with_header("content-type", "text/plain").to_request();
        let res = call_service(&mut app, req).await;

        assert!(res.status().is_success());
    }
}
