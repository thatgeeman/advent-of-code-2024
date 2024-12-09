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
};
use regex::{Regex, Match};
 
 #[derive(Debug)]
struct Node <T>{ // type T
    cur: T,
    next: Option<Box<Node<T>>> //smart pointer and Optional 
    // https://www.youtube.com/watch?v=2q1AzGUwL7M&ab_channel=CSHonors%40Illinois
}

impl<T> Node<T> {
    fn set_next(&mut self, next: Node<T>) {
        self.next = Some(Box::new(next));
    }
}

fn main(){ 

    let input_result = File::open("data/day5_a.txt");
    let input = match input_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Contents of file: {:?}", input);

    let reader = BufReader::new(input);
    let mut ln: i32 = 0;  
    let mut node: Node<i32> = Node {cur: 0, next: None};
    for line in reader.lines() {
        let line: String = line.unwrap(); 
        if line.is_empty() {
            break;
        }
        let re = Regex::new(r"(\d{2})").unwrap(); // find the groups of digits
        let mut re_iter = re.find_iter(&line);
        let child = re_iter.next().unwrap().as_str();
        let parent = re_iter.next().unwrap().as_str();
        if ln == 0 {
            node = Node {cur: parent.parse::<i32>().unwrap(), next: None}; 
            node.set_next(Node {cur: child.parse::<i32>().unwrap(), next: None});
        } else {
            println!("Child: {:?}, Parent: {:?}", child, parent);
            // TODO
            node.set_next(Node {cur: child.parse::<i32>().unwrap(), next: None});
            node.set_next(Node {cur: parent.parse::<i32>().unwrap(), next: None});
        }
        ln += 1;  
    
    } 
    //parse the linked list 
     
}