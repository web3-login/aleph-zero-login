use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="footer">
            <div class="container">
                <p id="copyright">
                    <a href="https://github.com/web3-login/aleph-zero-login">{ "web3-login/aleph-zero-login" }</a>
                </p>
            </div>
        </div>
    }
}
