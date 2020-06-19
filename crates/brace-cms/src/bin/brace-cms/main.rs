use std::io;

use brace_cms::core::config::Config;
use brace_cms::server::server;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let config = Config::load("config.toml").unwrap_or_else(|_| Config::new());

    brace_cms::log::init(&config).unwrap();

    server(config).await
}
