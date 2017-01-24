use regex::Regex;

pub fn regular_expression_matching(string: &str, reg: &str) -> bool {
    let re = Regex::new(["^", reg, "$"].join("").as_str()).unwrap();
    re.is_match(string)
}

#[cfg(test)]
mod test {
    use super::regular_expression_matching;

    #[test]
    fn test_regular_expression_matching() {
        assert_eq!(regular_expression_matching("aa", "a"), false);

        assert_eq!(regular_expression_matching("aa", "aa"), true);

        assert_eq!(regular_expression_matching("aaa", "aa"), false);

        assert_eq!(regular_expression_matching("aa", "a*"), true);

        assert_eq!(regular_expression_matching("aa", ".*"), true);

        assert_eq!(regular_expression_matching("ab", ".*"), true);

        assert_eq!(regular_expression_matching("aab", "c*a*b"), true);
    }
}
