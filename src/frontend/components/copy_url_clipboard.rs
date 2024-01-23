use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    pub url: String,
}

#[function_component(CopyUrlClipboard)]
pub fn copy_url_clipboard(props: &Props) -> Html {
    let clipboard = use_clipboard();

    let url = props.url.clone();

    let onclick = Callback::from(move |_| {
        clipboard.write_text(url.clone());
    });

    html! {
        <>
        <button id="copy-url-clipboard" {onclick} >{ "Copy URL to clipboard" }</button>
        </>
    }
}
