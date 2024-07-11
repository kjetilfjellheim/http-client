/**
 * URL decoding error. This error is thrown when
 * the URL encoded string can't be decoded to a normal
 * string. The error contains a message with the reason.
 */
#[derive(Debug)]
struct DecodeError {
    message: String
}

impl DecodeError {
    pub fn new(message: String) -> DecodeError {
        DecodeError {
            message
        }
    }
}

/**
 * Characters that don't need to be encoded. Im the URI 
 * standard, these are called "unreserved characters".
 * 
 * @See https://tools.ietf.org/html/rfc3986#section-2.3
 */
pub const UNRESERVED_CHARACTERS: [char; 66] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
    'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3',
    '4', '5', '6', '7', '8', '9', '-', '.', '_', '~',
];

/**
 * Encodes a string to a URL encoded string.
 * 
 * @param input The string to encode.
 * 
 * @return The URL encoded string.
 */
pub fn encode(input: &str) -> String {
    let mut result = String::new();
    for character in input.chars() {
        result.push_str(encode_char(&character).as_str());
    }
    result
}

/**
 * Encodes a character to a URL encoded string.
 * 
 * @param character The character to encode.
 * 
 * @return The URL encoded character.
 */
fn encode_char(character: &char) -> String {
    if UNRESERVED_CHARACTERS.contains(&character) {
        character.to_string()
    } else {
        let string = character.to_string();
        let bytes = string.as_bytes();
        encode_non_reserved_char(bytes)
    }
}

/**
 * Encodes a byte array of a non reserved
 * character to a URL encoded string.
 * 
 * @param bytes The byte array to encode.
 * 
 * @return The URL encoded string.
 */
fn encode_non_reserved_char(bytes: &[u8]) -> String {
    let mut result = String::new();
    for byte in bytes {            
        result.push('%');
        result.push_str(&format!("{:X}", byte));
    }
    result
}

/**
 * Decodes a URL encoded string to a normal string.
 * 
 * @param input The URL encoded string.
 * 
 * @return The normal string.
 */
pub fn decode(str: &str) -> Result<String, DecodeError> {
    let mut result = String::new();
    let mut chars = str.chars();
    while let Some(character) = chars.next() {
        if character == '%' {
            let bytes = get_bytes(&mut chars)?;
            let utf8 = match String::from_utf8(bytes) {
                Ok(utf8) => utf8,
                Err(err) => return Err(DecodeError::new(err.to_string()))
            };
            result.push_str(utf8.as_str());
        } else {
            result.push(character);
        }
    }
    Ok(result)
}

/**
 * Gets the bytes of a URL encoded character.
 * 
 * @param chars The iterator of characters.
 * 
 * @return The bytes of the URL encoded character.
 */
fn get_bytes(chars: &mut std::str::Chars) -> Result<Vec<u8>, DecodeError> {
    let mut bytes: Vec<u8> = Vec::new();
    let byte = get_byte(chars)?;
    bytes.push(byte);
    if byte >= 128 {
        chars.next();
        bytes.push(get_byte(chars)?);
    }
    Ok(bytes)
}

/**
 * Gets a byte from the URL encoded string.
 * 
 * @param chars The iterator of characters.
 * 
 * @return The byte.
 */
fn get_byte(chars: &mut std::str::Chars) -> Result<u8,DecodeError> {
    let char1 = match chars.next() {
        Some(char) => char,
        None => return Err(DecodeError::new("Unexpected end of string".to_string()))
    };
    let char2 = match chars.next() {
        Some(char) => char,
        None => return Err(DecodeError::new("Unexpected end of string".to_string()))
    };
    match u8::from_str_radix(format!("{}{}", char1, char2).as_str(), 16) {
        Ok(byte) => Ok(byte),
        Err(err) => Err(DecodeError::new(err.to_string()))
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_encode() {
        assert_eq!(super::encode("abcd"), "abcd");
        assert_eq!(super::encode("http://localhost:8080"), "http%3A%2F%2Flocalhost%3A8080");
        assert_eq!(super::encode("Hello Günter"), "Hello%20G%C3%BCnter");
        assert_eq!(super::encode("æøåÆØÅ,.-;:_!\"#¤%"), "%C3%A6%C3%B8%C3%A5%C3%86%C3%98%C3%85%2C.-%3B%3A_%21%22%23%C2%A4%25");
    }

    #[test]
    fn test_decode() {
        assert_eq!(super::decode("abcd").unwrap(), "abcd");
        assert_eq!(super::decode("http%3A%2F%2Flocalhost%3A8080").unwrap(), "http://localhost:8080");
        assert_eq!(super::decode("Hello%20G%C3%BCnter").unwrap(), "Hello Günter");
        assert_eq!(super::decode("%C3%A6%C3%B8%C3%A5%C3%86%C3%98%C3%85%2C.-%3B%3A_%21%22%23%C2%A4%25").unwrap(), "æøåÆØÅ,.-;:_!\"#¤%");
    }
}