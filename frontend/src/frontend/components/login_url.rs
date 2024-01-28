use crate::frontend::params::Params;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    pub params: Params,
}

#[function_component(LoginUrl)]
pub fn login_url(props: &Props) -> Html {
    let url: String = match &props.params.redirect_uri {
        Some(uri) => format!("{}", urlencoding::decode(uri).expect("UTF-8")),
        None => String::from(""),
    };
    html! {
        <div class="login-url">
        {"Login to: "}<input type="text" readonly={true} value={ format!("{}", url) } />
        </div>
    }
}
