// By using HashSet as a sliding window,
// checking if a character in the current can be done in O(1)O(1).

use std::collections::VecDeque;

pub fn length_of_longest_substring(s: &str) -> usize {
    let array: Vec<(char)> = s.char_indices().map(|i| i.1).collect::<Vec<_>>();
    let mut c: VecDeque<(char)> = VecDeque::with_capacity(s.char_indices().count());
    let mut max: usize = 0;

    for x in array.iter() {
        if c.contains(x) {
            let mut z: Option<char> = c.pop_front();
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
// The above solution requires at most 2n steps.
// In fact, it could be optimized to require only n steps.
// Instead of using a set to tell if a character exists or not,
// we could define a mapping of the characters to its index.
// Then we can skip the characters immediately when we found a repeated character.

use std::collections::HashMap;

pub fn approach_two(s: &str) -> usize {
    let array: Vec<(char)> = s.char_indices().map(|i| i.1).collect::<Vec<_>>();
    let mut reviews: HashMap<&char, usize> = HashMap::with_capacity(s.char_indices().count());
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
        assert_eq!(length_of_longest_substring("world"), 5);

        assert_eq!(length_of_longest_substring("hello world"), 6);

        assert_eq!(length_of_longest_substring("helloworld"), 5);

        assert_eq!(length_of_longest_substring("abcabcbb"), 3);

        assert_eq!(length_of_longest_substring("bbbbb"), 1);

        assert_eq!(length_of_longest_substring("pwwkew"), 3);

        assert_eq!(length_of_longest_substring("b"), 1);

        assert_eq!(length_of_longest_substring(""), 0);

        assert_eq!(length_of_longest_substring("大大小小"), 2);
    }

    use super::approach_two;

    #[test]
    fn test_approach_two() {
        assert_eq!(approach_two("world"), 5);

        assert_eq!(approach_two("hello world"), 6);

        assert_eq!(approach_two("helloworld"), 5);

        assert_eq!(approach_two("abcabcbb"), 3);

        assert_eq!(approach_two("bbbbb"), 1);

        assert_eq!(approach_two("pwwkew"), 3);

        assert_eq!(approach_two("b"), 1);

        assert_eq!(approach_two(""), 0);

        assert_eq!(approach_two("大大小小"), 2);
    }

    use test::Bencher;
    // 96 ns/iter (+/- 26)
    #[bench]
    fn bench_length_of_longest_substring(b: &mut Bencher) {
        b.iter(|| length_of_longest_substring("hello world"));
    }

    // 561 ns/iter (+/- 180)
    #[bench]
    fn bench_approach_two(b: &mut Bencher) {
        b.iter(|| approach_two("hello world"));
    }
}
