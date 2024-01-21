use yew::prelude::*;
use yew_router::prelude::use_location;

use crate::chain::Chain;
use crate::frontend::chain_select::ChainSelect;
use crate::frontend::configuration::Configuration;
use crate::frontend::footer::Footer;
use crate::frontend::params::Params;
use crate::frontend::signing::SigningExamplesComponent;

#[function_component(Home)]
pub fn home() -> Html {
    let selected_chain = use_state(|| Chain::Azero);

    let location = use_location().unwrap();

    let mut params = location.query::<Params>().unwrap();

    if params.redirect_uri.is_none() {
        params.redirect_uri = Some("https%3A%2F%2Foidcdebugger.com%2Fdebug".to_string());
    }

    let on_chain_select = {
        let selected_chain = selected_chain.clone();
        Callback::from(move |chain: Chain| selected_chain.set(chain))
    };

    html! {
        <>
        <div class="jumbotron mt-4 p-3 mb-5 bg-light rounded shadow">
            <h1>{ "Log in with your Azero.ID" }</h1>
        </div>
        <div class="row card justify-content-center d-grid gap-3">
            <p>{ "This is a demo of the Azero.ID login system." }</p>
            <Configuration  {params} />
            <p>
                { "To log in, you need to have an " }
                <ChainSelect on_select={on_chain_select} />
                { " token." }
            </p>
            <SigningExamplesComponent chain={(*selected_chain).clone()} />
        </div>
        <Footer />
        </>
    }
}
