#[macro_use]
extern crate ndarray;
use ndarray::Array;
use ndarray::Array2;
use ndarray::Zip;
use std::env;
use std::fs;
use std::time::Instant;
fn view(map: &Array2<u32>, location: (usize, usize)) -> u32 {
    let tree_height = map[location];
    let col = map.column(location.1);
    let mut distance_up = 0;
    for ipos in (0..location.0).rev() {
        distance_up += 1;
        if col[ipos] >= tree_height {
            break;
        }
    }
    let mut distance_down = 0;
    for ipos in location.0 + 1..col.len() {
        distance_down += 1;
        if col[ipos] >= tree_height {
            break;
        }
    }
    let line = map.row(location.0);
    let mut distance_left = 0;
    for ipos in (0..location.1).rev() {
        distance_left += 1;
        if line[ipos] >= tree_height {
            break;
        }
    }
    let mut distance_right = 0;
    for ipos in location.1 + 1..line.len() {
        distance_right += 1;
        if line[ipos] >= tree_height {
            break;
        }
    }
    distance_left * distance_right * distance_up * distance_down
}

//Usage ./rust PATH_TO_INPUT_FILE
fn main() {
    const RADIX: u32 = 10;
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("===========================");
    println!("  Advent Of Code 2022");
    println!("  Day 08");
    println!("  Rust version");
    println!("===========================");
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let n_column = contents.lines().next().unwrap().chars().count();
    let n_line = contents.lines().count();
    let mut map = Array2::<u32>::zeros((0, n_column));
    for line in contents.lines() {
        let test1 = Array::from_iter(line.chars().map(|c| c.to_digit(RADIX).unwrap() as u32));
        match map.push_row(test1.view()) {
            Err(_) => panic!("Conversion KO"),
            _ => (),
        }
    }
    let mut max_from_up = Array2::<u32>::zeros((n_line, n_column));
    for i_line in 1..(n_line - 1) {
        for i_col in 1..n_column - 1 {
            max_from_up[(i_line, i_col)] =
                std::cmp::max(max_from_up[(i_line - 1, i_col)], map[(i_line - 1, i_col)])
        }
    }
    let mut max_from_down = Array2::<u32>::zeros((n_line, n_column));
    for i_line in (1..n_line - 1).rev() {
        for i_col in (1..n_column - 1).rev() {
            max_from_down[(i_line, i_col)] =
                std::cmp::max(max_from_down[(i_line + 1, i_col)], map[(i_line + 1, i_col)])
        }
    }
    let mut max_from_left = Array2::<u32>::zeros((n_line, n_column));
    for i_line in 1..n_line - 1 {
        for i_col in 1..n_column - 1 {
            max_from_left[(i_line, i_col)] =
                std::cmp::max(max_from_left[(i_line, i_col - 1)], map[(i_line, i_col - 1)])
        }
    }
    let mut max_from_right = Array2::<u32>::zeros((n_line, n_column));
    for i_line in (1..n_line - 1).rev() {
        for i_col in (1..n_column - 1).rev() {
            max_from_right[(i_line, i_col)] = std::cmp::max(
                max_from_right[(i_line, i_col + 1)],
                map[(i_line, i_col + 1)],
            )
        }
    }
    let mut visible = Array2::<u32>::zeros((n_line, n_column));
    Zip::from(&mut visible)
        .and(&map)
        .and(&max_from_up)
        .and(&max_from_down)
        .and(&max_from_left)
        .and(&max_from_right)
        .for_each(|vis, &val, &mu, &md, &ml, &mr| {
            if val > mu || val > md || val > ml || val > mr {
                *vis = 1
            } else {
                *vis = 0
            }
        });
    let number_visible: u32 = 2 * n_column as u32 + 2 * n_line as u32 - 4
        + visible.slice(s![1..-1, 1..-1]).iter().sum::<u32>();
    let max_view = map.indexed_iter().fold(0, |vmax, (i_ele, _val)| {
        std::cmp::max(vmax, view(&map, i_ele))
    });
    let duration = start.elapsed();
    println!("Time elapsed in total is: {:?}", duration);
    println!("Problem A : number of visible tree : {} ", number_visible);
    println!("Problem B : maximum scenic score : {}", max_view);
}
