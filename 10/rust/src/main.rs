#[macro_use]
extern crate scan_fmt;
use std::env;
use std::fs;
use std::time::Instant;
fn light_pixel(crt_row: &mut [char; 40], current_register: i32, current_cycle: i32) -> () {
    let pixel_pos = current_cycle % 40;
    //println!("Pixel pos {}",pixel_pos);
    if i32::abs(pixel_pos - current_register) <= 1 {
        crt_row[pixel_pos as usize] = '#';
    }
    if pixel_pos == 39 {
        let li2: String = crt_row.iter().collect();
        println!("{}", li2);
        *crt_row = ['.'; 40];
    }
}
//Usage ./rust PATH_TO_INPUT_FILE
fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("===========================");
    println!("  Advent Of Code 2022");
    println!("  Day 10");
    println!("  Rust version");
    println!("===========================");
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut crt_row: [char; 40] = ['.'; 40];
    let mut next_check_cycle = 20;
    let check_interval = 40;
    let mut current_cycle = 0;
    let mut current_register = 1;
    let mut sum_signal_strengh = 0;
    println!("Problem B : ");
    for line in contents.lines() {
        if line == "noop" {
            light_pixel(&mut crt_row, current_register, current_cycle);
            current_cycle += 1;
        } else {
            if let Ok(val) = scan_fmt!(
                line,      // input string
                "addx {}", // format
                i32
            ) {
                light_pixel(&mut crt_row, current_register, current_cycle);
                current_cycle += 1;
                light_pixel(&mut crt_row, current_register, current_cycle);
                current_cycle += 1;
                if current_cycle >= next_check_cycle {
                    sum_signal_strengh += next_check_cycle * current_register;
                    next_check_cycle += check_interval;
                }
                current_register += val;
            } else {
                panic!("Error in the read line");
            }
        }
    }
    let duration = start.elapsed();
    println!(
        "Problem A : sum of signal strength : {:?}",
        sum_signal_strengh
    );
    println!("Time elapsed in total is: {:?}", duration);
}
