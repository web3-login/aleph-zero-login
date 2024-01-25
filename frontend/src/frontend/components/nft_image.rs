use crate::chain::Chain;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub chain: Chain,
    pub domain: String,
}

#[function_component(NftImage)]
pub fn nft_image(props: &Props) -> Html {
    let img_url = match props.chain {
        Chain::Azero => format!("https://azero.id/api/v1/image/{}.azero.png", props.domain),
        Chain::AzeroTest => format!("https://tzero.id/api/v1/image/{}.tzero.png", props.domain),
    };
    let url = match props.chain {
        Chain::Azero => format!("https://{}.azero.id", props.domain),
        Chain::AzeroTest => format!("https://{}.tzero.id", props.domain),
    };

    html! {
        <div class="nft-image">
        <div class="card">
            <div class="card-body">
                <a href={url} target="_blank" rel="noopener noreferrer">
                <img src={img_url} class="card-img-top" alt={format!("NFT image for {}", props.domain)} />
                </a>
            </div>
        </div>
        </div>
    }
}
