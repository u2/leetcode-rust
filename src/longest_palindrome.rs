// Brute Force
// Time complexity: n^3 (walking through needs n2, reversing needs n)

use std::cmp;

pub fn longest_palindrome(input: &str) -> &str {
    let length = input.chars().count();
    let s = input.char_indices().collect::<Vec<_>>();
    let len = input.len();

    for i in 0..length {
        for j in 0..i + 1 {
            let char_i = s[j].0;
            let jth = length - i + j - 1;
            let char_j: usize;
            if jth == length - 1 {
                char_j = len;
            } else {
                char_j = s[jth + 1].0;
            }

            let sub = &input[char_i..char_j];
            if is_palindrome(sub) {
                return sub;
            }
        }
    }
    return &input[0..0];
}

fn is_palindrome(s: &str) -> bool {
    let rs = s.chars().rev().collect::<String>();
    return rs == s;
}

// Expand Around Center
// Inspired from https://leetcode.com/articles/longest-palindromic-substring/
// Approach #4 (Expand Around Center)
// Time complexity n^2

pub fn find_longest_palindrome(input: &str) -> &str {
    let mut start: usize = 0;
    let mut end: usize = 0;
    let s = input.char_indices().collect::<Vec<_>>();
    let count = input.chars().count();

    for i in 0..count {
        let (l1, r1) = expand_around_center(&s, i, i);
        let (l2, r2) = expand_around_center(&s, i, i + 1);
        let len = cmp::max(r1 - l1, r2 - l2);
        if len > end - start {
            if r1 - l1 > r2 - l2 {
                start = l1;
                end = r1
            } else {
                start = l2;
                end = r2;
            }
        }
    }

    if end == count - 1 {
        return &input[(input.char_indices().nth(start).unwrap().0)..];
    } else {
        return &input[(input.char_indices().nth(start).unwrap().0)..input.char_indices()
            .nth(end + 1)
            .unwrap()
            .0];
    }
}

fn expand_around_center(ci: &Vec<(usize, char)>,
                        mut left: usize,
                        mut right: usize)
                        -> (usize, usize) {
    let mut changed = false;
    loop {
        if right == ci.len() {
            return (left, right - 1);
        }
        if left == right || ci[left].1 == ci[right].1 {
            if left == 0 || right == ci.len() - 1 {
                break;
            }
            left -= 1;
            right += 1;
            changed = true;
        } else {
            if changed {
                left += 1;
                right -= 1;
            }
            break;
        }
    }

    return (left, right);
}

// TODO
// http://articles.leetcode.com/longest-palindromic-substring-part-ii/

#[cfg(test)]
mod test {
    use super::find_longest_palindrome;
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

    #[test]
    fn test_find_longest_palindrome() {
        assert_eq!(find_longest_palindrome("babad"), "bab");
        assert_eq!(find_longest_palindrome("abccba"), "abccba");
        assert_eq!(find_longest_palindrome("cabccbad"), "abccba");
        assert_eq!(find_longest_palindrome("xbacabccbaddab"), "abccba");
        assert_eq!(find_longest_palindrome("abccbaddxe"), "abccba");
        assert_eq!(find_longest_palindrome("ddxeabccba"), "abccba");
        assert_eq!(find_longest_palindrome("a大大小小大大大"),
                   "大大小小大大");
    }

    use test::Bencher;
    // 35,575,528 ns/iter (+/- 3,888,585)
    #[bench]
    fn bench_longest_palindrome(b: &mut Bencher) {
        b.iter(|| {
            longest_palindrome("ddxeabccbssdfdfsafdsfasdfafdafljlk\
            jlkjasfdasdfasdfasdfddxeabccbaddxeabccbaddxeabccbaddxeabcc\
            baddxeabccbaddxeeabccbssdfdfsafdsfasdfafdafljlkjlkjasfdasdf\
            asdfasdfddxeabccbaddxeabccbaddabccbaddxeabccbaddxeabccbaddx\
            eabccbssdfdfsafdsfasdfafdafljlkjlkjasfdasdfasdfasdfddxeabcc\
            baddxeabccbaddxeabccbaddxeabccbaddxeabccbaddxeeabccbssdfdfs\
            afdsfasdfafdafljlkjlkjasfdasdfasdfasdfddxeabccbaddxeabccbad\
            dabccbaddxeabccbaddxeabccba");
        })
    }

    // 36,282,589 ns/iter (+/- 14,288,830)
    #[bench]
    fn bench_find_longest_palindrome(b: &mut Bencher) {
        b.iter(|| {
            longest_palindrome("ddxeabccbssdfdfsafdsfasdfafdafljlk\
            jlkjasfdasdfasdfasdfddxeabccbaddxeabccbaddxeabccbaddxeabcc\
            baddxeabccbaddxeeabccbssdfdfsafdsfasdfafdafljlkjlkjasfdasdf\
            asdfasdfddxeabccbaddxeabccbaddabccbaddxeabccbaddxeabccbaddx\
            eabccbssdfdfsafdsfasdfafdafljlkjlkjasfdasdfasdfasdfddxeabcc\
            baddxeabccbaddxeabccbaddxeabccbaddxeabccbaddxeeabccbssdfdfs\
            afdsfasdfafdafljlkjlkjasfdasdfasdfasdfddxeabccbaddxeabccbad\
            dabccbaddxeabccbaddxeabccba");
        })
    }
}
