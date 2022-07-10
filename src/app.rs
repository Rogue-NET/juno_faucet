use yew::prelude::*;
use web_sys::HtmlInputElement;

use gloo_net::http::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::*;

use futures::executor::block_on;
use tokio::runtime::Runtime;

use serde::{Deserialize, Serialize};
use bech32::{self, FromBase32, ToBase32};

pub const MAX_ADDRESS_LENGTH: usize = 255;



#[derive(PartialEq, Debug, Clone)]
pub enum AddressState {
    Good {address: String, http_response: Option<String>},
    NotGood {error1: String, error2: String},
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct PostMessage {
    denom: String,
    address: String,
}

#[derive(Debug)]
pub struct PostMessageResponse {
    val: Result<Response, gloo_net::Error>,
}

#[derive(Debug)]
pub struct UpdateTheState {
    update_value: Option<AddressState>,
}



#[function_component(App)]
pub fn app() -> Html {

    //let onclick1 = Callback::from(|mouse_event: MouseEvent| { // used to check console, can be removed for prod
    //    web_sys::console::log_1(&mouse_event)
    //});

    let mut out_of_closure_test = UpdateTheState {
        update_value: None,
    };

    let check_state = use_state_eq::<Option<AddressState>, _>(|| None);
    let check_state_outer = check_state.clone();

    let input_ref = NodeRef::default();
    let input_ref_outer = input_ref.clone();

    let onclick = Callback::from(move |_| { 
        let input = input_ref.cast::<HtmlInputElement>().unwrap();
        let address = input.value();
        let check1 = encode_decode(&address); // Will be Address if check passes, error otherwise
        let check2 = verify_length(&address); // Will be Address if check passes, error otherwise

        if check1 == check2 {

            //check_state.set(Some(AddressState::Good {address: check1, http_response: None}));
            out_of_closure_test.update_value = Some(AddressState::Good {address: check1, http_response: None}); // doesn't work

            // Create post message for request to faucet
            let post = PostMessage {
                denom: "ujuno".to_string(),
                address: check2,
            };
            
            // Verify post to JSON & send to faucet
            match JsValue::from_serde(&post) {

                Ok(x) => {

                    let opts = Request::new("https://httpbin.org/post")
                    .method(Method::POST)
                    .body(Some(x as JsValue))
                    .credentials(RequestCredentials::Include)
                    .mode(RequestMode::Cors)
                    .header("??content-type??", "application/json");
                    wasm_bindgen_futures::spawn_local( async {
                        match opts.send().await {
                            Ok(y) => {
                                check_state.set(Some(AddressState::Good {address, http_response: Some(y.status_text()) }));
                            },
                            Err(error) => {
                                check_state.set(Some(AddressState::NotGood {error1: error.to_string(), error2: error.to_string() }));
                            }
                        }
                    });
                },
                Err(_) => {
                    check_state.set(Some(AddressState::NotGood {error1: "Parsing Error".to_string(), error2: "Parsing Error".to_string()}));
                },
            }

        } else {
            check_state.set(Some(AddressState::NotGood {error1: check1, error2: check2}));
            // Display Both Errors to front end
            // do nothing
        }
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
    address: Option<AddressState>
}

#[function_component(ViewResponse)]
fn view_response(props: &ViewAddressProperties) -> Html {
    let response = match &props.address {
        None => return html!{},
        Some(AddressState::Good{address, http_response}) => format!("Funds sent to {:?} /// Post response {:?}", address.clone(), http_response.clone()),
        Some(AddressState::NotGood{error1, error2}) => {
            format!("{:?} /// {:?}", error1, error2)
        }
    };

    html! {
        <div>{ response }</div>
    }
}

pub fn encode_decode (user_address: &str) -> String {

    // Reasoning for using so many nested matches is that each function inside them
    // can potentially return a different Bech32::Error type, but grug dev turt
    // think maybe some big brain idomatic way to rewrite this

    // Error: InvalidChecksum - hits when someone enters a non-juno address
    // Error: MixedCase - hits when mixed case (usually would be a typo)
    // Error: decode/encode failed - Hits when someone enters a valid CW address but non Juno address (like a cosmos address)

    match bech32::decode(user_address) {
        Err(x) => return {
            format!("Error_1: {:?}", x)},
        Ok(_) => {
            let (_hrp, data, _variant) = bech32::decode(user_address).unwrap();

            match Vec::<u8>::from_base32(&data) {
                Err(x) => return {
                    format!("Error_1: {:?}", x)},
                Ok(_) => {
                    let mut xxx = Vec::<u8>::from_base32(&data).unwrap();

                    let yyy = Vec::to_base32(&mut xxx);

                    match bech32::encode("juno", yyy.clone(), bech32::Variant::Bech32) {
                        Err(x) => return {
                            format!("Error_1: {:?}", x)},
                        Ok(_) => {
                            let check = bech32::encode("juno", yyy, bech32::Variant::Bech32).unwrap();

                            if &check == user_address {
                                return user_address.to_string()
                            } else {
                                return "Error_1: decode/encode failed. Please enter a valid Juno address".to_string()
                            }
                        }
                        
                    }
                }
            }
        }
    };
}


pub fn verify_length (user_address: &str) -> String {

    // Error if length of bytes > 255 (max length of CW bech32 address)

    match bech32::decode(user_address) {
        Err(x) => return {
            format!("Error_2: {:?}", x)},
        Ok(_) => {
            let (_hrp, data, _variant) = bech32::decode(user_address).unwrap();

            if !matches!(data.len(), 1..=MAX_ADDRESS_LENGTH) {
                return "Error_2: Address is an invalid length".to_string()
            } else {
                return user_address.to_string()
            }
        }
    }

}
