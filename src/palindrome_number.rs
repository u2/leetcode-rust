pub fn palindrome_number(mut i: i32) -> bool {
    if i < 0 || (i > 0 && i % 10 == 0) {
        return false;
    }
    let mut x: i32 = 0;
    while i > x {
        x = x * 10 + i % 10;
        i = i / 10;
    }
    return x == i || x / 10 == i;
}

#[cfg(test)]
mod test {
    use super::palindrome_number;

    #[test]
    fn test_palindrome_number() {
        assert_eq!(palindrome_number(123 as i32), false);

        assert_eq!(palindrome_number(1230 as i32), false);

        assert_eq!(palindrome_number(0 as i32), true);

        assert_eq!(palindrome_number(-3 as i32), false);

        assert_eq!(palindrome_number(1331 as i32), true);
    }
}
