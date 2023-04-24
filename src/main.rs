use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    http::Response,
    response::{Html, IntoResponse},
    routing::get,
    Router, Server,
};
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::broadcast;
use tokio::fs::read_to_string;

type Snapshot = Vec<f32>;

#[tokio::main]
async fn main() {
    let (tx,_) = broadcast::channel::<Snapshot>(1);
    
    tracing_subscriber::fmt::init();

    let app_sate = AppState{ tx: tx.clone()};
    
    let router = Router::new()
        .route("/", get(root))
        .route("/style.css", get(get_css))
        .route("/index.mjs", get(get_mjs))
        .route("/realtime/cpu", get(realtime_cpu_get))
        .with_state(app_sate.clone());

    // update CPU at the background
    tokio::task::spawn_blocking(move ||{
        let mut sys = System::new();
        loop {
            sys.refresh_cpu();
            let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
            let _ = tx.send(v);
            std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL)
        }
    });

    let server = Server::bind(&"0.0.0.0:3000".parse().unwrap())
                        .serve(router.into_make_service());

    let addr = server.local_addr();
    println!("Listening on {}", addr);

    server.await.unwrap();
}

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<Snapshot>,
}

#[axum::debug_handler]
async fn root() -> impl IntoResponse{
    let markup = read_to_string("src/index.html").await.unwrap();
    Html(markup)
}

#[axum::debug_handler]
async fn get_css() -> impl IntoResponse{
    let markup = read_to_string("src/style.css").await.unwrap();

    Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(markup)
        .unwrap()
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
async fn realtime_cpu_get(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|ws: WebSocket| async {
        realtime_cpu_stream(state, ws).await})
}

async fn realtime_cpu_stream(app_sate: AppState ,mut ws: WebSocket) {
    let mut rx = app_sate.tx.subscribe();
    while let Ok(msg) = rx.recv().await{
        ws.send(Message::Text(serde_json::to_string(&msg).unwrap()))
              .await
              .unwrap()
    }
}
