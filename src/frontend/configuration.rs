use crate::frontend::params::Params;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub params: Params,
}

#[function_component(Configuration)]
pub fn configuration(props: &Props) -> Html {
    html! {
        <>
            <p> { format!("Login to: {}", props.params.redirect_uri.as_ref().unwrap()) }</p>
        </>
    }
}
