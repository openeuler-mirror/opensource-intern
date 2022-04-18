pub struct Search {}

impl Search {
    pub fn search(nums: Vec<String>, target: String) -> usize {
        let mut left: usize = 0;
        let mut right: usize = nums.len();
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else if nums[mid] > target {
                right = mid;
            } else {
                return mid as usize;
            }
        }
        0
    }
}
