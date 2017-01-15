pub fn two_sum(nums: &[usize], target: usize) -> Option<[usize;2]> {
    let len = nums.len();
    if len < 2 { return None };
    let mut i = 0;
    let mut j = 1;
    let result: Option<[usize;2]>;
    loop {
        if nums[i] + nums[j] == target {
            result = Some([i, j]);
            break;
        }
        j = j + 1;
        if j == len {
            i = i + 1;
            j = i + 1;
        }
        if i == len - 1 {
            result = None;
            break;
        }
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::two_sum;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(&[2, 7, 11, 15], 18), Some([1,2]));

        assert_eq!(two_sum(&[3,2,4], 6), Some([1,2]));

        assert_eq!(two_sum(&[3,2,4], 7), Some([0,2]));

        assert_eq!(two_sum(&[3,2,4], 8), None);

        assert_eq!(two_sum(&[3,2,11, 3], 5), Some([0,1]));

        assert_eq!(two_sum(&[3], 5), None);
    }
}
