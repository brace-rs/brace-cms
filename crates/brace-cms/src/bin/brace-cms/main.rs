use std::io;

use brace_cms::server::server;
use brace_config::Config;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let config = Config::load("config.toml").unwrap_or_else(|_| Config::new());

    brace_cms::log::init(&config).unwrap();

    server(config).await
}
