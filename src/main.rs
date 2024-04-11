use std::net::SocketAddr;

use axum::Router;
use clap::Parser;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// directory to serve
    #[arg(short, long, value_name = "PATH_TO_SERVE", default_value = ".")]
    path: std::path::PathBuf,

    /// where to listen
    #[arg(short, long, value_name = "SOCKET_ADDR", default_value = "[::]:8080")]
    listens: SocketAddr,
}

#[tokio::main]
async fn main() {
    let cli_args = CliArgs::parse();
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_private_network(true);
    let serve_dir = ServeDir::new(cli_args.path);
    let service = ServiceBuilder::new().layer(cors).service(serve_dir);
    let app = Router::new().fallback_service(service);

    let listener = tokio::net::TcpListener::bind(cli_args.listens)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
