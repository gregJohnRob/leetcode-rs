pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[k] = nums[i];
            k = k + 1;
        }
    }
    k as i32
}

#[cfg(test)]
mod tests {
    use crate::remove_element::remove_element;

    #[test]
    fn case_1() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let result = remove_element(&mut nums, val);
        assert_eq!(nums[0..2], vec!(2, 2));
        assert_eq!(result, 2);
    }

    #[test]
    fn case_2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        let result = remove_element(&mut nums, val);
        assert_eq!(nums[0..5], vec!(0, 1, 3, 0, 4));
        assert_eq!(result, 5);
    }
}
