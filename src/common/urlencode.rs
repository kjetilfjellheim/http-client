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
        let mut result = String::new();
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

#[cfg(test)]
mod test {

    #[test]
    fn test_encode() {
        assert_eq!(super::encode("http://localhost:8080"), "http%3A%2F%2Flocalhost%3A8080");
        assert_eq!(super::encode("Hello Günter"), "Hello%20G%C3%BCnter");
        assert_eq!(super::encode("æøåÆØÅ,.-;:_!\"#¤%"), "%C3%A6%C3%B8%C3%A5%C3%86%C3%98%C3%85%2C.-%3B%3A_%21%22%23%C2%A4%25");
    }
}