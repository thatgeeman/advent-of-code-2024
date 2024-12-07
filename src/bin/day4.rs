/*
- find XMAS 
- can appear horizontal, vertical, diagonal, written backwards, or even overlapping other words
- horizontal and backwards: regex for XMAS and SAMX
- diagonal: iterate column wise and row wise with an offset (can be used for vertical and horizontal as well)
- vertical and backwards: iterate column wise 
- overlapping: for example: XMASAMX counts as 2 XMAS
*/ 
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

fn re_counter(s: &str) -> i32 {
    let re = Regex::new(r"XMAS").unwrap();
    let rer = Regex::new(r"SAMX").unwrap();
    let mut count: i32 = 0; 
    if re.is_match(s) {
        count += 1;
        println!("Match: {:?} (fwd)", s);
    }
    if rer.is_match(s) {
        count += 1;
        println!("Match: {:?} (rev)", s);
    }
    count
}

fn main(){  
    let input_result = File::open("data/day4_a.txt");
    let input = match input_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Contents of file: {:?}", input);
    
    let reader = BufReader::new(input);
    
    let window: usize = 4;
    // re.find_iter()
    let mut count: i32 = 0;  
    let mut a: Vec<Vec<char>> = Vec::new();
    for line in reader.lines(){  
        let line: Vec<char> = line.unwrap().chars().collect();
        println!("Line: {:?}", line.clone());
        a.push(line); 
    } 
    let max_len= a.len();
    let line_len= a[0].len();
 
    println!("Text of length: {:?}", max_len );  
    println!("Line Length: {:?}", line_len);

    for i in 0..max_len {
        for j in 0..line_len {
            // right
            if j + window <= line_len {
                let mut s: String = String::new();
                for k in 0..window {
                    s.push(a[i][j+k]);
                }
                println!("String (right): {:?}", s);
                count += re_counter(&s);
            }
            // down
            if i + window <= max_len {
                let mut s: String = String::new();
                for k in 0..window {
                    s.push(a[i+k][j]);
                }
                println!("String (down): {:?}", s);
                count += re_counter(&s);
            }
            // diagonal right
            if i + window <= max_len && j + window <= line_len {
                let mut s: String = String::new();
                for k in 0..window {
                    s.push(a[i+k][j+k]);
                }
                println!("String (diagonal right): {:?}", s);
                count += re_counter(&s);
            }
            // anti-diagonal left
            if i + window <= max_len && j + 1 >= window {
                let mut s: String = String::new();
                for k in 0..window {
                    s.push(a[i+k][j-k]);
                }
                println!("String (anti-diagonal): {:?}", s);
                count += re_counter(&s);
            }
        }
        println!("End of line: {:?}", i);

    }
    println!("Count: {:?}", count);
}



