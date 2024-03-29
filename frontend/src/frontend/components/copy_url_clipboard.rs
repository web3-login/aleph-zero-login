use yew::prelude::*;
use yew_hooks::prelude::*;

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
        <div class="copy-url-clipboard">
        <button {onclick} >{ "Copy URL to clipboard" }</button>
        </div>
    }
}
