use yew::prelude::*;
use yew_router::prelude::{use_location, use_navigator};

use crate::chain::Chain;
use crate::frontend::components::chain_select::ChainSelect;
use crate::frontend::components::footer::Footer;
use crate::frontend::components::login_url::LoginUrl;
use crate::frontend::components::navigation::Navigation;
use crate::frontend::params::Params;
use crate::frontend::routes::Route;
use crate::frontend::signature::Signature;
use crate::frontend::signing::SigningExamplesComponent;

#[function_component(Home)]
pub fn home() -> Html {
    let selected_chain = use_state(|| Chain::Azero);
    let signature = use_state(|| Signature::default());

    let location = use_location().unwrap();
    let navigator = use_navigator().unwrap();

    let mut params = location.query::<Params>().unwrap();

    if params.redirect_uri.is_none() {
        params.redirect_uri = Some("https%3A%2F%2Foidcdebugger.com%2Fdebug".to_string());
    }

    let on_chain_select = {
        let selected_chain = selected_chain.clone();
        Callback::from(move |chain: Chain| selected_chain.set(chain))
    };

    let on_signed = {
        let selected_chain = selected_chain.clone();
        let params = params.clone();

        Callback::from(move |signature: Signature| {
            let mut params = params.clone();
            params.merge_default();
            params.merge_signature(&signature);
            params.merge_realm(&selected_chain.to_string());
            let url = format!("{}?{}", "/authorize", serde_qs::to_string(&params).unwrap());

            // Use web_sys to navigate to the new URL
            let window = web_sys::window().unwrap();
            let location = window.location();
            location.set_href(&url).unwrap();
        })
    };

    let nonce = params.nonce.clone().unwrap_or("random".to_string().clone());

    html! {
        <div id="home">
        <Navigation />
        <div class="jumbotron mt-4 p-3 mb-5 bg-light rounded shadow">
            <h1>{ "Log in with your Azero.ID" }</h1>
        </div>
        <div class="row card justify-content-center d-grid gap-3">
            <p>{ "This is a demo of the Azero.ID login system." }</p>
            <LoginUrl  params={params.clone()} />
            <p>
                { "To log in, you need to have an " }
                <ChainSelect on_select={on_chain_select} />
                { " token." }
            </p>
            <SigningExamplesComponent chain={(*selected_chain).clone()} {nonce} {on_signed} />
        </div>
        <Footer />
        </div>
    }
}
