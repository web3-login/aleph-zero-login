use yew::prelude::*;
use yew_router::prelude::*;

use super::pages::configuration::Configuration;

mod home;
pub use home::Home;

use crate::frontend::params::Params;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/authorize")]
    Authorize,
    #[at("/config")]
    Configuration,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Authorize)]
pub fn authorize() -> Html {
    let location = use_location().unwrap();
    let navigator = use_navigator().unwrap();

    let params = location.query::<Params>().unwrap();
    html! {
        <>
            <p>{ "This is the authorize page" }</p>
            <p>{ format!("Params: {:?}", params) }</p>
            <button onclick={Callback::from(move |_| navigator.push(&Route::Home))}>{ "Go back" }</button>
        </>
    }
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Configuration => html! { <Configuration /> },
        Route::Authorize => html! { <Authorize /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
