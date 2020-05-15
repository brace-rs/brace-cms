use std::io;

use brace_cms::server;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    server().await
}
