use is_valid::is_valid;

mod first_uniq_char;
mod is_valid;
mod move_zeroes;
mod two_sum;

// use binary_search::binary_search;

mod binary_search;
fn main() {
    let s1 = String::from("()");
    let s2 = String::from("()[]{}");
    let s3 = String::from("([])");

    println!("{}", is_valid(s1)); // Saída: true
    println!("{}", is_valid(s2)); // Saída: true
    println!("{}", is_valid(s3)); // Saída: false
                                  // let mut nums = vec![0, 1, 0, 3, 0, 0, 0, 12];

    // result = binary_search(3);
    // result = first_uniq_char("hellohe");
    // result = two_sum(&nums, 6);
    // move_zeroes(&mut nums);
}
