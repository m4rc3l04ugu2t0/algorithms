pub fn two_sum(nums: &[i32], target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let sum = nums[left] + nums[right];

        if sum == target {
            return vec![(left + 1) as i32, (right + 1) as i32];
        }

        if sum > target {
            right -= 1;
        } else {
            left += 1;
        }
    }

    vec![-1]
}
