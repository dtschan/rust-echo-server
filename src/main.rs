use axum::{
    routing::get,
    Router,
    response::{Html, Response},
    http::{HeaderMap, HeaderValue, StatusCode},
    extract::ConnectInfo,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{info, error};
use tracing_subscriber;
use dns_lookup::lookup_addr;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Define the route
    let app = Router::new()
        .route("/", get(echo_handler));

    // Bind to all interfaces
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    println!("Server running on http://0.0.0.0:3000");
    
    // Start the server
    axum::serve(
        listener, 
        app.into_make_service_with_connect_info::<SocketAddr>()
    ).await.unwrap();
}

async fn echo_handler(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> Response {
    // Attempt to resolve DNS name
    let dns_name = match lookup_addr(&addr.ip()) {
        Ok(name) => {
            info!("Reverse DNS lookup for {}: {}", addr.ip(), name);
            name
        },
        Err(e) => {
            error!("Reverse DNS lookup failed for {}: {}", addr.ip(), e);
            addr.ip().to_string()
        }
    };

    // Prepare headers output
    let headers_output = headers.iter()
        .map(|(name, value)| {
            format!("{}: {}", 
                name.as_str(), 
                value.to_str().unwrap_or("Invalid header value")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Construct response
    let response_body = format!(
        "Client IP: {}\n\
        DNS Name: {}\n\n\
        Request Headers:\n{}",
        addr.ip(),
        dns_name,
        headers_output
    );

    // Create a response with text/plain content type
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(response_body.into())
        .unwrap()
}
