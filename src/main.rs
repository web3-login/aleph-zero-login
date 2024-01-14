use aleph_zero_login::config::load_config;

use axum::Router;
use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[derive(Parser, Debug)]
#[command(name = "aleph-zero-login")]
struct Opt {
    /// Sets a custom config file. If not specified, 'config.yml' is used as the default.
    #[structopt(
        short,
        long,
        default_value = "config.yml",
        help = "Specify the path to the configuration file"
    )]
    config: String,

    #[structopt(short, long, help = "Specify the path to the frontend directory")]
    dir: Option<PathBuf>,

    #[structopt(short, long, help = "Specify the port to listen on")]
    port: Option<u16>,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let opts = Opt::parse();

    let mut config = load_config(&opts.config).unwrap();

    if let Some(port) = opts.port {
        config.port = port;
    }

    if let Some(dir) = opts.dir {
        config.frontend_dir = dir;
    }

    log::info!("config: {:?}", config);

    serve(serve_dir("/", config.frontend_dir), config.port).await;
}

fn serve_dir(path: &str, dir: PathBuf) -> Router {
    Router::new().nest_service(path, ServeDir::new(dir))
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    log::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot` function

    #[tokio::test]
    async fn test_using_serve_dir() {
        let req = Request::builder()
            .method("GET")
            .uri("/index.html")
            .body(Body::empty())
            .unwrap();

        let response = serve_dir("/", "static/".into()).oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
