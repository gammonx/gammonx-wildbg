use clap::Parser;
use std::sync::Arc;
use tokio::net::TcpListener;
use web::startup::{self, Args};
use web::web_api::WebApi;
use std::env;
use axum::Router; 
use web::axum::router;

#[tokio::main]
async fn main() {
    // Read base path (e.g. "/bot") from environment
    let base_path = env::var("SERVICE_BASEPATH").unwrap_or_else(|_| "/bot/wildbg".to_string());

    // Read address & log links
    let web_address = startup::get_web_address(&Args::parse());
    log_server_links(&web_address);

    // Start TCP listener
    let listener = TcpListener::bind(&web_address)
        .await
        .unwrap_or_else(|_| panic!("Could not bind to the web address: '{web_address}'"));

    // Create app router
    let web_api = Arc::new(WebApi::try_default());
    let api_router = router(web_api);

    // Mount under base path if specified
    let app = if base_path.is_empty() || base_path == "/" {
        api_router
    } else {
        Router::new().nest(&base_path, api_router)
    };
    
    axum::serve(listener, app).await.unwrap();
}

/// Log the web address and helpful links to the command line.
///
/// # Arguments
///
/// * `web_address` - The address and port number to use for generating links and logging.
fn log_server_links(web_address: &str, base_path: &str) {
    println!("The server is running at 'http://{web_address}{base_path}'.\n");

    println!("You can access the server for example via");
    println!(
        "http://{web_address}{base_path}/move?die1=3&die2=1&p24=2&p19=-5&p17=-3&p13=5&p12=-5&p8=3&p6=5&p1=-2"
    );
    println!("http://{web_address}/swagger-ui");
}
