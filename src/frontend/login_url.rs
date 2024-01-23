use crate::frontend::params::Params;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    pub params: Params,
}

#[function_component(LoginUrl)]
pub fn login_url(props: &Props) -> Html {
    html! {
        <>
            <p> { format!("Login to: {}", props.params.redirect_uri.as_ref().unwrap()) }</p>
        </>
    }
}
