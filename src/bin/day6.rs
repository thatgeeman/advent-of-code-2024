/*
- navigate the maze 
- if facing a #, turn 90 degrees to the right
- otherwise keep moving in that direction
- stop when edge of maze is reached
- count number of steps taken
*/

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::{HashSet, HashMap}, 
}; 
use regex::Regex;


fn main(){ 
    
    let input_result = File::open("data/day6_a.txt");
    let input = match input_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Contents of file: {:?}", input);
    let mut reader = BufReader::new(input);   
    let mut state: Vec<Vec<String>> = Vec::new();
    let mut si = 0;
    let mut sj = 0;
    for (i, line) in reader.by_ref().lines().enumerate() {
        let line: String = line.unwrap(); 
        let mut line_vec: Vec<String> = Vec::new();
        for (j, c) in line.chars().into_iter().enumerate() {
            line_vec.push(c.clone().to_string());
            if &c.to_string() == "^" {
                si = i;
                sj = j;
            }
        }
        state.push(line_vec);
    }
    let maxi = state.len();
    let maxj = state[0].len();
    println!("Start State");
    print_state(&state);
    println!("Start Position of guard: ({}, {})", si, sj);
    println!("Max i: {}, Max j: {}", maxi, maxj);
    step(&mut state, si, sj, maxi, maxj);
    print_state(&state);
} 

fn step(state: &mut Vec<Vec<String>>, si: usize, sj: usize, maxi: usize, maxj: usize) {
    let mut i = si;
    let mut j = sj; 
    let mut direction = "up";
    let mut steps = 0;
    let mut path: Vec<(usize, usize)> = Vec::new();
    // assumes wall is only on 2 sides
    loop {
        
        if i==0 || i+1==maxi || j==0 || j+1==maxj {
            break;  // breat if reached the edge
        } 
        match direction {
            "up" => {
                if state[i-1][j] == "#" { // previous cell up
                    // turn 90 degs from current cell
                    if state[i][j+1] == "#" { // if target cell is wall
                        direction = "down";
                    } else {
                        direction = "right"; // alternative target cell
                    }
                } else {
                    i -= 1; // move up
                }
                
            },
            "down" => {
                if state[i+1][j] == "#" { // next cell down
                    if state[i][j-1] == "#" { // if target cell is wall
                        direction = "up";
                    } else {
                        direction = "left"; // target cell alternative
                    }
                } else {
                    i += 1; // move down
                } 
                
            },
            "left" => {
                if state[i][j-1] == "#" { // previous cell left
                    if state[i-1][j] == "#" { // if target cell is wall
                        direction = "right";
                    } else {
                        direction = "up";
                    }
                } else {
                    j -= 1; // move left
                } 
            },
            "right" => {
                if state[i][j+1] == "#" { // next cell right
                    if state[i+1][j] == "#" {  // if target cell is wall
                        direction = "left";
                    } else {
                        direction = "down";
                    }
                } else {
                    j += 1; // move right
                } 
            },
            _ => {
                panic!("Invalid direction");
            }
        }
        println!("Current Position ({}): ({}, {})", steps, i, j);
        state[i][j] = "X".to_string(); 
        steps += 1;
        path.push((i, j));
    }
    println!("Path: {:?}", path);
    println!("Steps: {}", steps);
    println!("End Position of guard: ({}, {})", i, j);
    // unique steps 
    let unique_steps = path.iter().collect::<HashSet<_>>();
    println!("Unique Steps: {:?}", unique_steps);
    println!("Unique Steps Count: {}", unique_steps.len());
}


fn print_state(state: &Vec<Vec<String>>) {
    for line in state.iter() {
        for c in line.iter() {
            print!("{}", c);
        }
        println!();
    }
}