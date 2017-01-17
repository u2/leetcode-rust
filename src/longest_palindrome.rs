// Brute Force
pub fn longest_palindrome(input: &str) -> &str {
    let length = input.len();
    let mut sub: &str;
    for i in 0..length {
        for j in 0..i + 1 {
            sub = &input[j..length - i + j];
            if is_palindrome(sub.to_string()) {
                return sub;
            }
        }
    }
    return &input[0..0];
}

pub fn is_palindrome(s: String) -> bool {
    let mut rs = s.clone().into_bytes();
    rs.reverse();
    return rs == s.into_bytes();
}

#[cfg(test)]
mod test {
    use super::is_palindrome;
    use super::longest_palindrome;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(String::from("aba")), true);
        assert_eq!(is_palindrome(String::from("ab")), false);
        assert_eq!(is_palindrome(String::from("a")), true);
        assert_eq!(is_palindrome(String::from("abab")), false);
    }

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(longest_palindrome("babad"), "bab");
        assert_eq!(longest_palindrome("abccba"), "abccba");
        assert_eq!(longest_palindrome("cabccbad"), "abccba");
        assert_eq!(longest_palindrome("xbacabccbaddab"), "abccba");
        assert_eq!(longest_palindrome("abccbaddxe"), "abccba");
        assert_eq!(longest_palindrome("ddxeabccba"), "abccba");
    }
}
