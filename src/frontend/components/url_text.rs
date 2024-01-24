use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    pub url: String,
}

#[function_component(UrlText)]
pub fn url_text(props: &Props) -> Html {
    html! {
        <div class="url-text">
            <textarea id="url_text" rows="3" max-rows="6" cols="100" readonly=true value={props.url.clone()} />
        </div>
    }
}
