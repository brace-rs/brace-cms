use std::io;

use brace_cms::server;
use brace_config::Config;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let config = Config::load("config.toml").unwrap_or_else(|_| Config::new());

    server(config).await
}
