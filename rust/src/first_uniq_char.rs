use std::collections::HashMap;

pub fn _first_uniq_char(s: &str) -> i32 {
    let mut char_count: HashMap<char, u32> = HashMap::new();

    for c in s.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    println!("{:?}", char_count);

    for (i, c) in s.chars().enumerate() {
        if char_count[&c] == 1 {
            return i as i32;
        }
    }
    -1
}
