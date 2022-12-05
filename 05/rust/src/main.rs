#[macro_use]
extern crate scan_fmt;
use std::env;
use std::fs;
use std::time::Instant;
#[derive(Debug)]
struct Instruction {
    number: u32,
    from: usize,
    to: usize,
}
//Usage ./rust PATH_TO_INPUT_FILE
fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("===========================");
    println!("  Advent Of Code 2022");
    println!("  Day 05");
    println!("  Rust version");
    println!("===========================");
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut crate_piles: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    //Read the input file
    for line in contents.lines() {
        if line.contains("[") {
            //This is a description of the crate piles
            let char_array: Vec<char> = line.chars().collect();
            let pile_number = (char_array.len() + 1) / 4;
            crate_piles.resize(pile_number, Vec::new());
            for i_crate in 0..pile_number {
                //Crate columns works by 4 chars "[x] ", the name of the crate is the 2nd char
                crate_piles[i_crate].push(char_array[i_crate * 4 + 1]);
            }
        }
        if line.starts_with("move") {
            //this is a move instruction
            if let Ok((number, from, to)) = scan_fmt!(
                line,                    // input string
                "move {} from {} to {}", // format
                u32,
                usize,
                usize
            ) {
                instructions.push(Instruction {
                    number,
                    from: from - 1,
                    to: to - 1,
                });
            } else {
                panic!("Error in the read line");
            }
        }
    }
    //Put the pile in the right order for pushing/popping
    for pile in crate_piles.iter_mut() {
        pile.reverse();
        while *pile.last().unwrap() == ' ' {
            pile.pop();
        }
    }
    //Copy the initial state and solve problem A
    let mut crate_piles_a = crate_piles.clone();
    for inst in instructions.iter() {
        for _ in 0..inst.number {
            //move the crate one by one
            let cur_crate = crate_piles_a[inst.from]
                .pop()
                .expect("not enough crates in pile");
            crate_piles_a[inst.to].push(cur_crate);
        }
    }
    let mut message_a = String::new();
    for pile in crate_piles_a.iter() {
        message_a.push(*pile.last().expect("Empty crate pile"));
    }
    //Copy the initial state and solve problem B
    let mut crate_piles_b = crate_piles.clone();
    for inst in instructions.iter() {
        let mut loaded_crate_pile: Vec<char> = Vec::new();
        for _ in 0..inst.number {
            //move the crate on the loaded crate pile
            let cur_crate = crate_piles_b[inst.from]
                .pop()
                .expect("not enough crates in pile");
            loaded_crate_pile.push(cur_crate);
        }
        for _ in 0..inst.number {
            //unload the loaded crate pile
            let cur_crate = loaded_crate_pile.pop().expect("not enough crates in pile");
            crate_piles_b[inst.to].push(cur_crate);
        }
    }
    let mut message_b = String::new();
    for pile in crate_piles_b.iter() {
        message_b.push(*pile.last().expect("Empty crate pile"));
    }
    let duration = start.elapsed();
    println!("Time elapsed in total is: {:?}", duration);
    println!("Problem A : message : {:?}", message_a);
    println!("Problem B : message : {:?}", message_b);
}
