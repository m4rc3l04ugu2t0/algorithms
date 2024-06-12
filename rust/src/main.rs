use two_sum::two_sum;

mod first_uniq_char;
mod two_sum;

// use binary_search::binary_search;

mod binary_search;
fn main() {
    let result;
    let nums = vec![2, 3, 4];

    // result = binary_search(3);
    // result = first_uniq_char("hellohe");
    result = two_sum(&nums, 6);
    println!("{:?}", result);
}
