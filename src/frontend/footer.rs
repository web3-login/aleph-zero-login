use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="footer">
            <p id="copyright">
                <a href="https://github.com/web3-login/aleph-zero-login">{ "web3-login/aleph-zero-login" }</a>
            </p>
        </div>
    }
}
