use axum::{routing::get, Router, Server, extract::State, Json, response::{IntoResponse, Html}, http::Response};
use sysinfo::{System, SystemExt, CpuExt};
use std::sync::{Arc, Mutex};
use tokio::fs::read_to_string;


#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root))
        .route("/index.mjs", get(get_mjs))
        .route("/api/cpu", get(get_cpu))
        .with_state(AppState {
            sys: Arc::new(Mutex::new(System::new()))
        });

    let server = Server::bind(&"0.0.0.0:3000".parse().unwrap())
                        .serve(router.into_make_service());

    let addr = server.local_addr();
    println!("Listening on {}", addr);
    server.await.unwrap();
}

#[derive(Debug,Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

#[axum::debug_handler]
async fn root() -> impl IntoResponse{
    let markup = read_to_string("src/index.html").await.unwrap();
    Html(markup)
}

#[axum::debug_handler]
async fn get_mjs() -> impl IntoResponse{
    let markup = read_to_string("src/index.mjs").await.unwrap();

    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(markup)
        .unwrap()
}

#[axum::debug_handler]
async fn get_cpu(State(state): State<AppState>) -> impl IntoResponse{
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();
    
    let v:Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
    Json(v)
}
