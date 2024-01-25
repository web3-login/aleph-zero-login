use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::chain::Chain;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub nft_id: String,
    pub chain: Chain,
    pub onchange: Callback<String>,
}

#[function_component(NftInput)]
pub fn nft_input(props: &Props) -> Html {
    let on_input = {
        let onchange = props.onchange.clone();
        Callback::from(move |event: InputEvent| {
            let input_element = event.target_dyn_into::<HtmlInputElement>().unwrap();
            let value: String = input_element.value();
            onchange.emit(value);
        })
    };

    html! {
        <div class="nft-input">
        <div class="col-12">
            <div class="input-group mb-3">
                <input type="text" placeholder="azero" oninput={on_input} /> {format!(".{}", props.chain.get_tld())}
            </div>
        </div>
        </div>
    }
}
