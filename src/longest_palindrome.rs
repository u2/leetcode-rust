// Brute Force
pub fn longest_palindrome(input: &str) -> &str {
    let length = input.chars().count();
    let mut sub: &str;
    let mut char_i: usize;
    let mut char_j: usize;
    let mut jth: usize;
    for i in 0..length {
        for j in 0..i + 1 {
            char_i = input.char_indices().nth(j).unwrap().0;
            jth = length - i + j - 1;
            if jth == length - 1 {
                char_j = input.len();
            } else {
                char_j = input.char_indices().nth(jth + 1).unwrap().0;
            }

            sub = &input[char_i..char_j];
            if is_palindrome(sub) {
                return sub;
            }
        }
    }
    return &input[0..0];
}

pub fn is_palindrome(s: &str) -> bool {
    let rs = s.chars().rev().collect::<String>();
    return rs == s;
}

#[cfg(test)]
mod test {
    use super::is_palindrome;
    use super::longest_palindrome;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome("aba"), true);
        assert_eq!(is_palindrome("ab"), false);
        assert_eq!(is_palindrome("a"), true);
        assert_eq!(is_palindrome("abab"), false);
        assert_eq!(is_palindrome("小小"), true);
    }

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(longest_palindrome("babad"), "bab");
        assert_eq!(longest_palindrome("abccba"), "abccba");
        assert_eq!(longest_palindrome("cabccbad"), "abccba");
        assert_eq!(longest_palindrome("xbacabccbaddab"), "abccba");
        assert_eq!(longest_palindrome("abccbaddxe"), "abccba");
        assert_eq!(longest_palindrome("ddxeabccba"), "abccba");
        assert_eq!(longest_palindrome("a大大小小大大大"),
                   "大大小小大大");
    }
}
