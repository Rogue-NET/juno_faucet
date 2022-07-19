use crate::verify;

use yew::prelude::*;

use gloo_net::http::*;
use web_sys::HtmlInputElement;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum AddressState {
    Good { address: String },
    NotGood { error1: String },
    Processing { message: String },
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct PostMessage {
    denom: String,
    address: String,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum FundsState {
    Available { amount: String },
    NotEnough { amount: String },
    Error { error_message: String },
    Checking { msg: String },
}

#[function_component(App)]
pub fn app() -> Html {
    let check_state = use_state_eq::<Option<AddressState>, _>(|| None);
    let check_state_outer = check_state.clone();

    let check_funds = use_state_eq::<Option<FundsState>, _>(|| None);
    let check_funds_outer = check_funds.clone();

    let check_button = use_state(|| false);
    let checker = use_state(|| true);

    let input_ref = NodeRef::default();
    let input_ref_outer = input_ref.clone();

    let onclick_get = Callback::from(move |_| {
        let check_funds_clone = check_funds.clone();
        check_funds_clone.set(Some(FundsState::Checking {
            msg: "One sec...".to_string(),
        }));

        wasm_bindgen_futures::spawn_local(async move {
            match Request::new("https://faucet-api.roguenet.io/status")
                .method(Method::GET)
                .send()
                .await
            {
                Ok(x) => match x.text().await {
                    Ok(y) => {
                        
                        let z: Vec<_> = y.split('"').collect();

                        if ((z[35].parse::<i64>().unwrap()) / 1000000) > 20 {
                            check_funds_clone.set(Some(FundsState::Available {
                                amount: ((z[35].parse::<i64>().unwrap()) / 1000000)
                                    .to_string(),
                            }));
                        } else {
                            check_funds_clone.set(Some(FundsState::NotEnough {
                                amount: ((z[35].parse::<i64>().unwrap()) / 1000000)
                                    .to_string(),
                            }));
                        }
                    },

                    Err(_) => {
                        check_funds_clone.set(Some(FundsState::Error {
                            error_message: "Parse error, please try again".to_string(),
                        }));
                    }
                },

                Err(_) => {
                    check_funds_clone.set(Some(FundsState::Error {
                        error_message: "No response from server, please try again".to_string(),
                    }));
                }
            }
        });
    });

    let onclick = Callback::from(move |_| {
        let check_state_clone = check_state.clone();
        let input = input_ref.cast::<HtmlInputElement>().unwrap();
        let address = input.value();

        if check_button.eq(&checker) {
            check_state_clone.set(Some(AddressState::Processing {
                message:
                    "Cooldown triggered to prevent spam. Please refresh your browser and try again."
                        .to_string(),
            }));
            return;
        } else {
            check_button.set(true);
        };

        check_state_clone.set(Some(AddressState::Processing {
            message: "Processing your request, usually takes about 10 seconds...".to_string(),
        }));

        let check1 = verify::encode_decode(&address);
        let check2 = verify::verify_length(&address);

        if check1 == check2 {
            let post = PostMessage {
                denom: "ujunox".to_string(),
                address: address.clone(),
            };

            if JsValue::from_serde(&post).is_ok() {
                let opts = Request::new("https://faucet-api.roguenet.io/credit")
                    .json(&post)
                    .unwrap()
                    .header(
                        "Content-Security-Policy",
                        "script-src none; connect-src *.roguenet.io; default-src *.roguenet.io",
                    )
                    .method(Method::POST);

                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(x) = opts.send().await {
                        let rez = x.status_text();
                        if rez == *"OK".to_string() {
                            check_state_clone.set(Some(AddressState::Good { address }));
                        } else if rez == *"Method Not Allowed".to_string() {
                            check_state_clone.set(Some(AddressState::NotGood {
                                error1: "wow so thirsty...please wait 1 hour and try again"
                                    .to_string(),
                            }));
                        } else {
                            check_state_clone.set(Some(AddressState::NotGood {
                                error1: "Something went wrong...Please try again".to_string(),
                            }));
                        }
                    }
                });
            }
        } else {
            check_state_clone.set(Some(AddressState::NotGood {
                error1: format!("{} /// {}", check1, check2),
            }));
        };
    });

    html! {
        <>
            <h1>{ "Juno Faucet" }</h1>
            <h2>{ "drip drop gimme some junox" }</h2>
            <div class ="container">
                <input ref={input_ref_outer.clone()} type="text" id="address" placeholder="juno1..." autocomplete="off" />
                <button class ="button1" onclick={onclick}>{"Send"}</button>
            </div>
            <div class ="response_container">
                <ViewResponse address={(*check_state_outer).clone()} />
            </div>
            <div class ="container2" style="inline">
                <ViewFunds funds={(*check_funds_outer).clone()} />
                <button class="button2" onclick={onclick_get}>{"Status"}</button>
            </div>

            <div class ="footer">
                <p>{ "Built by:     "}
                    <a href="https://twitter.com/roguenet_">{ "   RogueNET"}</a>
                    { "  | Powered by:     "}
                    <a href="https://junonetwork.io/">{ " Juno Network" }</a>
                    { "   +   " }
                    <a href="https://github.com/cosmos/cosmjs">{ "  cosmjs" }</a>
                </p>
            </div>


        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct ViewFundsAvailable {
    funds: Option<FundsState>,
}

#[function_component(ViewFunds)]
fn view_funds(funds: &ViewFundsAvailable) -> Html {
    let funds_message = match &funds.funds {
        None => return html! {},
        Some(FundsState::Checking { msg }) => msg.to_string(),
        Some(FundsState::Available { amount }) => {
            format!("Junox available is: {} ||| Time to get drippy", amount)
        }
        Some(FundsState::NotEnough { amount }) => format!(
            "Junox available is: {} ||| Please wait until this is 20 or more",
            amount
        ),
        Some(FundsState::Error { error_message }) => format!("Error message: {}", error_message),
    };

    html! {
        <div>{funds_message}</div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ViewAddressProperties {
    address: Option<AddressState>,
}

#[function_component(ViewResponse)]
fn view_response(props: &ViewAddressProperties) -> Html {
    let response = match &props.address {
        None => return html! {},
        Some(AddressState::Processing { message }) => message.to_string(),
        Some(AddressState::Good { address }) => format!("Funds sent to {}", address.clone()),
        Some(AddressState::NotGood { error1 }) => error1.to_string(),
    };

    html! {
        <div>{ response }</div>
    }
}
