use yew::prelude::*;
use yew_router::prelude::use_location;

use crate::chain::Chain;
use crate::frontend::components::chain_select::ChainSelect;
use crate::frontend::components::footer::Footer;
use crate::frontend::components::login_url::LoginUrl;
use crate::frontend::components::navigation::Navigation;
use crate::frontend::components::nft_image::NftImage;
use crate::frontend::components::nft_input::NftInput;
use crate::frontend::components::signing::Signing;
use crate::frontend::params::Params;
use crate::frontend::signature::Signature;

#[function_component(Home)]
pub fn home() -> Html {
    let selected_chain = use_state(|| Chain::Azero);
    let nft_id = use_state(|| "azero".to_string());

    let location = use_location().unwrap();

    let mut params = location.query::<Params>().unwrap();

    if params.redirect_uri.is_none() {
        params.redirect_uri = Some("https%3A%2F%2Foidcdebugger.com%2Fdebug".to_string());
    }

    let on_chain_select = {
        let selected_chain = selected_chain.clone();
        Callback::from(move |chain: Chain| selected_chain.set(chain))
    };

    let on_nft_id_change = {
        let nft_id = nft_id.clone();
        Callback::from(move |nft: String| nft_id.set(nft))
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
        <div>
        <div class="jumbotron mt-4 p-3 mb-5 bg-light rounded shadow">
            <h1>{ "Log in with your Azero.ID" }</h1>
        </div>
        <div class="row card justify-content-center d-grid gap-3">
            <p>{ "This is a demo of a openidconnect login system based on Azero.ID." }</p>
            <LoginUrl  params={params.clone()} />
            <p>
                { "To log in, you need to have an " }
                <ChainSelect on_select={on_chain_select} />
                { " token." }
            </p>
        </div>
        <div class="card justify-content-center d-grid">
            <NftImage chain={(*selected_chain).clone()} domain={(*nft_id).clone()} />
            <NftInput chain={(*selected_chain).clone()} nft_id={(*nft_id).clone()} onchange={on_nft_id_change} />
            <Signing chain={(*selected_chain).clone()} nft_id={(*nft_id).clone()} {nonce} {on_signed} />
        </div>
        </div>
        <Footer />
        </div>
    }
}
