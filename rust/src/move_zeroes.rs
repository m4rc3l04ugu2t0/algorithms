pub fn _move_zeroes(nums: &mut Vec<i32>) {
    let mut last_non_zero_found_at = 0;

    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums[last_non_zero_found_at] = nums[i];
            last_non_zero_found_at += 1;
        }
    }

    for i in last_non_zero_found_at..nums.len() {
        nums[i] = 0;
    }
}
