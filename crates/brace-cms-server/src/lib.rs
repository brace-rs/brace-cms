use std::io;
use std::net::Ipv4Addr;

use brace_cms_core::config::Config;
use brace_web::core::middleware::Logger;
use brace_web::core::{web, App, HttpServer};

pub mod routes;

pub async fn server(config: Config) -> io::Result<()> {
    let host = config
        .get::<_, Ipv4Addr>("server.host")
        .unwrap_or_else(|_| Ipv4Addr::new(127, 0, 0, 1));
    let port = config.get::<_, u16>("server.port").unwrap_or(8080);
    let addr = format!("{}:{}", host, port);
    let format = r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#;

    let postgres = brace_cms_store::postgres::configure(&config).await;

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(format))
            .configure(postgres.clone())
            .route("/", web::get().to(crate::routes::index::get))
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
