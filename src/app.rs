use web_sys::HtmlInputElement;
use yew::prelude::*;

use gloo_net::http::*;

use wasm_bindgen::prelude::*;

//use futures::{try_join, select};

use bech32::{self, FromBase32, ToBase32};
use serde::{Deserialize, Serialize};

pub const MAX_ADDRESS_LENGTH: usize = 255;



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





#[function_component(App)]
pub fn app() -> Html {
    let check_state = use_state_eq::<Option<AddressState>, _>(|| None);
    let check_state_outer = check_state.clone();

    let input_ref = NodeRef::default();
    let input_ref_outer = input_ref.clone();

    let onclick = Callback::from(move |_| {
        let input = input_ref.cast::<HtmlInputElement>().unwrap();
        let check_state_clone = check_state.clone();
        check_state_clone.set(Some(AddressState::Processing{ message: "Processing your request, usually takes about 10 seconds...".to_string()}));
        let address = input.value();

        let check1 = encode_decode(&address); 
        let check2 = verify_length(&address);

        if check1 == check2 {
            let post = PostMessage {
                denom: "ujunox".to_string(),
                address: check2,
            };

            if let Ok(_) = JsValue::from_serde(&post) {
                let opts = Request::new("https://faucet-api.roguenet.io/credit")
                    .json(&post).unwrap()
                    .header("Content-Security-Policy", "'self'")
                    .method(Method::POST);

                wasm_bindgen_futures::spawn_local( async move {

                    if let Ok(x) = opts.send().await {
                        let rez = x.status_text();
                        if rez == "OK".to_string() {
                            check_state_clone.set(Some(AddressState::Good {address}));
                        } else if rez == "Method Not Allowed".to_string() {
                            check_state_clone.set(Some(AddressState::NotGood {error1: "Wow thirsty dev...Please wait 1 hour and try again".to_string()}));
                        } else {
                            check_state_clone.set(Some(AddressState::NotGood {error1: "Something went wrong...Please try again".to_string()}));
                        }
                    }

                });

            }

        } else {
            check_state_clone.set(Some(AddressState::NotGood {error1: format!("{} /// {}", check1, check2)}));
            
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
            <body>
            </body>
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
pub struct ViewAddressProperties {
    address: Option<AddressState>,
}

#[function_component(ViewResponse)]
fn view_response(props: &ViewAddressProperties) -> Html {
    let response = match &props.address {
        None => return html! {},
        Some(AddressState::Processing { message }) => format!("{}", message),
        Some(AddressState::Good { address }) => format!("Funds sent to {}", address.clone()),
        Some(AddressState::NotGood { error1 }) => {
            format!("{}", error1)
        }
    };

    html! {
        <div>{ response }</div>
    }
}

pub fn encode_decode(user_address: &str) -> String {
    // Error: InvalidChecksum - hits when someone enters a non-juno address
    // Error: MixedCase - hits when mixed case (usually would be a typo)
    // Error: decode/encode failed - Hits when someone enters a valid CW address but non Juno address (like a cosmos address)

    match bech32::decode(user_address) {
        Err(x) => return format!("Error_1: {:?}", x),
        Ok(_) => {
            let (_hrp, data, _variant) = bech32::decode(user_address).unwrap();

            match Vec::<u8>::from_base32(&data) {
                Err(x) => return format!("Error_1: {:?}", x),
                Ok(_) => {
                    let xxx = Vec::<u8>::from_base32(&data).unwrap();

                    let yyy = Vec::to_base32(&xxx);

                    match bech32::encode("juno", yyy.clone(), bech32::Variant::Bech32) {
                        Err(x) => return format!("Error_1: {:?}", x),
                        Ok(_) => {
                            let check =
                                bech32::encode("juno", yyy, bech32::Variant::Bech32).unwrap();

                            if check == user_address {
                                return user_address.to_string();
                            } else {
                                return "Error_1: Decode/encode failed. Please enter a valid Juno address".to_string();
                            }
                        }
                    }
                }
            }
        }
    };
}

pub fn verify_length(user_address: &str) -> String {
    // Error if length of bytes > 255 (max length of CW bech32 address)

    match bech32::decode(user_address) {
        Err(x) => return format!("Error_2: {:?}", x),
        Ok(_) => {
            let (_hrp, data, _variant) = bech32::decode(user_address).unwrap();

            if !matches!(data.len(), 1..=MAX_ADDRESS_LENGTH) {
                return "Error_2: Address is an invalid length".to_string();
            } else {
                return user_address.to_string();
            }
        }
    }
}
