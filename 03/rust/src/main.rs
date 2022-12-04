use itertools::Itertools;
use std::env;
use std::fs;
use std::time::Instant;
fn priority(c: char) -> u32 {
    if c as u32 >= 97 {
        c as u32 - 97 + 1
    } else {
        c as u32 - 65 + 27
    }
}
//Usage ./rust PATH_TO_INPUT_FILE
fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("===========================");
    println!("  Advent Of Code 2022");
    println!("  Day 03");
    println!("  Rust version");
    println!("===========================");
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut sum_a: u32 = 0;
    //problem A
    for line in contents.lines() {
        let first_compartment: String = String::from(&line[0..line.len() / 2]);
        let second_compartment: String = String::from(&line[line.len() / 2..line.len()]);
        let common = first_compartment
            .chars()
            .filter(|c| second_compartment.contains(*c))
            .next()
            .expect("No common char found");
        sum_a += priority(common);
    }
    //problem B
    let mut sum_b: u32 = 0;
    let lines = contents.lines();
    for lines_chunck in &lines.chunks(3) {
        let mut li = lines_chunck.into_iter();
        let line1 = String::from(li.next().expect("number of lines not multiple of 3"));
        let line2 = String::from(li.next().expect("number of lines not multiple of 3"));
        let line3 = String::from(li.next().expect("number of lines not multiple of 3"));
        let common12: Vec<char> = line1.chars().filter(|c| line2.contains(*c)).collect();
        let common123: char = common12
            .into_iter()
            .filter(|&c| line3.contains(c))
            .next()
            .expect("No common char found");
            sum_b += priority(common123);
    }
    let duration = start.elapsed();
    println!("Time elapsed in total is: {:?}", duration);
    println!("Problem A : priority sum : {:?}", sum_a);
    println!("Problem B : priority sum : {:?}", sum_b);
}
