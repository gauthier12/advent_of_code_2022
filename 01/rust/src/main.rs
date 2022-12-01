#[macro_use]
extern crate scan_fmt;
use std::env;
use std::fs;
use std::time::Instant;

//Usage ./rust PATH_TO_INPUT_FILE
fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut sum: Vec<u32> = Vec::new();
    let mut cur_sum = 0;
    for line in contents.lines() {
        if let Ok(cal) = scan_fmt!(
            line, // input string
            "{}", // format
            u32
        ) {
            //Number found : add it to the current elf sum
            cur_sum += cal;
        } else {
            //No number found : push the current elf to the list and start a new elf
            sum.push(cur_sum);
            //println!("New elf carrying : {}", cur_sum);
            cur_sum = 0;
        }
    }
    //Last elf : push the the last elf to the list
    //println!("Last elf carrying: {}", cur_sum);
    sum.push(cur_sum);
    sum.sort();
    let duration = start.elapsed();
    println!("Time elapsed in total is: {:?}", duration);
    println!(
        "Problem A : Elf carrying the most Calories             : {:?}",
        sum[sum.len() - 1]
    );
    println!(
        "Problem B : top three Elves carrying the most Calories : {:?}",
        sum[sum.len() - 1] + sum[sum.len() - 2] + sum[sum.len() - 3]
    );
}
