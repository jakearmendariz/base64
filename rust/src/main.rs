use std::env;
mod library;
const NUM_TRIALS:u8 = 20;

fn main() {
    let args: Vec<String> = env::args().collect();
    let encode_str = &args[1];
    println!("{}", encode_str);
    
    let mut result = encode_str.to_string();
    for _ in 0..NUM_TRIALS {
        result = library::utf8_to_binary(result);
        result = library::encode_to_base64(result);
    }
    for _ in 0..NUM_TRIALS {
        result = library::base64_to_binary(result);
        result = library::decode_to_utf8(result);
    }
    println!("{}", result);
}
