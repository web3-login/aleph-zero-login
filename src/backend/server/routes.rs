use crate::frontend::app::App;
use axum::{response::Html, routing::get, Router};
use web3_login::prelude::Server;
use yew::ServerRenderer;

pub async fn get_index() -> Html<String> {
    let renderer = ServerRenderer::<App>::new();

    let rendered = renderer.render().await;

    Html(rendered)
}
