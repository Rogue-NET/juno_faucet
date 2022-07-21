use yew::prelude::Properties;
use serde::{Serialize, Deserialize};


#[derive(Properties, PartialEq)]
pub struct ViewFundsAvailable {
    pub funds: Option<FundsState>,
}

#[derive(Properties, PartialEq)]
pub struct ViewAddressProperties {
    pub address: Option<AddressState>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct PostMessage {
    pub denom: String,
    pub address: String,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum AddressState {
    Good { address: String },
    NotGood { error1: String },
    Processing { message: String },
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum FundsState {
    Available { amount: String },
    NotEnough { amount: String },
    Error { error_message: String },
    Checking { msg: String },
}
