// What's zigzag pattern?
// https://discuss.leetcode.com/topic/22925/if-you-are-confused-with-zigzag-pattern-come-and-see
pub fn zigzag_conversion(string: &str, num: usize) -> String {
    if num <= 1 { return string.to_string() };
    let mut res: Vec<Vec<char>> = Vec::with_capacity(num);
    for _ in 0..num {
        res.push(Vec::with_capacity(string.chars().count() / num + 1))
    }
    let chars = string.chars().into_iter();
    let mut index: usize = 0;
    let mut increase: bool = true;
    for c in chars {
        res[index].push(c);
        if increase {
            index += 1;
        } else {
            index -= 1;
        }
        if index % (num - 1) == 0 { increase = !increase; }
    }

    return res.into_iter().map(|i| i.into_iter().collect::<String>() ).collect::<String>();
}

#[cfg(test)]
mod test {
    use super::zigzag_conversion;

    #[test]
    fn test_zigzag_conversion() {
        assert_eq!(zigzag_conversion("PAYPALISHIRING", 3), "PAHNAPLSIIGYIR");

        assert_eq!(zigzag_conversion("PAYPALISHIRING", 1), "PAYPALISHIRING");

        assert_eq!(zigzag_conversion("我叫金三顺", 3), "我顺叫三金");
    }
}
