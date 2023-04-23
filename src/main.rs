use axum::{routing::get, Router, Server, extract::State};
use sysinfo::{System, SystemExt, CpuExt};
use std::{fmt::Write, sync::{Arc, Mutex}};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root))
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


async fn root(State(state): State<AppState>) -> String{
    let mut s = String::new();   
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();

    for (i, cpu) in sys.cpus().iter().enumerate() {
        let i  = i + 1;
        
        let usage =  cpu.cpu_usage();
        writeln!(&mut s , "CPU {i} {usage}%").unwrap();
    }
    
    return s;
}
