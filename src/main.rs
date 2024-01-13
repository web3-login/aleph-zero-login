use axum::Router;
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    serve(serve_dir("/", "static"), 8080).await;
}

fn serve_dir(path: &str, dir: &str) -> Router {
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

        let response = serve_dir("/", "static/").oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
