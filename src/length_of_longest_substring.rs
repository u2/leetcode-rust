use std::collections::VecDeque;

pub fn length_of_longest_substring(s: String) -> usize {
    let array = s.as_bytes();
    let mut c: VecDeque<u8> = VecDeque::with_capacity(array.len());
    let mut max: usize = 0;
    let mut z: Option<u8>;

    for x in array.iter() {
        if c.contains(x) {
            z = c.pop_front();
            while z != Some(*x) {
                z = c.pop_front();
            }
            c.push_back(*x);
        } else {
            c.push_back(*x);
            println!("c: {:?}", c);
            if max < c.len(){ max = c.len(); };
        }
    }
    return max;
}

#[cfg(test)]
mod test {
    use super::length_of_longest_substring;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring(String::from("world")), 5);

        assert_eq!(length_of_longest_substring(String::from("hello world")), 6);

        assert_eq!(length_of_longest_substring(String::from("helloworld")), 5);

        assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
        assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
        assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
    }
}
