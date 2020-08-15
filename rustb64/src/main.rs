use std::env;

fn encoder() -> [char; 64] {
    [
        'A','B','C','D','E','F','G','H','I',
        'J','K','L','M','N','O','P','Q','R',
        'S','T','U','V','W','X','Y','Z',
        'a','b','c','d','e','f','g','h','i',
        'j','k','l','m','n','o','p','q','r',
        's','t','u','v','w','x','y','z',
        '0','1','2','3','4','5','6','7','8','9',
        '+','/'
    ]
}


fn decode(val:u8) -> u8{
    let mut value = val;
    if value > 64 && value < 91 { //capital alpha character
        value -= 65;
        return value;
    }else if value > 96 && value < 123{
        value -= 71; //Already has 26 because of the b64 conversion
        return value;
    }else{
        let mut index:usize = 52;
        let encoder = encoder();
        while index < encoder.len() { 
            if value as char == encoder[index] {
                return index as u8;
            }
            index += 1;
        }
        println!("Shouldn't ever get to this point, character not in allowed");
        return 0;
    }
}

fn base64_to_binary(base64:String)->String {
    let word = base64.clone().replace("=", "");
    let mut result = String::new();
    for character in word.clone().into_bytes() {
        let index = decode(character);
        let ext = &mut format!("{:b}", index);
        while (ext.chars().count() % 6) != 0 {
            *ext = "0".to_string() + &ext;
        }
        // println!("value:{}->{} => {}", value, ext, encoder()[value as usize]);
        result += ext;
    }
    while (result.chars().count() % 8) != 0 {
        result += "0";
    }
    return result;
}

fn to_binary(word:String)->String {
    let mut result = String::new();
    let mut cpy = String::new();
    for character in word.clone().into_bytes() {
        let ext = &mut format!("0{:b}", character);
        while (ext.chars().count() % 8) != 0 {
            *ext = "0".to_string() + &ext;
        }
        // println!("{} -> {}", character, ext);
        result += ext;
        cpy += ext;
    }
    while (result.chars().count() % 6) != 0 {
        result += "0";
        cpy += "0";
    }
    return result;
}

fn pad_binary_string(binary_str:String, len:usize){
    let pad = String::new();
    for _ binary_str.chars.count()..len{
        pad += "0";
    }
    if binary_str.chars.count() < len {
        *binary_str = pad + &ext;
    }

}


fn encode_to_base64(binary_str:String)->String {
    let length = binary_str.chars().count();
    let mut index = 0;
    let mut result = String::new();
    while index + 6 <= length {
        let set = (&binary_str[index..index+6]).to_string();
        result.push(encode_binary_to_char(set, 6));
        index += 6;
    }
    while (result.chars().count() % 3) != 0 {
        result += "=";
    }
    return result;
}

fn decode_to_utf8(binary_str:String)->String {
    let length = binary_str.chars().count();
    let mut index = 0;
    let mut result = String::new();
    while index + 8 <= length {
        let set = (&binary_str[index..index+8]).to_string();
        result.push(encode_binary_to_char(set, 8));
        index += 8;
    }
    return result;
}

fn encode_binary_to_char(char_binary:String, length:u32)->char{
    let mut index = 2_i32.pow(length-1);
    let mut result = 0;
    for bit in char_binary.clone().into_bytes() {
        result += if bit.to_string() == "49" { index } else { 0 };
        index /= 2;
    }
    // println!("{} -> result:{}", char_binary, result);
    if length == 6 {//convert
        return encoder()[result as usize];
    }else{//to ascii
        return result as u8 as char;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let encode_str = &args[1];
    println!("{}", encode_str);
    let mut result = encode_str.to_string();
    for _ in 0..30{
        result = to_binary(result);
        result = encode_to_base64(result);
        // println!("{}", result);
    }

    for _ in 0..30 {
        result = base64_to_binary(result);
        result = decode_to_utf8(result);
        // println!("{}", result);
    }
}

// Python
// real    0m0.210s
// user    0m0.190s
// sys     0m0.021s



// Rust 
// real    0m0.120s
// user    0m0.118s
// sys     0m0.002s


//30 runs
//Rust
// real    0m27.784s
// user    0m27.741s
// sys     0m0.042s

//python
// real    0m45.248s
// user    0m45.213s
// sys     0m0.031s