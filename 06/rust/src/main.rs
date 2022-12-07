use std::env;
use std::fs;
use std::time::Instant;
fn test_chars(char_array: &Vec<char>, i_pos: usize, number: usize) -> bool {
    if number == 0 {
        true
    } else {
        if i_pos + number >= char_array.len() {
            false
        } else {
            for i_char in i_pos + 1..i_pos + number {
                if char_array[i_pos] == char_array[i_char] {
                    return false;
                }
            }
            test_chars(&char_array, i_pos + 1, number - 1)
        }
    }
}

//Usage ./rust PATH_TO_INPUT_FILE
fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("===========================");
    println!("  Advent Of Code 2022");
    println!("  Day 06");
    println!("  Rust version");
    println!("===========================");
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    //Read the input file
    let char_array: Vec<char> = contents.chars().collect();
    let mut first_char_a = 0;
    let size_a = 4;
    for i_char in 0..char_array.len() - size_a - 1 {
        if test_chars(&char_array, i_char, size_a) {
            first_char_a = i_char + size_a;
            break;
        }
    }
    let mut first_char_b = 0;
    let size_b = 14;
    for i_char in 0..char_array.len() - size_b - 1 {
        if test_chars(&char_array, i_char, size_b) {
            first_char_b = i_char + size_b;
            break;
        }
    }

    let duration = start.elapsed();
    println!("Time elapsed in total is: {:?}", duration);
    println!("Problem A : first character : {:?}", first_char_a);
    println!("Problem B : first character : {:?}", first_char_b);
}
