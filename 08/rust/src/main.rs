use std::fmt; // Import `fmt`
#[macro_use]
extern crate scan_fmt;
extern crate ndarray;
use ndarray::Array2;
use ndarray::Array;
use ndarray::Axis;
use rayon::prelude::*;
use std::env;
use std::fs;
use std::time::Instant;

// For input 
const COLUMN_NUM: usize = 31;

// For test 
//const COLUMN_NUM: usize = 11;
/*
#[derive(Debug)]
struct FSelement {
    name: String,
    path: String,
    size: u32,
}
impl fmt::Display for FSelement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{:08} {}/{}", self.size, self.path, self.name)
    }
}*/

//Usage ./rust PATH_TO_I    NPUT_FILE
fn main() {
    
    let mut map = Array2::<u8>::zeros((0, COLUMN_NUM));
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("===========================");
    println!("  Advent Of Code 2022");
    println!("  Day 08");
    println!("  Rust version");
    println!("===========================");
    let filename = &args[1];
    println!("In file {}", filename);
    println!("current_num_threads() = {}", rayon::current_num_threads());
    match rayon::current_thread_index() {
        // The division was valid
        Some(nt) => println!("current_thread_index() = {}", nt),
        None => println!("No thread index"),
    }
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for line in contents.lines() {
        let test1 = Array::from_iter(line.chars().map(|c| if c=='#' {1} else {0} ));
        match map.push_row(test1.view())
        {
            Err(_) => panic!("Conversion KO"),
            _ => (),
        }
    }
    let speed_list:[(usize,usize);5] = [(1,1),(1,3),(1,5),(1,7),(2,1)];
    let mult_total:u64 = speed_list.par_iter().map(
        |(vs,hs)| 
        {
            (0..map.len_of(Axis(0))).step_by(*vs).map(
                | ir |
                {
                    map[[ir,(ir/vs*hs)%COLUMN_NUM]] as u32
                }
            ).sum::<u32>() as u64
        }
    ).product();

    let duration = start.elapsed();
    println!("Time elapsed in total is: {:?}", duration);
    println!("Problem A : size of the folders <100000 : {} ", mult_total);
    println!("Problem B : size of the folder to delete : {}", mult_total);
}
