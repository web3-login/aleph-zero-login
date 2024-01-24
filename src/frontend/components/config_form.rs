use yew::prelude::*;

use crate::frontend::params::Params;
use web_sys::HtmlInputElement;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub params: Params,
    pub on_change: Callback<Params>,
}

#[function_component(ConfigForm)]
pub fn config_form(props: &Props) -> Html {
    let oninput = {
        let params = props.params.clone();
        let update_params = props.on_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_params = params.clone();
            // Update the corresponding field in new_params based on input's id or name
            match input.id().as_str() {
                "client_id" => new_params.client_id = Some(input.value()),
                "redirect_uri" => new_params.redirect_uri = Some(input.value()),
                "response_type" => new_params.response_type = Some(input.value()),
                "response_mode" => new_params.response_mode = Some(input.value()),
                "state" => new_params.state = Some(input.value()),
                "nonce" => new_params.nonce = Some(input.value()),
                "realm" => new_params.realm = Some(input.value()),
                "signature" => new_params.signature = Some(input.value()),
                "account" => new_params.account = Some(input.value()),
                "contract" => new_params.contract = Some(input.value()),
                _ => {}
            }
            update_params.emit(new_params); // Notify the parent about the changes
        })
    };

    html! {
        <div class="config-form">
        <form>
        <div class="mb-3">
            <label for="client_id" class="form-label">{"Client ID"}</label>
            <input type="text" class="form-control" id="client_id" value={props.params.client_id.clone()} oninput={oninput.clone()} />
        </div>
        <div class="mb-3">
            <label for="redirect_uri" class="form-label">{"Redirect URI"}</label>
            <input type="text" class="form-control" id="redirect_uri" value={props.params.redirect_uri.clone()} oninput={oninput.clone()} />
        </div>
        <div class="mb-3">
            <label for="response_type" class="form-label">{"Response Type"}</label>
            <input type="text" class="form-control" id="response_type" value={props.params.response_type.clone()} oninput={oninput.clone()} />
        </div>
        <div class="mb-3">
            <label for="response_mode" class="form-label">{"Response Mode"}</label>
            <input type="text" class="form-control" id="response_mode" value={props.params.response_mode.clone()} oninput={oninput.clone()} />
        </div>
        <div class="mb-3">
            <label for="state" class="form-label">{"State"}</label>
            <input type="text" class="form-control" id="state" value={props.params.state.clone()} oninput={oninput.clone()} />
        </div>
        <div class="mb-3">
            <label for="nonce" class="form-label">{"Nonce"}</label>
            <input type="text" class="form-control" id="nonce" value={props.params.nonce.clone()} oninput={oninput.clone()} />
        </div>
        <div class="mb-3">
            <label for="realm" class="form-label">{"Realm"}</label>
            <input type="text" class="form-control" id="realm" value={props.params.realm.clone()} oninput={oninput.clone()} />
        </div>
        <div class="mb-3">
            <label for="signature" class="form-label">{"Signature"}</label>
            <input type="text" class="form-control" id="signature" value={props.params.signature.clone()} oninput={oninput.clone()} />
        </div>
        <div class="mb-3">
            <label for="account" class="form-label">{"Account"}</label>
            <input type="text" class="form-control" id="account" value={props.params.account.clone()} oninput={oninput.clone()} />
        </div>
        <div class="mb-3">
            <label for="contract" class="form-label">{"Contract"}</label>
            <input type="text" class="form-control" id="contract" value={props.params.contract.clone()} oninput={oninput.clone()} />
        </div>
        </form>
        </div>
    }
}
