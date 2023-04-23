use axum::{routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(root));
    let server = Server::bind(&"0.0.0.0:3000".parse().unwrap())
                        .serve(router.into_make_service());

    let addr = server.local_addr();
    println!("Listening on {}", addr);
    server.await.unwrap();
}


async fn root() -> &'static str {
    "HELLO AXUM"
}
