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
    let mut score_A: u32 = 0;
    let mut score_B: u32 = 0;
    for line in contents.lines() {
        if let Ok((oponent, me)) = scan_fmt!(
            line,    // input string
            "{} {}", // format
            char, char
        ) {
            match me {
                'X' => {
                    //pb A : I played Rock
                    score_A += 1;
                    //pb B : I need to lose
                    score_B += 0;
                    match oponent {
                        'A' => {
                            //Oponent played Rock
                            //pb A : Draw
                            score_A += 3;
                            //pb B : I need to play Scissors
                            score_B += 3;
                        }
                        'B' => {
                            //Oponent played Paper
                            //pb A : I lose
                            score_A += 0;
                            //pb B : I need to play Rock
                            score_B += 1;
                        }
                        'C' => {
                            //Oponent played Scissors
                            //pb A : I win
                            score_A += 6;
                            //pb B : I need to play Paper
                            score_B += 2;
                        }
                        _ => (),
                    }
                }
                'Y' => {
                    //pb A : I played Paper
                    score_A += 2;
                    //pb B : I need to draw
                    score_B += 3;
                    match oponent {
                        'A' => {
                            //Oponent played Rock
                            //pb A : I win
                            score_A += 6;
                            //pb B : I need to play Rock
                            score_B += 1;
                        }
                        'B' => {
                            //Oponent played Paper
                            //pb A : Draw
                            score_A += 3;
                            //pb B : I need to play Paper
                            score_B += 2;
                        }
                        'C' => {
                            //Oponent played Scissors
                            //pb A : I lose
                            score_A += 0;
                            //pb B : I need to play Scissors
                            score_B += 3;
                        }
                        _ => (),
                    }
                }
                'Z' => {
                    //pb A : I played Scissors
                    score_A += 3;
                    //pb B : I need to win
                    score_B += 6;
                    match oponent {
                        'A' => {
                            //Oponent played Rock : I lose
                            score_A += 0;
                            //pb B : I need to play Paper
                            score_B += 2;
                        }
                        'B' => {
                            //Oponent played Paper : I win
                            score_A += 6;
                            //pb B : I need to play Scissors
                            score_B += 3;
                        }
                        'C' => {
                            //Oponent played Scissors : Draw
                            score_A += 3;
                            //pb B : I need to play Rock
                            score_B += 1;
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        } else {
            panic!("Error in the read line");
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed in total is: {:?}", duration);
    println!("Problem A : final score : {:?}", score_A);
    println!("Problem B : final score : {:?}", score_B);
}
