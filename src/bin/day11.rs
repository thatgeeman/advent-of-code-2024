/*
- stones aranged straight with numbers engraved on it
- the stones change everytime we blink: stones split to 2 or numbers change
- rules for change: 0 -> 1, even number of digits -> two stones 1000 becomes (left half) 10, (right half) 0 with no leading zeros
- if none applies: replace stone with 2024xnumber
*/

use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{prelude::*, BufReader},
};

fn len_check(x: i64) -> bool {
    let s = x.to_string();
    s.len() % 2 == 0
}

fn split_num(x: i64) -> Vec<i64> {
    let s = x.to_string();
    let half = s.len() / 2 as usize;
    let (left, right) = s.split_at(half);
    let left = left.parse::<i64>().unwrap();
    let right = right.parse::<i64>().unwrap();
    vec![left, right]
}

fn restructure(x: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut new_vec = Vec::new();
    for row in x.iter() {
        for &col in row.iter() {
            new_vec.push(vec![col]);
        }
    }
    new_vec
}

fn main() {
    let input_result = File::open("data/day11_a.txt");
    let input = match input_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Contents of file: {:?}", input);
    let mut reader = BufReader::new(input);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    println!("Line: {}", line);
    println!("Length of Line: {}\n", line.len());
    let mut data: Vec<Vec<i64>> = Vec::new();
    for (idx, c) in line.split_whitespace().enumerate() {
        if let Ok(d) = c.parse::<i64>() {
            data.push(vec![d as i64]);
        }
    }
    let mut new_data = Vec::new();
    let mut count_blinks = 0;
    let n_blinks = 25;
    let mut prev_lens = 0;
    let mut new_lens = 0;
    while n_blinks > count_blinks {
        if count_blinks > 0 {
            new_data = (&new_data[new_data.len().saturating_sub(new_lens)..])
                .to_vec()
                .clone();
            // new_data = new_data[new_lens..].to_vec();
            data = new_data.clone();
            // reset new_data
            new_data = Vec::new();
        }
        for (i, row) in data.iter().enumerate() {
            let mut new_row = Vec::new();
            for &col in row.iter() {
                let val = if col == 0 {
                    new_row.push(1);
                } else if len_check(col) {
                    let slices = split_num(col);
                    for item in slices {
                        new_row.push(item);
                    }
                } else {
                    new_row.push(col * 2024);
                };
            }
            new_data.push(new_row);
        }
        println!("Data {:?}", data);
        println!("New Data: Raw {:?}", new_data);
        prev_lens = data.len();
        new_data = restructure(new_data.clone());
        new_lens = new_data.len();
        println!(
            "New data: After {} Blinks: {:?}",
            count_blinks + 1,
            new_data
        );

        // remove the previous slice
        println!("Input len: {} | Output len: {}", prev_lens, new_lens);
        count_blinks += 1;
        println!("\n");
    }
    // new_data = (&new_data[prev_lens..]).to_vec().clone();
    println!("--Final Summary--");
    println!("Data: {:?}", data);
    println!("New Data: {:?}", new_data);
    // new_data = new_data.into_iter().flatten().collect();
    println!("Number of stones: {}", new_data.len());
}
