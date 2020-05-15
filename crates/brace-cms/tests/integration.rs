use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use assert_cmd::prelude::*;
use awc::Client;

#[actix_rt::test]
async fn test_server() {
    let mut process = Command::cargo_bin("brace-cms").unwrap().spawn().unwrap();

    sleep(Duration::from_millis(1000));

    let client = Client::default();
    let res = client.get("http://127.0.0.1:8080").send().await.unwrap();

    assert_eq!(res.status(), 200);

    process.kill().unwrap();
}
