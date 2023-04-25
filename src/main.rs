use std::io;

use suffix_array_build::build_suffix_array;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input = input.trim().to_string();
    let suffix_array: Vec<usize> = build_suffix_array(input.clone());

    println!("Original string: {}, len = {}", input, input.len()-1);
    println!("{:#?}", suffix_array);
}   
