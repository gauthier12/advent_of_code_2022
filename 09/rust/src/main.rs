use std::fmt;
#[macro_use]
extern crate scan_fmt;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;
struct Ins {
    dir: char,
    dis: i32,
}
impl fmt::Display for Ins {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "move {} for {:3} ", self.dir, self.dis)
    }
}
fn move_tail(hp: (i32, i32), tp: (i32, i32)) -> (i32, i32) {
    let mut new_pos = tp;
    if i32::abs(hp.0 - tp.0) > 1 || i32::abs(hp.1 - tp.1) > 1 {
        if hp.1 > tp.1 {
            new_pos.1 += 1;
        }
        if hp.1 < tp.1 {
            new_pos.1 -= 1;
        }
        if hp.0 > tp.0 {
            new_pos.0 += 1;
        }
        if hp.0 < tp.0 {
            new_pos.0 -= 1;
        }
    }
    return new_pos;
}
fn display(rope: &Vec<(i32, i32)>) -> () {
    let min_line = std::cmp::min(rope.iter().map(|p| p.0).min().unwrap()-2,-2);
    let  max_line = std::cmp::max(rope.iter().map(|p| p.0).max().unwrap()+2,2);
    let min_col = std::cmp::min(rope.iter().map(|p| p.1).min().unwrap()-2,-2);
    let  max_col = std::cmp::max(rope.iter().map(|p| p.1).max().unwrap()+2,2);
    let mut charmat: Vec<Vec<char>> =
        vec![vec!['.'; (max_col - min_col) as usize]; (max_line - min_line) as usize];

    charmat[(0 - min_line) as usize][(0 - min_col) as usize]='s';
    for (num, knot) in rope.iter().enumerate().rev() {
        charmat[(knot.0 - min_line) as usize][(knot.1 - min_col) as usize] =
            char::from_digit(num as u32, 10).unwrap();
    }
    for li in charmat.iter().rev() {
        let li2: String = li.iter().collect();
        println!("{}", li2);
    }
}
//Usage ./rust PATH_TO_INPUT_FILE
fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("===========================");
    println!("  Advent Of Code 2022");
    println!("  Day 09");
    println!("  Rust version");
    println!("===========================");
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut instructions: Vec<Ins> = Vec::new();
    for line in contents.lines() {
        if let Ok((dir, dis)) = scan_fmt!(
            line,    // input string
            "{} {}", // format
            char, i32
        ) {
            instructions.push(Ins { dir, dis });
        } else {
            panic!("Error in the read line");
        }
    }
    //short rope
    let mut short_rope = vec![(0, 0); 2];
    let mut short_tail_positions = HashSet::new();
    //println!("Instructions");
    for instr in instructions.iter() {
        //println!("{}", instr);
        for _i_step in 0..instr.dis {
            //println!(" ");
            match instr.dir {
                'R' => short_rope[0].1 += 1,
                'L' => short_rope[0].1 -= 1,
                'U' => short_rope[0].0 += 1,
                'D' => short_rope[0].0 -= 1,
                _ => break,
            }
            for i_knot in 1..short_rope.len() {
                short_rope[i_knot] = move_tail(short_rope[i_knot - 1], short_rope[i_knot]);
            }
            short_tail_positions.insert(short_rope[short_rope.len() - 1]);
        }
    }
    //long rope
    let mut long_rope = vec![(0, 0); 10];
    let mut long_tail_positions = HashSet::new();
    //println!("Instructions");
    for instr in instructions.iter() {
        //println!("{}", instr);
        for _i_step in 0..instr.dis {
            match instr.dir {
                'R' => long_rope[0].1 += 1,
                'L' => long_rope[0].1 -= 1,
                'U' => long_rope[0].0 += 1,
                'D' => long_rope[0].0 -= 1,
                _ => break,
            }
            for i_knot in 1..long_rope.len() {
                long_rope[i_knot] = move_tail(long_rope[i_knot - 1], long_rope[i_knot]);
            }
            long_tail_positions.insert(long_rope[long_rope.len() - 1]);
            //display(&long_rope);
            //println!(" ");
        }
    }

    let duration = start.elapsed();
    println!("Time elapsed in total is: {:?}", duration);
    println!(
        "Problem A : number of position visited by the tail : {:?}",
        short_tail_positions.len()
    );
    println!(
        "Problem B : number of position visited by the tail : {:?}",
        long_tail_positions.len()
    );
}
