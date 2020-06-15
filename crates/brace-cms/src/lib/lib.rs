use std::io;
use std::net::Ipv4Addr;

use brace_config::Config;
use brace_web::core::middleware::Logger;
use brace_web::core::{web, App, HttpResponse, HttpServer};

pub mod store {
    pub mod postgres {
        pub use brace_cms_store_postgres::*;
    }
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world")
}

pub async fn server(config: Config) -> io::Result<()> {
    brace_cms_log::init(&config).unwrap();

    let host = config
        .get::<_, Ipv4Addr>("server.host")
        .unwrap_or_else(|_| Ipv4Addr::new(127, 0, 0, 1));
    let port = config.get::<_, u16>("server.port").unwrap_or(8080);
    let addr = format!("{}:{}", host, port);
    let format = r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#;

    let postgres = brace_cms_store_postgres::configure(&config).await;

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(format))
            .configure(postgres.clone())
            .route("/", web::get().to(index))
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
