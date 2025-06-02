use axum::{
    routing::{post},
    Router, Json, http::StatusCode,
    extract::State,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use std::fs::File;
use std::io::{BufReader, Write};
use rustls_pemfile::{certs, pkcs8_private_keys};
use tokio_rustls::TlsAcceptor;
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

mod tui;
use tui::{run_tui, AccelReading};

#[tokio::main]
async fn main() {
    // Create a channel for sending accelerometer data to the TUI
    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(tokio::sync::Mutex::new(tx));

    // Spawn TUI in a separate thread
    thread::spawn(move || {
        if let Err(e) = run_tui(rx) {
            eprintln!("Error running TUI: {}", e);
        }
    });

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/motion", post(handle_motion))
        .fallback_service(ServeDir::new("static"));

    // Share tx with the handler
    let app = app.with_state(Arc::clone(&tx));

    // Load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("certs/cert.pem").expect("failed to open cert file"));
    let key_file = &mut BufReader::new(File::open("certs/key.pem").expect("failed to open key file"));

    // Parse TLS key/cert
    let cert_chain = certs(cert_file)
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse cert");
    let mut keys = pkcs8_private_keys(key_file)
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse key");

    // Create TLS config
    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, rustls::pki_types::PrivateKeyDer::Pkcs8(keys.remove(0)))
        .expect("bad certificate/key");

    let tls_acceptor = TlsAcceptor::from(Arc::new(config));
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running at https://{}", addr);
    println!("IMPORTANT: Use https:// when accessing the server!");
    println!("Note: You may need to accept the self-signed certificate in your browser.");

    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind to address");
    loop {
        let (stream, addr) = listener.accept().await.expect("Failed to accept connection");
        let acceptor = tls_acceptor.clone();
        let app = app.clone();

        tokio::spawn(async move {
            match acceptor.accept(stream).await {
                Ok(tls_stream) => {
                    println!("New HTTPS connection from {}", addr);
                    let io = hyper_util::rt::TokioIo::new(tls_stream);
                    
                    let service = app.into_service();
                    let service = tower::ServiceBuilder::new()
                        .map_response(|r: axum::response::Response| r.map(axum::body::Body::new))
                        .service(service);

                    let hyper_service = hyper_util::service::TowerToHyperService::new(service);

                    if let Err(err) = hyper_util::server::conn::auto::Builder::new(hyper_util::rt::TokioExecutor::new())
                        .serve_connection(io, hyper_service)
                        .await
                    {
                        println!("Error serving connection: {}", err);
                    }
                }
                Err(e) => {
                    println!("Failed to establish HTTPS connection from {}: {}", addr, e);
                    println!("Make sure you're using https:// instead of http:// in your browser!");
                }
            }
        });
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct MotionData {
    x: f32,
    y: f32,
    z: f32,
}

// Updated handler with proper type annotations and channel sending
async fn handle_motion(
    State(tx): State<Arc<tokio::sync::Mutex<mpsc::Sender<AccelReading>>>>,
    Json(data): Json<MotionData>,
) -> Result<StatusCode, (StatusCode, String)> {
    let reading = AccelReading {
        x: data.x,
        y: data.y,
        z: data.z,
        timestamp: Instant::now(),
    };

    // Send to TUI
    let guard = tx.lock().await;
    if let Err(e) = guard.send(reading) {
        eprintln!("Failed to send reading to TUI: {}", e);
    }

    print!("\rAccelerometer: x={:6.2}, y={:6.2}, z={:6.2}", data.x, data.y, data.z);
    let _ = std::io::stdout().flush();
    Ok(StatusCode::OK)
}


