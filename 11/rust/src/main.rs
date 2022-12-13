#[macro_use]
extern crate scan_fmt;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;
#[derive(Debug,Clone)]
enum Operator {
    Nothing,
    Add,
    Multiply,
}
#[derive(Debug,Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operator: Operator,
    operator_value: u64,
    test_value: u64,
    true_monkey: usize,
    false_monkey: usize,
    activity:u64,
}
//Usage ./rust PATH_TO_INPUT_FILE
fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("===========================");
    println!("  Advent Of Code 2022");
    println!("  Day 11");
    println!("  Rust version");
    println!("===========================");
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut line_iter = contents.lines();
    let mut monkey_party: Vec<Monkey> = Vec::new();
    while let Some(line) = line_iter.next() {
        if let Ok(_monkey_num) = scan_fmt!(
            line,         // input string
            "Monkey {}:", // format
            u64
        ) {
            if let Some(starting_line) = line_iter.next() {
                if let Ok(starting_string) = scan_fmt!(
                    starting_line,                     // input string
                    "   Starting items: {/[0-9 ,]+/}", // format
                    String
                ) {
                    let items: VecDeque<u64> = starting_string
                        .split(", ")
                        .map(|v| v.parse::<u64>().unwrap())
                        .collect();
                    if let Some(operation_line) = line_iter.next() {
                        if let Ok((char_operator, number)) = scan_fmt!(
                            operation_line,                  // input string
                            "   Operation: new = old {} {}", // format
                            char,
                            String
                        ) {
                            let operator;
                            match char_operator {
                                '+' => {
                                    operator = Operator::Add;
                                }
                                '*' => {
                                    operator = Operator::Multiply;
                                }
                                _ => {
                                    operator = Operator::Nothing;
                                }
                            }
                            let operator_value: u64 = number.parse().unwrap_or(0);
                            if let Some(test_line) = line_iter.next() {
                                if let Ok(test_value) = scan_fmt!(
                                    test_line,                  // input string
                                    "   Test: divisible by {}", // format
                                    u64
                                ) {
                                    if let Some(true_line) = line_iter.next() {
                                        if let Ok(true_monkey) = scan_fmt!(
                                            true_line,                          // input string
                                            "     If true: throw to monkey {}", // format
                                            usize
                                        ) {
                                            if let Some(false_line) = line_iter.next() {
                                                if let Ok(false_monkey) = scan_fmt!(
                                                    false_line,                          // input string
                                                    "     If false: throw to monkey {}", // format
                                                    usize
                                                ) {
                                                    monkey_party.push(Monkey {
                                                        items,
                                                        operator,
                                                        operator_value,
                                                        test_value,
                                                        true_monkey,
                                                        false_monkey,
                                                        activity:0
                                                    });
                                                } else {
                                                    panic!("Expecting false monkey");
                                                }
                                            } else {
                                                panic!("File end before the false monkey");
                                            }
                                        } else {
                                            panic!("Expectiing true monkey");
                                        }
                                    } else {
                                        panic!("File end before the true monkey");
                                    }
                                } else {
                                    panic!("Expecting the test number");
                                }
                            } else {
                                panic!("File end before the test");
                            }
                        } else {
                            panic!("Expecting the operation");
                        }
                    } else {
                        panic!("File end before the operation");
                    }
                } else {
                    panic!("Expecting the starting items");
                }
            } else {
                panic!("File end before the items");
            }
        }
    }
    //Problem A
    let mut monkey_party_a = monkey_party.clone();
    for _iround in 1..=20
    {
        for i_monkey in 0..monkey_party_a.len()
        {
            while let Some(item) = monkey_party_a[i_monkey].items.pop_front() {
                monkey_party_a[i_monkey].activity+=1;
                let new_level:f64;
                //inspect the item 
                //change the worry level
                match monkey_party_a[i_monkey].operator
                {
                    Operator::Add => {new_level = (item +monkey_party_a[i_monkey].operator_value) as f64;},
                    Operator::Multiply => {new_level = if monkey_party_a[i_monkey].operator_value==0 {(item*item) as f64} else {(item * monkey_party_a[i_monkey].operator_value) as f64}; },
                    Operator::Nothing=>{new_level = item as f64},
                }
                //get bored
                let int_new_level = (new_level/3.0).floor() as u64;
                //test and throw
                let j_monkey = if int_new_level%monkey_party_a[i_monkey].test_value ==0
                {
                    monkey_party_a[i_monkey].true_monkey
                }
                else {
                    monkey_party_a[i_monkey].false_monkey
                }; 
                monkey_party_a[j_monkey].items.push_back(int_new_level);
            }
        }
    }
    let mut activities_a:Vec<u64> = monkey_party_a.iter().map(|m| m.activity).collect();
    activities_a.sort();
    let monkey_business_a = activities_a[activities_a.len()-1]*activities_a[activities_a.len()-2];
    //Problem B
    let common_factor = monkey_party.iter().fold(1,|sum,val| {sum*val.test_value});
    let mut monkey_party_b = monkey_party.clone();
    for _iround in 1..=10000
    {
        for i_monkey in 0..monkey_party_b.len()
        {
            while let Some(item) = monkey_party_b[i_monkey].items.pop_front() {
                monkey_party_b[i_monkey].activity+=1;
                let new_level:f64;
                //inspect the item 
                //change the worry level
                match monkey_party_b[i_monkey].operator
                {
                    Operator::Add => {new_level = (item +monkey_party_b[i_monkey].operator_value) as f64;},
                    Operator::Multiply => {new_level = if monkey_party_b[i_monkey].operator_value==0 {(item*item) as f64} else {(item * monkey_party_b[i_monkey].operator_value) as f64}; },
                    Operator::Nothing=>{new_level = item as f64},
                }
                //get bored
                let int_new_level = (new_level.floor() as u64)%common_factor;
                //test and throw
                let j_monkey = if int_new_level%monkey_party_b[i_monkey].test_value ==0
                {
                    monkey_party_b[i_monkey].true_monkey
                }
                else {
                    monkey_party_b[i_monkey].false_monkey
                }; 
                monkey_party_b[j_monkey].items.push_back(int_new_level);
            }
        }
    }
    let mut activities_b:Vec<u64> = monkey_party_b.iter().map(|m| m.activity).collect();
    activities_b.sort();
    let monkey_business_b = activities_b[activities_b.len()-1]*activities_b[activities_b.len()-2];

    let duration = start.elapsed();
    println!("Problem A : monkey business : {}", monkey_business_a );
    println!("Problem B : monkey business : {}", monkey_business_b);
    println!("Time elapsed in total is: {:?}", duration);
}
