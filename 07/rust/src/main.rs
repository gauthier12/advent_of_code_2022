#[macro_use]
extern crate scan_fmt;
use std::env;
use std::fs;
use std::rc::Weak;
use std::time::Instant;
#[derive(Debug)]
enum FileType {
    Folder,
    File,
}

#[derive(Debug)]
struct FileTreeElement {
    name: String,
    size: usize,
    typ: FileType,
    children: Vec<FileTreeElement>,
    parent: Weak<FileTreeElement>,
}
//Usage ./rust PATH_TO_I    NPUT_FILE
fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("===========================");
    println!("  Advent Of Code 2022");
    println!("  Day 07");
    println!("  Rust version");
    println!("===========================");
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut device: FileTreeElement = FileTreeElement {
        name: '/'.to_string(),
        size: 0,
        typ: FileType::Folder,
        children: Vec::new(),
        parent: Weak::new(),
    };
    let mut cur_pos = &device;
    //Read the input file
    let mut line_iter = contents.lines();
    line_iter.next();
    for line in line_iter {
        let first_char = line.chars().next().unwrap();
        match first_char {
            '$' => {
                if line.starts_with("$ cd ") {
                } else {
                    if line.starts_with("$ ls ") {
                    } else {
                    }
                }
                //command
                println!("command          {}", line);
            }
            'd' => {
                //command
                if let Ok(dir_name) = scan_fmt!(
                    line,     // input string
                    "dir {}", // format
                    String
                ) {
                    println!(" dir         |{}|", dir_name);

                } else {
                    panic!("Error in the read line");
                }
            }
            '0'..='9' => {
                //command
                println!("listing of file  {}", line);
            }
            _ => {
                //other choice
                println!("other            {}", line);
            }
        }
        /*
        if line.starts_with("$") {
            //This is a linux command
            let char_array: Vec<char> = line.chars().collect();
            let pile_number = (char_array.len() + 1) / 4; //+ 1 because it is missing the last space
            crate_piles.resize(pile_number, Vec::new());
            for i_crate in 0..pile_number {
                //Crate columns works by 4 chars "[x] ", the name of the crate is the 2nd char
                crate_piles[i_crate].push(char_array[i_crate * 4 + 1]);
            }
        }
        if line.starts_with("move") {
            //this is a move instruction

        }*/
    }
    println!("{:?}", device);
    let duration = start.elapsed();
    println!("Time elapsed in total is: {:?}", duration);
    //println!("Problem A : message : {:?}", message_a);
    //println!("Problem B : message : {:?}", message_b);
}
