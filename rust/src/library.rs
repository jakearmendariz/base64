const ENCODER:[char; 64] = [
    'A','B','C','D','E','F','G','H','I',
    'J','K','L','M','N','O','P','Q','R',
    'S','T','U','V','W','X','Y','Z',
    'a','b','c','d','e','f','g','h','i',
    'j','k','l','m','n','o','p','q','r',
    's','t','u','v','w','x','y','z',
    '0','1','2','3','4','5','6','7','8','9',
    '+','/'
];

pub fn base64_to_binary(base64:String)->String {
    let word = base64.clone().replace("=", "");
    let mut result = String::new();
    for character in word.clone().into_bytes() {
        let binary_str = &mut format!("{:b}", decode(character));
        result  += &format!("{:0>6}", binary_str)[..];
    }
    while (result.chars().count() % 8) != 0 {
        result += "0";
    }
    return result;
}

pub fn utf8_to_binary(word:String)->String {
    let mut result = String::new();
    for character in word.clone().into_bytes() {
        let binary_str = &mut format!("0{:b}", character);
        result += &format!("{:0>8}", binary_str)[..];
    }
    while (result.chars().count() % 6) != 0 {
        result += "0";
    }
    return result;
}

pub fn encode_to_base64(binary_str:String)->String {
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

pub fn decode_to_utf8(binary_str:String)->String {
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
        return ENCODER[result as usize];
    }else{//to ascii
        return result as u8 as char;
    }
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
        while index < ENCODER.len() { 
            if value as char == ENCODER[index] {
                return index as u8;
            }
            index += 1;
        }
        println!("Shouldn't ever get to this point, character not in allowed");
        return 0;
    }
}
