use crate::chain::Chain;
use std::str::FromStr;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

pub enum Msg {
    ChangeChain(Chain),
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub on_select: Callback<Chain>,
}

#[function_component(ChainSelect)]
pub fn chain_select(props: &Props) -> Html {
    let on_change = {
        let on_select = props.on_select.clone();
        Callback::from(move |event: Event| {
            let target: HtmlSelectElement = event.target_unchecked_into();
            let value = target.value();
            match Chain::from_str(&value) {
                Ok(chain) => on_select.emit(chain),
                Err(err) => log::error!("{}", err),
            }
        })
    };

    html! {
        <div class="chain-select">
            <select class="form-select" aria-label="Select chain" onchange={on_change}>
                <option value="Azero" selected=true>{ "Azero" }</option>
                <option value="AzeroTest">{ "AzeroTest" }</option>
            </select>
        </div>
    }
}
