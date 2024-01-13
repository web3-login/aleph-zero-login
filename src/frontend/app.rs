use yew::prelude::*;

use super::footer::Footer;
use super::signing::SigningExamplesComponent;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
        <h1>{ "Log in with your Azero.ID" }</h1>
        <p>{ "This is a demo of the Azero.ID login system." }</p>
        <p>{ "To log in, you need to have an Azero.ID token." }</p>
        <SigningExamplesComponent />
        <Footer />
        </>
    }
}
