/* 
- bridge repair
- determine test values for combinations of operators 
- test value: operators evaluated L to R not based on BODMAS 
- operators: add or multiply
- number of operators = number of spaces, ie always odd.
- find the sum of the test values that are valid
*/

use std::{
    fs::File,
    io::{prelude::*, BufReader}, 
};  
use itertools::Itertools; 

fn main(){ 
    
    let input_result = File::open("data/day7_a.txt");
    let input = match input_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Contents of file: {:?}", input);
    let mut reader = BufReader::new(input);  
    let mut val_sum: i64 = 0; 
    for line in reader.by_ref().lines() {
        let line: String = line.unwrap(); 
        // split by : 
        let line_vec: Vec<&str> = line.split(": ").collect();
        println!("Line: {:?}", line_vec);
        let result = line_vec[0].parse::<i64>().unwrap();
        let vals: Vec<i64> = line_vec[1].split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
        // nops less by 1 
        // 0 is add, 1 is multiply
        println!("Vals: {:?} and ops: {:?}", vals, vals.len()-1);
        let it = itertools::repeat_n(vec!["+", "*"], vals.len()-1).multi_cartesian_product().into_iter();
        println!("Expected Result: {:?}", result); 
        let total_trials = 2usize.pow((vals.len()-1) as u32); // 2^n 
        for (i, p) in it.enumerate() {
            println!("Trial {:?}/{:?} with {:?}", i, total_trials-1, p);
            if perform_test(&vals, &p, result) {
                val_sum += result;
                break; // break after first valid test to avoid double counting
            } 
        } 
    }
         
    println!("Final sum: {}", val_sum);
    
}

fn perform_test(vals: &Vec<i64>, ops: &Vec<&str>, result:i64) -> bool {
    let mut cur_result: i64 = vals[0];
    // println!("0: {}", cur_result);
    for (i, v) in vals.iter().enumerate().skip(1) {
        // println!("{}: {}", i, v);
        //safety  
        cur_result = match ops[i-1] {
            "+" => cur_result + v,
            "*" => cur_result * v,
            _ => panic!("Invalid operator")
        }; 
    }
    println!("Result: {}", cur_result);
    cur_result == result
}