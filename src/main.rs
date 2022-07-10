mod app;
use app::App;

fn main() {
    yew::start_app::<App>();
}



// Simple UNIT test to ensure logic works. can be removed for prod
//#[test]
//fn test1 () {
//
//    let input_addr = "juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y".to_string();
//
//    let (_hrp, data, _variant) = bech32::decode(&input_addr).unwrap(); //decode input address to u5
//
//    let mut xxx = Vec::<u8>::from_base32(&data).unwrap(); // convert bytes to u8
//
//    let yyy = Vec::to_base32(&mut xxx); // convert bytes back to u5
//
//    let check = bech32::encode("juno", yyy, bech32::Variant::Bech32).unwrap(); // encode u5 bytes to String address
//
//    let should_fail_random_text = "alsdfjdlsfjsalk".to_string();
//
//    let should_fail_incorrect_address = "juno1sjllsnramtg3ewxqwwrwjxfgc4n4ef9uee0aeq";
//
//    assert_eq!(input_addr, check);
//    assert_ne!(input_addr, should_fail_random_text);
//    assert_ne!(input_addr, should_fail_incorrect_address);
//}