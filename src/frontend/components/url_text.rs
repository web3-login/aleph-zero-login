use yew::prelude::*;
use yew_router::prelude::*;


#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    pub url: String,
}


#[function_component(UrlText)]
pub fn url_text(props: &Props) -> Html {
    html! {
        <>
        <textarea id="url" rows=1 readonly=true value={props.url.clone()} />
        </>
    }
}