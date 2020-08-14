
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

fn to_binary(word:String)->String {
    let mut result = String::new();
    let mut cpy = String::new();
    for character in word.clone().into_bytes() {
        let ext = &mut format!("0{:b}", character);
        while (ext.chars().count() % 8) != 0 {
            ext.push('0');
        }
        result += ext;
        cpy += ext;
    }
    while (result.chars().count() % 6) != 0 {
        result += "0";
        cpy += "0";
    }
    println!("{}", cpy);
    return result;
}

fn encode_to_base64(binary_str:String)->String {
    let length = binary_str.chars().count();
    let mut index = 0;
    let mut result = String::new();
    while index + 6 <= length {
        // let mut set = String::new();
        let set = (&binary_str[index..index+6]).to_string();
        result.push(encode_binary_to_char(set));
        index += 6;
    }
    while (result.chars().count() % 3) != 0 {
        result += "=";
    }
    return result;
}

fn encode_binary_to_char(char_binary:String)->char{
    println!("set:{}", char_binary);
    let mut index = 32;
    let mut result = 0;
    for bit in char_binary.clone().into_bytes() {
        // println!("bit.to_string() = {}", bit.to_string());
        result += if bit.to_string() == "49" { index } else { 0 };
        index /= 2;
    }
    println!("result:{}", result);
    return encoder()[result];
}

fn main() {
    let encode_str = "Jake Armendariz!";
    let binary_str = to_binary(encode_str.to_string());
    println!("{}", binary_str);
    let result = encode_to_base64(binary_str);

    println!("{}", result);
}

// 01001010 01100001 01101011 01100101 0100000 01000001 01110010 01101101 01100101 01101110 01100100 01100001 01110010 01101001 01111010 0
// 01001010 01100001 01101011 01100101 00100000 01000001 01110010 01101101 01100101 01101110 01100100 01100001 01110010 01101001 01111010
// 01001010 01100001 01101011 01100101 01000000 01000001 01110010 01101101 01100101 01101110 01100100 01100001 01110010 01101001 01111010