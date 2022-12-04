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
    let mut num_pair_a: u32 = 0;
    let mut num_pair_b: u32 = 0;
    for line in contents.lines() {
        if let Ok((min_1,max_1,min_2,max_2)) = scan_fmt!(
            line,    // input string
            "{}-{},{}-{}", // format
            u32, u32,u32,u32
        ) {
            //an interval is included if the min and the max is inside the other interval
            if(min_1>=min_2&&max_1<=max_2)||(min_2>=min_1&&max_2<=max_1)
            {
                num_pair_a+=1;
            }
            //if min AND max of an interval are under the min of the other, it doesn't overlap --> if not, it overlaps
            if!( ((min_1<min_2)&&(max_1<min_2))||((min_2<min_1)&&(max_2<min_1)))
            {
                num_pair_b+=1;
            }
        } else {
            panic!("Error in the read line");
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed in total is: {:?}", duration);
    println!("Problem A : number of included pair : {:?}", num_pair_a);
    println!("Problem B : number of overlapping pair : {:?}", num_pair_b);
}
