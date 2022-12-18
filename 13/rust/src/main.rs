use std::cmp::Ordering;
use core::str::Chars;
use itertools::Itertools;
use std::env;
use std::fmt;
use std::fs;
use std::iter::Peekable;
use std::time::Instant;
#[derive(Debug,PartialEq, Eq)]
enum ListE {
    Num(u32),
    List(Vec<ListE>),
}
impl Ord for ListE {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), Self::Num(b)) => a.cmp(&vec![Self::Num(*b)]),
            (Self::Num(a), Self::List(b)) => vec![Self::Num(*a)].cmp(&b),
            (Self::Num(a), Self::Num(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for ListE {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl fmt::Display for ListE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ListE::Num(val) => {
                write!(f, "{}", val)
            }
            ListE::List(list_vec) => {
                write!(f, "[");
                if list_vec.len() != 0 {
                    for elem in &list_vec[..list_vec.len() - 1] {
                        write!(f, "{},", elem);
                    }
                    write!(f, "{}", list_vec.last().unwrap());
                }
                    write!(f, "]");
                return Ok(());
            }
        }
    }
}

fn read_line(char_iter: &mut Peekable<Chars>, level: u32) -> ListE {
    let mut number_string: String = String::new();
    let mut res_list: Vec<ListE> = Vec::new();
    while let Some(char) = char_iter.next() {
        match char {
            '[' => {
                match char_iter.peek() {
                    Some(']') => {
                        //nothing
                        char_iter.next();
                        return ListE::List(res_list);
                    }
                    _ => {
                        res_list.push(read_line(char_iter, level + 1));
                    }
                }
            }
            '0'..='9' => {
                number_string.push(char);
                match char_iter.peek() {
                    Some('0') | Some('1') | Some('2') | Some('3') | Some('4') | Some('5')
                    | Some('6') | Some('7') | Some('8') | Some('9') => {
                        //nothing
                    }
                    _ => {
                        return ListE::Num(number_string.parse::<u32>().unwrap());
                    }
                }
            }
            ',' => {
                res_list.push(read_line(char_iter, level + 1));
            }
            ']' => {
                return ListE::List(res_list);
            }
            _ => {
            }
        }
    }
    ListE::List(res_list)
}
//Usage ./rust PATH_TO_INPUT_FILE
fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("===========================");
    println!("  Advent Of Code 2022");
    println!("  Day 13");
    println!("  Rust version");
    println!("===========================");
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut pairs: Vec<[ListE;2]> = Vec::new();
    for lines_chunck in contents.split("\n\n") {
        let mut li = lines_chunck.lines();
        let line1 = li.next().expect("Missing line");
        let line2 = li.next().expect("Missing line");
        let mut char_iter1 = line1.chars().peekable();
        let read1 = read_line(&mut char_iter1, 0);
        let mut char_iter2 = line2.chars().peekable();
        let read2 = read_line(&mut char_iter2, 0);
        pairs.push([read1,read2]);
    }
    let num_ordered_pair = pairs.iter().enumerate().fold(0,|sum,(idx,[valg,vald])| {sum+if valg<vald {idx+1}else {0}});
    let mut lists: Vec<_> = pairs.iter().flatten().collect();
    let line1 = "[[2]]";
    let line2 = "[[6]]";
    let mut char_iter1 = line1.chars().peekable();
    let read1 = read_line(&mut char_iter1, 0);
    let mut char_iter2 = line2.chars().peekable();
    let read2 = read_line(&mut char_iter2, 0);
    lists.push(&read1);
    lists.push(&read2);
    lists.sort();
    let decoder_key = lists.into_iter().positions(|val| {val==&read1 || val==&read2}).fold(1, |sum,val| sum*(val as i32 +1 ));

    let duration = start.elapsed();
    println!("Problem A : number of ordered pair : {} ", num_ordered_pair);
    println!("Problem B : Decoder key : {} ", decoder_key);
    println!("Time elapsed in total is: {:?}", duration);
}
