pub fn add_two_numbers(l1: &[usize], l2: &[usize]) -> Vec<usize> {
    let length: usize = match l1.len() >= l2.len() {
        true => l1.len(),
        false => l2.len(),
    };
    let mut vec: Vec<usize> = Vec::with_capacity(length + 1);
    let mut carry: usize = 0;
    for i in 0..length {
        let a = *l1.get(i).unwrap_or(&0);
        let b = *l2.get(i).unwrap_or(&0);
        let mut c = a + b + carry;
        if c > 9 {
            carry = 1;
            c = c - 10;
        } else {
            carry = 0;
        }
        vec.push(c);
    }
    if carry == 1 {
        vec.push(carry);
    }
    return vec;
}

#[cfg(test)]
mod test {
    use super::add_two_numbers;

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(add_two_numbers(&[2, 4, 3], &[5, 6, 4]), vec![7, 0, 8]);

        assert_eq!(add_two_numbers(&[2, 2], &[5, 6, 4]), vec![7, 8, 4]);

        assert_eq!(add_two_numbers(&[2, 2, 1], &[5, 6, 4]), vec![7, 8, 5]);

        assert_eq!(add_two_numbers(&[2, 2, 1, 3], &[5, 6, 4]), vec![7, 8, 5, 3]);

        assert_eq!(add_two_numbers(&[9], &[9]), vec![8, 1]);
    }
}
