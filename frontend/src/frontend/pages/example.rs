use crate::frontend::components::{footer::Footer, navigation::Navigation};
use rust_2048::Model;
use yew::prelude::*;
use yew_oauth2::config::openid::Config;
use yew_oauth2::openid::{use_auth_agent, OAuth2};
use yew_oauth2::prelude::*;

#[function_component(Example)]
pub fn example() -> Html {
    let config = Config::new("example-client", "https://azero.web3-login.net");

    html! {
        <div id="example">
        <Navigation />
        <div class="jumbotron mt-4 p-3 mb-5 bg-light rounded shadow">
            <h1>{ "Play 2048" }</h1>
            <OAuth2 {config}>
            <MyApplicationMain/>
          </OAuth2>
        </div>
        <Footer />
        </div>
    }
}

#[function_component(MyApplicationMain)]
fn my_app_main() -> Html {
    let agent = use_auth_agent().expect("Must be nested inside an OAuth2 component");

    let login = use_callback(agent.clone(), |_, agent| {
        let _ = agent.start_login();
    });
    let logout = use_callback(agent, |_, agent| {
        let _ = agent.logout();
    });

    html!(
      <>
        <Failure><FailureMessage/></Failure>
        <Authenticated>
          <button onclick={logout}>{ "Logout" }</button>
            <Model/>
        </Authenticated>
        <NotAuthenticated>
          <button onclick={login}>{ "Login" }</button>
          <p>{ "You need an NFT? Get one from " }<a href="https://azero.id/">{"AZERO.ID"}</a> {"."}</p>
        </NotAuthenticated>
      </>
    )
}
