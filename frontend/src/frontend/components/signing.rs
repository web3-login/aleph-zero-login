use crate::chain::Chain;
use crate::frontend::services::{get_accounts, sign_nonce, to_hex, Account};
use crate::frontend::signature::Signature;
use anyhow::anyhow;
use futures::FutureExt;
use web_sys::console;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub chain: Chain,
    pub nft_id: String,
    pub nonce: String,
    pub on_signed: Callback<Signature>,
}

pub struct Signing {
    stage: SigningStage,
}

pub enum SigningStage {
    Error(String),
    CreatingOnlineClient,
    EnterMessage,
    RequestingAccounts,
    SelectAccount(Vec<Account>),
    Signing(Account),
    SigningSuccess {
        signer_account: Account,
        signature: String,
    },
}

pub enum Message {
    Error(anyhow::Error),
    RequestAccounts,
    ReceivedAccounts(Vec<Account>),
    /// usize represents account index in Vec<Account>
    SignWithAccount(usize),
    ReceivedSignature(Vec<u8>),
}

impl Component for Signing {
    type Message = Message;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Signing {
            stage: SigningStage::EnterMessage,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::RequestAccounts => {
                self.stage = SigningStage::RequestingAccounts;
                ctx.link().send_future(get_accounts().map(
                    |accounts_or_err| match accounts_or_err {
                        Ok(accounts) => Message::ReceivedAccounts(accounts),
                        Err(err) => Message::Error(err),
                    },
                ));
            }
            Message::ReceivedAccounts(accounts) => {
                self.stage = SigningStage::SelectAccount(accounts);
            }
            Message::Error(err) => self.stage = SigningStage::Error(err.to_string()),
            Message::SignWithAccount(i) => {
                if let SigningStage::SelectAccount(accounts) = &self.stage {
                    let account = accounts.get(i).unwrap();
                    let account_address = account.address.clone();
                    let account_source = account.source.clone();

                    self.stage = SigningStage::Signing(account.clone());
                    let nonce = ctx.props().nonce.to_string();

                    ctx.link().send_future(async move {
                        let Ok(signature) =
                            sign_nonce(account_source, account_address, nonce.clone()).await
                        else {
                            return Message::Error(anyhow!("Signing via extension failed"));
                        };
                        Message::ReceivedSignature(signature)
                    });
                }
            }
            Message::ReceivedSignature(signature) => {
                if let SigningStage::Signing(account) = &self.stage {
                    let signature = to_hex(signature);
                    console::log_1(&format!("signature: {:?}", signature).into());
                    ctx.props().on_signed.emit(Signature {
                        account: account.address.clone(),
                        domain: format!("{}.{}", ctx.props().nft_id, ctx.props().chain.get_tld()),
                        signature,
                    });
                }
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let message_html: Html = match &self.stage {
            SigningStage::Error(_)
            | SigningStage::EnterMessage
            | SigningStage::CreatingOnlineClient => html!(<></>),
            _ => {
                html!(
                    <div>
                        <div class="mb">
                            <b>{"Domain: "}</b> <br/>
                            {&ctx.props().nft_id} {format!(".{}", ctx.props().chain.get_tld())}
                        </div>
                    </div>
                )
            }
        };

        let signer_account_html: Html = match &self.stage {
            SigningStage::Signing(signer_account)
            | SigningStage::SigningSuccess { signer_account, .. } => {
                html!(
                    <div class="mb">
                            <b>{"Account used for signing: "}</b> <br/>
                            {"Name: "}{&signer_account.name} <br/>
                            {"Address: "}{&signer_account.address} <br/>
                    </div>
                )
            }
            _ => html!(<></>),
        };

        let stage_html: Html = match &self.stage {
            SigningStage::Error(error_message) => {
                html!(<div class="error"> {"Error: "} {error_message} </div>)
            }
            SigningStage::CreatingOnlineClient => {
                html!(
                    <div>
                        <b>{"Creating Online Client..."}</b>
                    </div>
                )
            }
            SigningStage::EnterMessage => {
                let get_accounts_click = ctx.link().callback(|_| Message::RequestAccounts);

                html!(
                    <>
                        <div class="mb"><br/></div>
                        <button onclick={get_accounts_click}> {"=> Select an Account for Signing"} </button>
                    </>
                )
            }
            SigningStage::RequestingAccounts => {
                html!(<div>{"Querying extensions for accounts..."}</div>)
            }
            SigningStage::SelectAccount(accounts) => {
                if accounts.is_empty() {
                    html!(<div>{"No Web3 extension accounts found. Install Talisman or the Polkadot.js extension and add an account."}</div>)
                } else {
                    html!(
                        <>
                            <div class="mb"><b>{"Select an account you want to use for signing:"}</b></div>
                            { for accounts.iter().enumerate().map(|(i, account)| {
                                let sign_with_account = ctx.link().callback(move |_| Message::SignWithAccount(i));
                                html! {
                                    <button onclick={sign_with_account}>
                                        {&account.source} {" | "} {&account.name}<br/>
                                        <small>{&account.address}</small>
                                    </button>
                                }
                            }) }
                        </>
                    )
                }
            }
            SigningStage::Signing(_) => {
                html!(<div>{"Singing message with browser extension..."}</div>)
            }
            SigningStage::SigningSuccess { .. } => {
                html!(
                    <>
                    </>
                )
            }
        };

        html! {
            <div class="signing">
                {message_html}
                {signer_account_html}
                {stage_html}
            </div>
        }
    }
}
