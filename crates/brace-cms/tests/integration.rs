use std::env::current_dir;
use std::process::Command;
use std::str::from_utf8;
use std::thread::sleep;
use std::time::Duration;

use assert_cmd::prelude::*;
use awc::Client;

#[actix_rt::test]
async fn test_server() {
    let dir = current_dir().unwrap().join("tests/fixtures");
    let mut process = Command::cargo_bin("brace-cms")
        .unwrap()
        .current_dir(dir)
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(1000));

    let client = Client::default();
    let res = client.get("http://127.0.0.1:65080").send().await.unwrap();

    assert_eq!(res.status(), 200);

    let mut res = client
        .get("http://127.0.0.1:65080/postgres")
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);

    let body = res.body().await.unwrap();
    let text = from_utf8(&body).unwrap();

    assert!(text.contains("PostgreSQL"));

    process.kill().unwrap();
}
