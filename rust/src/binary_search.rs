const NUMS: [i32; 7] = [-5, -3, 0, 1, 3, 8, 12];

pub fn binary_search(target: i32) -> i32 {
    let mut low = 0;
    let mut high = NUMS.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;

        if NUMS[mid] == target {
            return mid as i32;
        } else if NUMS[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    -1
}
