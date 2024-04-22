use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))  // http://127.0.0.1:3000
        .route("/foo", get(get_foo).post(post_foo)) // http://127.0.0.1:3000/foo
        .route("/foo/bar", get(foo_bar)); // http://127.0.0.1:3000/foo/bar

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
// which calls one of these handlers
async fn root() -> String {
    String::from("hello axum")
}
async fn get_foo() -> String {
    String::from("get请求的foo")
}
async fn post_foo() -> String {
    String::from("post请求的foo")
}
async fn foo_bar() -> String {
    String::from("foo:bar")
}



