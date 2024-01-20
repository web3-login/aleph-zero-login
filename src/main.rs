use aleph_zero_login::backend::config::load_config;
use aleph_zero_login::backend::server::create_server;
use aleph_zero_login::backend::server::router;
use axum::Router;
use clap::Parser;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use web3_login::config::load_yml_config;

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

    let web3_login_config = load_yml_config(opts.config.into());

    let server = create_server(web3_login_config);
    let app = router(server).unwrap();

    let frontend = serve_dir("/", config.frontend_dir);

    let app = app
        .nest("/dist", frontend.clone())
        .fallback_service(frontend.clone());
    serve(app, config.port).await;
}

fn serve_dir(path: &str, dir: PathBuf) -> Router {
    Router::new().nest_service(path, ServeDir::new(dir))
}

async fn serve(app: Router, port: u16) {
    let addr_v4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    log::info!("Listening on {}", addr_v4);
    let listener = TcpListener::bind(addr_v4).await.unwrap();
    axum::serve(listener, app.into_make_service())
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
