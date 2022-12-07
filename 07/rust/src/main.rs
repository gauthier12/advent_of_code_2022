use std::fmt; // Import `fmt`
#[macro_use]
extern crate scan_fmt;
use std::env;
use std::fs;
use std::time::Instant;

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
    //Read the input file
    let mut line_iter = contents.lines();
    line_iter.next();
    let mut folder_pile: Vec<String> = Vec::new();
    folder_pile.push("".to_string());
    for line in line_iter {
        if let Ok((file_size, file_name)) = scan_fmt!(
            line,    // input string
            "{} {}", // format
            u32, String
        ) {
            file_list.push(FSelement {
                name: file_name,
                path: folder_pile.join("/"),
                size: file_size,
            });
        } else {
            if line.starts_with("$ cd ") {
                if let Ok(dir_name) = scan_fmt!(
                    line,       // input string
                    "$ cd  {}", // format
                    String
                ) {
                    if dir_name == ".." {
                        folder_pile.pop();
                    } else {
                        folder_list.push(FSelement {
                            name: dir_name.clone(),
                            path: folder_pile.join("/"),
                            size: 0,
                        });
                        folder_pile.push(dir_name);
                    }
                }
            }
        }
    }

    let mut size_a = 0;
    for folder in folder_list.iter_mut() {
        let path = format!("{}/{}", folder.path, folder.name);
        folder.size = file_list.iter().fold(0, |s, fse| {
            if fse.path.starts_with(&path) {
                s + fse.size
            } else {
                s
            }
        });
        if folder.size < 100000 {
            size_a += folder.size;
        }
    }

    let total_space = 70000000;
    let used_space: u32 = file_list.iter().fold(0, |s, v| s + v.size);
    let free_space = total_space - used_space;
    let update_space = 30000000;
    let needed_space = update_space - free_space;
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
    println!("Problem A : size of the folder <100000 : {}", size_a);
    println!("Problem B : size of the directory to delete : {}", size_b);
}
