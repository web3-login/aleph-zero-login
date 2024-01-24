use yew::prelude::*;

use crate::frontend::components::copy_url_clipboard::CopyUrlClipboard;
use crate::frontend::components::navigation::Navigation;
use crate::frontend::components::url_text::UrlText;
use crate::frontend::components::config_form::ConfigForm;
use crate::frontend::params::Params;

#[function_component(Configuration)]
pub fn configuration() -> Html {
    let params = use_state(|| Params::default());

    let update_params = {
        let params = params.clone();
        Callback::from(move |new_params: Params| params.set(new_params))
    };

    let url = format!("https://azero.id/?client_id={}&redirect_uri={}&response_type={}&response_mode={}&state={}&nonce={}&realm={}&signature={}&account={}&contract={}",
        params.client_id.clone().unwrap_or_default(),
        params.redirect_uri.clone().unwrap_or_default(),
        params.response_type.clone().unwrap_or_default(),
        params.response_mode.clone().unwrap_or_default(),
        params.state.clone().unwrap_or_default(),
        params.nonce.clone().unwrap_or_default(),
        params.realm.clone().unwrap_or_default(),
        params.signature.clone().unwrap_or_default(),
        params.account.clone().unwrap_or_default(),
        params.contract.clone().unwrap_or_default(),
    );

    html! {
        <>
        <Navigation />
        <h1>{ "Config for your Webpage" }</h1>
        <p>{ "You need an NFT? Get one from " }<a href="https://azero.id/">{"AZERO.ID"}</a> {"."}</p>
        <p>{ "This is the configuration page" }</p>
        <ConfigForm params = {(*params).clone()} on_change = {update_params} />
        <UrlText url={url.clone()} />
        <br/>
        <CopyUrlClipboard url={url} />
        </>
    }
}