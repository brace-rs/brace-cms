use std::io;

use brace_web::core::{web, App, HttpResponse, HttpServer};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world")
}

pub async fn server() -> io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
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
