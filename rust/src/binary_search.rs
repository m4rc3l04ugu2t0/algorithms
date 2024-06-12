const _NUMS: [i32; 7] = [-5, -3, 0, 1, 3, 8, 12];

pub fn _binary_search(target: i32) -> i32 {
    let mut low = 0;
    let mut high = _NUMS.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;

        if _NUMS[mid] == target {
            return mid as i32;
        } else if _NUMS[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    -1
}
