use bech32::{self, FromBase32, ToBase32};

pub const MAX_ADDRESS_LENGTH: usize = 255;

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
                        Err(x) => return format!("Error:_1 {:?}", x),
                        Ok(_) => {
                            let check =
                                bech32::encode("juno", yyy, bech32::Variant::Bech32).unwrap();

                            if check == user_address {
                                return user_address.to_string();
                            } else {
                                return "Error_1: Please enter a valid Juno address".to_string();
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
