use crate::frontend::routes::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::{use_location, use_navigator};

#[function_component(Navigation)]
pub fn navigation() -> Html {
    let navigator = use_navigator().unwrap();

    let go_home_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Home));
        html! {
            <button {onclick}>{"Home"}</button>
        }
    };

    let go_config_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Configuration));
        html! {
            <button {onclick}>{"Configuration"}</button>
        }
    };

    html! {
        <nav class="navbar navbar-expand-lg navbar-light bg-light">
            <div class="container-fluid">
            {go_home_button}
            {go_config_button}
            </div>
        </nav>
    }
}
