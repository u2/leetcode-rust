pub fn atoi(string: &str) -> Option<i32> {
    let mut f: bool = false;
    let mut res: Vec<char> = Vec::with_capacity(string.chars().count());
    for c in string.chars().into_iter() {
        match c {
            '-' | '+' => res.push(c),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                res.push(c);
                f = true;
            }
            ' ' => {}
            _ => {
                if !f {
                    return None;
                }
            }
        }
    }
    if f {
        Some(res.into_iter().collect::<String>().parse::<i32>().unwrap())
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::atoi;

    #[test]
    fn test_atoi() {
        assert_eq!(atoi("123"), Some(123));

        assert_eq!(atoi(" 123"), Some(123));

        assert_eq!(atoi(" +123"), Some(123));

        assert_eq!(atoi(" -123"), Some(-123));

        assert_eq!(atoi(" -a123"), None);

        assert_eq!(atoi(" -123a44"), Some(-12344));

        assert_eq!(atoi(" -xfd"), None);

        assert_eq!(atoi("    "), None);
    }
}
