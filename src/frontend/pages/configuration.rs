use yew::prelude::*;

use crate::frontend::components::copy_url_clipboard::CopyUrlClipboard;
use crate::frontend::components::navigation::Navigation;
use crate::frontend::params::Params;
use crate::frontend::components::url_text::UrlText;

#[function_component(Configuration)]
pub fn configuration() -> Html {
    html! {
        <>
        <Navigation />
        <h1>{ "Config for your Webpage" }</h1>
        <p>{ "You need an NFT? Get one from " }<a href="https://azero.id/">{"AZERO.ID"}</a> {"."}</p>
        <p>{ "This is the configuration page" }</p>

        <UrlText url={"https://azero.id/"} />
        <br/>
        <CopyUrlClipboard url={"https://azero.id/"} />
        </>
    }
}
