// The goal is to count the frequency of consecutives strings in a row. 

use std::collections::HashMap;

fn count_consecutive_string_number(value: &str) -> HashMap<char, u32> {
    let mut temp: HashMap<char, u32> = HashMap::new();
    let mut target = value.chars();
    target.next();
    for current_char in value.chars() {
        match target.next() {
            Some(next_char) if current_char == next_char => {
               match temp.get(&current_char) {
                Some(number) if number > &1 => temp.insert(current_char,  number + 1),
                Some(number) if number < &1 => temp.insert(current_char,  number + 2),
                _ => temp.insert(current_char, 2)
               }
            },
            _ => 
            match temp.get(&current_char) {
                Some(_number) => None,
                _ => temp.insert(current_char, 0)
            }
        };
    }

    temp
}

fn main() {
    let result: HashMap<char, u32> = count_consecutive_string_number("nBalooonnn");
    println!("{:#?}", result);

    // {
    //     'B': 0,
    //     'a': 0,
    //     'l': 0,
    //     'o': 3,
    //     'n': 3,
    // }
}