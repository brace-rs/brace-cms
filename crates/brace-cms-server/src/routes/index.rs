use brace_web::core::HttpResponse;

pub async fn get() -> HttpResponse {
    HttpResponse::Ok().body("Hello world")
}

#[cfg(test)]
mod tests {
    use brace_web::core::test::{call_service, init_service, TestRequest};
    use brace_web::core::{web, App};

    use super::get;

    #[actix_rt::test]
    async fn test_server_index_get() {
        let mut app = init_service(App::new().route("/", web::get().to(get))).await;
        let req = TestRequest::with_header("content-type", "text/plain").to_request();
        let res = call_service(&mut app, req).await;

        assert!(res.status().is_success());
    }
}
