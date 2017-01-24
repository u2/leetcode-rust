pub fn reverse_integer(x: i32) -> i32 {
    let mut s: Vec<char> = x.to_string().chars().collect();
    s.reverse();
    return s.into_iter().collect::<String>().parse::<i32>().unwrap();
}

#[cfg(test)]
mod test {
    use super::reverse_integer;

    #[test]
    fn test_reverse_integer() {
        assert_eq!(reverse_integer(123 as i32), 321);

        assert_eq!(reverse_integer(1230 as i32), 321);
    }
}
