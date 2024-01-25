use super::components::nft_image::NftImage;
use super::services::{
    extension_signature_for_partial_extrinsic, get_accounts, polkadot, sign_nonce, Account,
};
use super::signature::Signature;
use crate::chain::Chain;
use anyhow::anyhow;
use futures::FutureExt;
use subxt::ext::codec::Encode;
use subxt::tx::SubmittableExtrinsic;
use subxt::tx::TxPayload;
use subxt::utils::{AccountId32, MultiSignature};
use subxt::{OnlineClient, PolkadotConfig};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub chain: Chain,
    pub nft_id: String,
    pub nonce: String,
    pub on_signed: Callback<Signature>,
}

pub struct SigningExamplesComponent {
    remark_call_bytes: Vec<u8>,
    online_client: Option<OnlineClient<PolkadotConfig>>,
    stage: SigningStage,
}

impl SigningExamplesComponent {
    fn set_message(&mut self, message: String) {
        let remark_call = polkadot::tx().system().remark(message.as_bytes().to_vec());
        let online_client = self.online_client.as_ref().unwrap();
        let remark_call_bytes = remark_call
            .encode_call_data(&online_client.metadata())
            .unwrap();
        self.remark_call_bytes = remark_call_bytes;
    }
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
        signature: MultiSignature,
        signed_extrinsic_hex: String,
        submitting_stage: SubmittingStage,
    },
}

pub enum SubmittingStage {
    Initial {
        signed_extrinsic: SubmittableExtrinsic<PolkadotConfig, OnlineClient<PolkadotConfig>>,
    },
    Submitting,
    Success {
        remark_event: polkadot::system::events::ExtrinsicSuccess,
    },
    Error(anyhow::Error),
}

pub enum Message {
    Error(anyhow::Error),
    OnlineClientCreated(OnlineClient<PolkadotConfig>),
    ChangeMessage(String),
    RequestAccounts,
    ReceivedAccounts(Vec<Account>),
    /// usize represents account index in Vec<Account>
    SignWithAccount(usize),
    ReceivedSignature(Vec<u8>),
    SubmitSigned,
    ExtrinsicFinalized {
        remark_event: polkadot::system::events::ExtrinsicSuccess,
    },
    ExtrinsicFailed(anyhow::Error),
}

impl Component for SigningExamplesComponent {
    type Message = Message;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let url: String = ctx.props().chain.get_url().to_string();

        ctx.link().send_future(OnlineClient::<PolkadotConfig>::from_url(url.clone()).map(|res| {
            match res {
                Ok(online_client) => Message::OnlineClientCreated(online_client),
                Err(err) => Message::Error(anyhow!("Online Client could not be created. Make sure you have a local node running:\n{err}")),
            }
        }));

        SigningExamplesComponent {
            stage: SigningStage::CreatingOnlineClient,
            online_client: None,
            remark_call_bytes: vec![],
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        let url: String = ctx.props().chain.get_url().to_string();
        ctx.link().send_future(OnlineClient::<PolkadotConfig>::from_url(url.clone()).map(|res| {
            match res {
                Ok(online_client) => Message::OnlineClientCreated(online_client),
                Err(err) => Message::Error(anyhow!("Online Client could not be created. Make sure you have a local node running:\n{err}")),
            }
        }));
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::OnlineClientCreated(online_client) => {
                self.online_client = Some(online_client);
                self.stage = SigningStage::EnterMessage;
            }
            Message::ChangeMessage(message) => {
                self.set_message(message);
            }
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
                    let account_id: AccountId32 = account_address.parse().unwrap();

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
                    ctx.props().on_signed.emit(Signature {
                        account: account.address.clone(),
                        domain: format!("{}.{}", ctx.props().nft_id, ctx.props().chain.get_tld()),
                        signature: format!(
                            "0x{}",
                            hex::encode(signature.encode())[4..].to_string()
                        ),
                    });
                }
            }
            Message::SubmitSigned => {
                if let SigningStage::SigningSuccess {
                    submitting_stage: submitting_stage @ SubmittingStage::Initial { .. },
                    ..
                } = &mut self.stage
                {
                    let SubmittingStage::Initial { signed_extrinsic } =
                        std::mem::replace(submitting_stage, SubmittingStage::Submitting)
                    else {
                        panic!("unreachable")
                    };

                    ctx.link().send_future(async move {
                        match submit_wait_finalized_and_get_extrinsic_success_event(
                            signed_extrinsic,
                        )
                        .await
                        {
                            Ok(remark_event) => Message::ExtrinsicFinalized { remark_event },
                            Err(err) => Message::ExtrinsicFailed(err),
                        }
                    });
                }
            }
            Message::ExtrinsicFinalized { remark_event } => {
                if let SigningStage::SigningSuccess {
                    submitting_stage, ..
                } = &mut self.stage
                {
                    *submitting_stage = SubmittingStage::Success { remark_event }
                }
            }
            Message::ExtrinsicFailed(err) => {
                if let SigningStage::SigningSuccess {
                    submitting_stage, ..
                } = &mut self.stage
                {
                    *submitting_stage = SubmittingStage::Error(err)
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
                let _remark_call = polkadot::tx()
                    .system()
                    .remark(ctx.props().nft_id.as_bytes().to_vec());
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
                let on_input = ctx.link().callback(move |event: InputEvent| {
                    let input_element = event.target_dyn_into::<HtmlInputElement>().unwrap();
                    let value = input_element.value();
                    Message::ChangeMessage(value)
                });

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
            SigningStage::SigningSuccess {
                signature,
                signed_extrinsic_hex,
                submitting_stage,
                ..
            } => {
                let submitting_stage_html = match submitting_stage {
                    SubmittingStage::Initial { .. } => {
                        let submit_extrinsic_click =
                            ctx.link().callback(move |_| Message::SubmitSigned);
                        html!(<button onclick={submit_extrinsic_click}> {"=> Submit the signed extrinsic"} </button>)
                    }
                    SubmittingStage::Submitting => {
                        html!(<div class="loading"><b>{"Submitting Extrinsic... (please wait a few seconds)"}</b></div>)
                    }
                    SubmittingStage::Success { remark_event } => {
                        html!(<div style="overflow-wrap: break-word;"> <b>{"Successfully submitted Extrinsic. Event:"}</b> <br/> {format!("{:?}", remark_event)} </div>)
                    }
                    SubmittingStage::Error(err) => {
                        html!(<div class="error"> {"Error: "} {err.to_string()} </div>)
                    }
                };

                html!(
                    <>
                        {submitting_stage_html}
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

async fn submit_wait_finalized_and_get_extrinsic_success_event(
    extrinsic: SubmittableExtrinsic<PolkadotConfig, OnlineClient<PolkadotConfig>>,
) -> Result<polkadot::system::events::ExtrinsicSuccess, anyhow::Error> {
    let events = extrinsic
        .submit_and_watch()
        .await?
        .wait_for_finalized_success()
        .await?;

    let events_str = format!("{:?}", &events);
    web_sys::console::log_1(&events_str.into());
    for event in events.find::<polkadot::system::events::ExtrinsicSuccess>() {
        web_sys::console::log_1(&format!("{:?}", event).into());
    }

    let success = events.find_first::<polkadot::system::events::ExtrinsicSuccess>()?;
    success.ok_or(anyhow!("ExtrinsicSuccess not found in events"))
}
