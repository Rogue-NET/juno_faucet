use web_sys::HtmlInputElement;
use yew::prelude::*;

use gloo_net::http::*;

use wasm_bindgen::prelude::*;

use bech32::{self, FromBase32, ToBase32};
use serde::{Deserialize, Serialize};

pub const MAX_ADDRESS_LENGTH: usize = 255;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum AddressState {
    Good { address: String },
    NotGood { error1: String, error2: String },
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
        let address = input.value();
        let check1 = encode_decode(&address); 
        let check2 = verify_length(&address); 

        if check1 == check2 {
            let post = PostMessage {
                denom: "ujunox".to_string(),
                address: check2,
            };

            let zz = JsValue::from_serde(&post).unwrap();
            let pyr = zz.clone();

            if let Ok(y) = JsValue::from_serde(&post) {
                let opts = Request::new("https://faucet-api.roguenet.io/credit")

                    .method(Method::POST) // <<
                    .body(zz) // << 413


                    .header("Content-type", "application/json"); // <<< 400


                    //.method(Method::POST)
                    //.body(zz as JsValue);
                    //.credentials(RequestCredentials::Include)
                    //.mode(RequestMode::Cors);
                    //.header("??content-type??", "application/json");
                wasm_bindgen_futures::spawn_local( async move {
                    opts.send().await.unwrap();
                });


                web_sys::console::log_1(&pyr);
                web_sys::console::log_1(&"eeeeeeeee".into());


                //check_state.set(Some(Address::Good {address}));
            }

        } else {

            web_sys::console::log_1(&"hello".into());
            //check_state.set(Some(AddressState::NotGood {error1: "Parse Error".to_string(), error2: "Parse Error".to_string()}));
            
        };
    });

    html! {
        <>
            <h1>{ "Juno Faucet" }</h1>
            <h2>{ "Enter your testnet Juno address and click send" }</h2>
            <div class ="container">
                <input ref={input_ref_outer.clone()} type="text" id="address" placeholder="juno1..." autocomplete="off" />
                <button class ="button1" onclick={onclick}>{"Send"}</button>
            </div>
            <div class ="response_container">
            <ViewResponse address={(*check_state_outer).clone()} />
            </div>
            <body>
            </body>
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
        Some(AddressState::Good { address }) => format!("Funds sent to {:?}", address.clone()),
        Some(AddressState::NotGood { error1, error2 }) => {
            format!("{:?} /// {:?}", error1, error2)
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
                                return "Error_1: decode/encode failed. Please enter a valid Juno address".to_string();
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
