use std::cmp::max;

const BASE64_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
    'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z', '0', '1', '2', '3',
    '4', '5', '6', '7', '8', '9', '+', '/'
];

pub fn base64_encode(str: &str) -> String {
    let mut base64_encoded = String::new();
    let mut current_bits_stored: u8 = 0;
    let mut bit_storage: u8 = 0;
    for byte_val in str.as_bytes().iter() { 
        let new_bits_push: u8 = 6 - current_bits_stored;

        bit_storage <<= new_bits_push;
        bit_storage ^= byte_val>>(8 - new_bits_push);
        bit_storage = 0b00111111 & bit_storage;
        // Pushing first new_bits_push bits
        current_bits_stored += new_bits_push;

        // Get base64 encoded value
        base64_encoded.push(BASE64_TABLE[bit_storage as usize]);                  
        current_bits_stored -= 6;
        let remaining_bits: u8 = 8 - new_bits_push;
        // Push remaining bits
        current_bits_stored += remaining_bits;
        
        bit_storage <<= new_bits_push;
        bit_storage = (byte_val<<new_bits_push)>>new_bits_push;
        bit_storage = 0b00111111 & bit_storage;

        if current_bits_stored >= 6 {
            base64_encoded.push(BASE64_TABLE[bit_storage as usize]);                  

            bit_storage = 0;
            current_bits_stored = 0;
        }
    };
    // Add remaining
    let padding = str.as_bytes().len() * 8;
    if (padding % 6) != 0 {
        bit_storage = bit_storage << (6 - current_bits_stored);
        base64_encoded.push(BASE64_TABLE[bit_storage as usize]);
    }
    //Add padding
    for i in 0..((str.as_bytes().len() * 8) % 3) {
        base64_encoded.push('=');
    }
    
    base64_encoded               

}

#[cfg(test)]
mod test {
 
    use super::*;

    #[test]
    fn test_base64_encode() {
        assert_eq!("VGVzdGluZzEwMA==", base64_encode("Testing100"));
        assert_eq!("VGVzdGluZzEw", base64_encode("Testing10"));
        assert_eq!("VGVzdGluZzE=", base64_encode("Testing1"));                   
        assert_eq!("VGVzdGluZw==", base64_encode("Testing"));
        assert_eq!("VGVzdGlu", base64_encode("Testin"));
        assert_eq!("", base64_encode(""));
        assert_eq!("VGhpcyBpcyBhIHZlcnkgbG9uZyBsaW5lLg==", base64_encode("This is a very long line."));
    }

}