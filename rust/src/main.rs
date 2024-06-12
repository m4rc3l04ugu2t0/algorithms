mod first_uniq_char;
use first_uniq_char::first_uniq_char;

// use binary_search::binary_search;

mod binary_search;
fn main() {
    let result: i32;

    // result = binary_search(3);
    result = first_uniq_char("hellohe");
    println!("{}", result);
}
