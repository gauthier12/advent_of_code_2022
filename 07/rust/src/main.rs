use std::fmt; // Import `fmt`
#[macro_use]
extern crate scan_fmt;
use std::env;
use std::fs;
use std::time::Instant;
use rayon::prelude::*;


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
    let mut file_list: Vec<FSelement> = Vec::new();
    let mut folder_list: Vec<FSelement> = Vec::new();
    //We keep a folder pile to construct the full path of the files and folders and we start it with the root folder ""
    let mut folder_pile: Vec<String> = vec!["".to_string()];
    //Read the input file
    for line in contents.lines().skip(1) {
        // If it starts with a number, this a file : push it to the files list
        if let Ok((file_size, file_name)) = scan_fmt!(line, "{} {}", u32, String) {
            file_list.push(FSelement {
                name: file_name,
                path: folder_pile.join("/"),
                size: file_size,
            });
        } else {
            //if it is not a file, maybe we are changing the current folder
            if line.starts_with("$ cd ") {
                //This is a cd command
                if let Ok(dir_name) = scan_fmt!(line, "$ cd  {}", String) {
                    //Going one folder up
                    if dir_name == ".." {
                        folder_pile.pop();
                    } else {
                        //Add the new folder to the list
                        folder_list.push(FSelement {
                            name: dir_name.clone(),
                            path: folder_pile.join("/"),
                            size: 0,
                        });
                        //update the current folder path
                        folder_pile.push(dir_name);
                    }
                }
            }
            //In fact, we don't care about the other lines
        }
    }
    //We compute the folder size by summing all the file with the folder path in their path
    folder_list.par_iter_mut().for_each(|f| {
        let path = format!("{}/{}", f.path, f.name);
        f.size=file_list.iter().fold(0, |s, fse| {
        if fse.path.starts_with(&path) {
            s + fse.size
        } else {
            s
        }})}
    );
    //we compute the sum of the folder < 100000
    let size_a:u32 = folder_list.iter().map(|f| if f.size<100000 {f.size}else{0}).sum();
    //How much space do we need to free ?
    let total_space = 70000000;
    let used_space: u32 = file_list.iter().fold(0, |s, v| s + v.size);
    let free_space = total_space - used_space;
    let update_space = 30000000;
    let needed_space = update_space - free_space;
    //We sort the folder by size and search the first/smallest freeing enought space
    folder_list.sort_by_key(|f| f.size);
    let mut size_b = 0;
    for folder in folder_list.iter() {
        if folder.size >= needed_space {
            size_b = folder.size;
            break;
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed in total is: {:?}", duration);
    println!("Problem A : size of the folders <100000 : {} ", size_a);
    println!("Problem B : size of the folder to delete : {}", size_b);
}
