/*
- safety manuals printing 
- page ordering rule: X|Y menas both page number X and Y must be updated 
- page ordering rule: page X must be printed before page Y
- pages to be produced in each update also provided - not in correct order.
- if update includes X and Y then X must be printed before Y (anywhere before)
- identifying which updates are already in the right order
- ordering rules involving missing page numbers are ignored
- need to know the middle page number of each update being printed.
- take sum of correcly ordered updates
*/

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::{HashSet, HashMap}, 
}; 
use regex::Regex;


fn main(){ 
    
    let input_result = File::open("data/day5_a.txt");
    let input = match input_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Contents of file: {:?}", input);
    let mut node_items: HashSet<(i32, i32)> = HashSet::new();
    let mut reader = BufReader::new(input);   
    let mut order: Vec<Vec<i32>> = Vec::new();
    let re = Regex::new(r"(\d{2})").unwrap(); // find the groups of digits
    let re2 = Regex::new(r"\d+").unwrap(); // for order
    let mut switch = false;
    for line in reader.by_ref().lines() {
        let line: String = line.unwrap(); 
        if line.is_empty() { 
            switch = true;
            continue;
        }
        if !switch {
            // read the position
            let mut re_iter = re.find_iter(&line);
            let next = re_iter.next().unwrap().as_str().parse::<i32>().unwrap();
            let prev = re_iter.next().unwrap().as_str().parse::<i32>().unwrap();  
            
            // store the parent and child nodes 
            node_items.insert((next, prev)); 
        } else {
            // stop reading the position and start reading the order
            let matches: Vec<i32> = re2.find_iter(&line)
                                        .map(|mat| mat.as_str().parse::<i32>().unwrap())
                                        .collect();
            if matches.len()>0 {
                order.push(matches);
            }
        }
    }
    
    println!("Order: {:?}", order);
    println!("Node Items: {:?}", node_items);
    let mut sum = 0;
    for o in order.iter() { // o is a vec 
        if check_order(o, &node_items) {  
            println!("update: {:?} is correct", o);
            let mid_val_index = if o.len() % 2 == 0 {
                o.len() / 2
            } else {
                (o.len() - 1) / 2
            };
            println!("mid value: {:?}", o[mid_val_index]);
            sum += o[mid_val_index];
        }
    }
    println!("Sum: {:?}", sum);
} 

fn check_order(order: &Vec<i32>, rules: &HashSet<(i32, i32)>) -> bool {
    let map: HashMap<i32, usize> = order.iter().enumerate().map(|(i, &page)| (page, i)).collect();

    for &(x, y) in rules {
        if let (Some(&pos_x), Some(&pos_y)) = (map.get(&x), map.get(&y)) {
            if pos_x >= pos_y {
                return false;
            }
        }
    }
    true
}