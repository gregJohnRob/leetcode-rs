pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut nums3: Vec<i32> = Vec::new();
    let mut i1: usize = 0;
    let mut i2: usize = 0;
    while i1 < m as usize {
        while i2 < n as usize && nums1[i1] >= nums2[i2] {
            nums3.push(nums2[i2]);
            i2 = i2 + 1;
        }
        nums3.push(nums1[i1]);
        i1 = i1 + 1;
    }
    while i2 < n as usize {
        nums3.push(nums2[i2]);
        i2 = i2 + 1;
    }
    for i in 0..nums3.len() {
        nums1[i] = nums3[i];
    }
}

#[cfg(test)]
mod tests {
    use crate::merge_sorted_array::merge;

    #[test]
    fn case_1 () {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec!(1, 2, 2, 3, 5, 6));
    }

    #[test]
    fn case_2 () {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec!(1));
    }

    #[test]
    fn case_3 () {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec!(1));
    }
}
