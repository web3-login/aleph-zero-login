use yew::prelude::*;

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    pub url: String,
}

#[function_component(UrlText)]
pub fn url_text(props: &Props) -> Html {
    html! {
        <div class="url-text">
            <textarea id="url_text" rows="5" max-rows="6" cols="80" readonly=true value={props.url.clone()} />
        </div>
    }
}
