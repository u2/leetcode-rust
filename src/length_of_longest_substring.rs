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
            if max < c.len() {
                max = c.len();
            };
        }
    }
    return max;
}

// Optimized
// https://leetcode.com/articles/longest-substring-without-repeating-characters/

use std::collections::HashMap;

pub fn approach_two(s: String) -> usize {
    let array = s.as_bytes();
    let mut reviews: HashMap<&u8, usize> = HashMap::with_capacity(array.len());
    let mut i: usize = 0;
    let mut index: usize = 0;
    let mut max: usize = 0;

    for x in array.iter() {
        if reviews.contains_key(x) && *reviews.get(x).unwrap() >= i {
            i = *reviews.get(x).unwrap() + 1;
        } else {
            if max < index - i + 1 {
                max = index - i + 1;
            }
        }
        reviews.insert(x, index);
        index += 1;
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

        assert_eq!(length_of_longest_substring(String::from("b")), 1);

        assert_eq!(length_of_longest_substring(String::from("")), 0);
    }

    use super::approach_two;

    #[test]
    fn test_approach_two() {
        assert_eq!(approach_two(String::from("world")), 5);

        assert_eq!(approach_two(String::from("hello world")), 6);

        assert_eq!(approach_two(String::from("helloworld")), 5);

        assert_eq!(approach_two(String::from("abcabcbb")), 3);

        assert_eq!(approach_two(String::from("bbbbb")), 1);

        assert_eq!(approach_two(String::from("pwwkew")), 3);

        assert_eq!(approach_two(String::from("b")), 1);

        assert_eq!(approach_two(String::from("")), 0);
    }
}
