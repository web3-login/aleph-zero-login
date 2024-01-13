use yew::prelude::*;

use super::chain_select::ChainSelect;
use super::footer::Footer;
use super::signing::SigningExamplesComponent;
use crate::chain::Chain;

#[function_component(App)]
pub fn app() -> Html {
    let selected_chain = use_state(|| Chain::Azero);

    let on_chain_select = {
        let selected_chain = selected_chain.clone();
        Callback::from(move |chain: Chain| selected_chain.set(chain))
    };

    html! {
        <>
        <h1>{ "Log in with your Azero.ID" }</h1>
        <p>{ "This is a demo of the Azero.ID login system." }</p>
        <p>{ "To log in, you need to have an Azero.ID token." }</p>
        <ChainSelect on_select={on_chain_select} />
        <SigningExamplesComponent chain={(*selected_chain).clone()} />
        <Footer />
        </>
    }
}
