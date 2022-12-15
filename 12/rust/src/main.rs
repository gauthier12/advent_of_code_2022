#[macro_use]
extern crate ndarray;
use ndarray::Array;
use ndarray::Array2;
use ndarray::Axis;
use std::env;
use std::fs;
use std::time::Instant;
//Usage ./rust PATH_TO_INPUT_FILE
fn main() {
    const RADIX: i32 = 10;
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("===========================");
    println!("  Advent Of Code 2022");
    println!("  Day 12");
    println!("  Rust version");
    println!("===========================");
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let n_column = contents.lines().next().unwrap().chars().count();
    let n_line = contents.lines().count();
    let mut map = Array2::<i32>::zeros((0, n_column));
    let mut distance_from_start= Array::from_elem((n_line, n_column), n_column*n_line);
    let mut distance_to_end= Array::from_elem((n_line, n_column), n_column*n_line);
    let mut start_position: (usize, usize) = (0, 0);
    let mut end_position: (usize, usize) = (0, 0);

    for (i, line) in contents.lines().enumerate() {
        let test1 = Array::from_iter(line.chars().enumerate().map(|(j, c)| {
            if (c as i32) >= 97 {
                c as i32 - 97
            } else {
                if c == 'S' {
                    start_position = (i, j);
                    0
                } else {
                    end_position = (i, j);
                    25
                }
            }
        }));
        match map.push_row(test1.view()) {
            Err(_) => panic!("Conversion KO"),
            _ => (),
        }
    }
    distance_from_start[start_position]=0;
    //for i_dis in 0..3 as usize
    for i_dis in 0..=n_line*n_column
    {
        for ((i,j),current_height) in map.indexed_iter()
        {
            if distance_from_start[(i, j)]==i_dis
            {
                if i >= 1 {
                    let p_next = (i - 1, j);
                    if map[p_next] <= current_height + 1 && distance_from_start[p_next]>distance_from_start[(i, j)]
                    {
                        distance_from_start[p_next] = i_dis+1;
                    }
                }
                if j >= 1 {
                    let p_next = (i, j - 1);
                    if map[p_next] <= current_height + 1 && distance_from_start[p_next]>distance_from_start[(i, j)]
                    {
                        distance_from_start[p_next] = i_dis+1;
                    }
                }
                if i < map.len_of(Axis(0)) - 1 {
                    let p_next = (i + 1, j);
                    if map[p_next] <= current_height + 1 && distance_from_start[p_next]>distance_from_start[(i, j)]
                    {
                        distance_from_start[p_next] = i_dis+1;
                    }
                }
                if j < map.len_of(Axis(1)) - 1 {
                    let p_next = (i, j+1);
                    if map[p_next] <= current_height + 1 && distance_from_start[p_next]>distance_from_start[(i, j)]
                    {
                        distance_from_start[p_next] = i_dis+1;
                    }
                }
            }
        }

    }
    //
    let mut min_distance = n_line*n_column;
    distance_to_end[end_position]=0;
    'search: for i_dis in 0..=n_line*n_column
    {
        for ((i,j),current_height) in map.indexed_iter()
        {
            if distance_to_end[(i, j)]==i_dis
            {
                if i >= 1 {
                    let p_next = (i - 1, j);
                    if map[p_next] >= current_height - 1 && distance_to_end[p_next]>distance_to_end[(i, j)]
                    {
                        distance_to_end[p_next] = i_dis+1;
                        if map[p_next] ==0
                        {
                            min_distance = distance_to_end[p_next];
                            break 'search;
                        }
                    }
                }
                if j >= 1 {
                    let p_next = (i, j - 1);
                    if map[p_next] >= current_height - 1 && distance_to_end[p_next]>distance_to_end[(i, j)]
                    {
                        distance_to_end[p_next] = i_dis+1;
                        if map[p_next] ==0
                        {
                            min_distance = distance_to_end[p_next];
                            break 'search;
                        }
                    }
                }
                if i < map.len_of(Axis(0)) - 1 {
                    let p_next = (i + 1, j);
                    if map[p_next] >= current_height - 1 && distance_to_end[p_next]>distance_to_end[(i, j)]
                    {
                        distance_to_end[p_next] = i_dis+1;
                        if map[p_next] ==0
                        {
                            min_distance = distance_to_end[p_next];
                            break 'search;
                        }
                    }
                }
                if j < map.len_of(Axis(1)) - 1 {
                    let p_next = (i, j+1);
                    if map[p_next] >= current_height - 1 && distance_to_end[p_next]>distance_to_end[(i, j)]
                    {
                        distance_to_end[p_next] = i_dis+1;
                        if map[p_next] ==0
                        {
                            min_distance = distance_to_end[p_next];
                            break 'search;
                        }
                    }
                }
            }
        }

    }
    let duration = start.elapsed();
    println!("Time elapsed in total is: {:?}", duration);
    println!("Problem A : minimum distance : {} ", distance_from_start[end_position]);
    println!("Problem B : minimum distance : {} ", min_distance);
    //println!("Problem B : maximum scenic score : {}", max_view);
}
