/* 
- puzzle input is a topolofical map of the surrounding area
- fill in missing hiking trails
- map indicates height at each position using scale 0-9
- good hike route: long as possible (longest), gradual uphill slope
- path that starts at 0 and ends at 9, always going up with 1 step
- no diagonal steps - only L, R, U, D from the perspective of the map (global cam)
- trailhead's score is the number of 9-height positions reachable 
- can start from 0's located anywhere in the map
Approach:
- need to use a depth/breadth first search.
- the slope can be thought of as a heuristic that should be always at delta 1
- need to keep track of state (if visited), action, and if both directions
  were valid (shared paths) up to a junction. 
*/

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::{HashSet, HashMap}, 
}; 
use regex::Regex; 

#[derive(Debug, Clone)]
struct Node {
    height: i32,
    row: usize, 
    col: usize,
    neighbors: Neighbors,  
    id: usize,
}

#[derive(Debug, Clone)]
struct Neighbors {
    right: Option<(i32,i32)>,
    left: Option<(i32,i32)>, 
    up: Option<(i32,i32)>,
    down: Option<(i32,i32)>,  // R, L, U, D
}

fn main(){ 
    // -- parsing of data begins
    let input_result = File::open("data/day10_a.txt");
    let input = match input_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Contents of file: {:?}", input);
    let reader = BufReader::new(input);   
    let data: Vec<Vec<i32>> = reader.lines()
                             .filter_map(Result::ok)
                             .map(|line| {
                                line.chars()
                                .filter_map(|c| c.to_digit(10))
                                .map(|d| d as i32)
                                .collect()
                                })
                             .collect();

    // println!("Example Data: {:?}", data);
    let mut id_counter=0;
    let mut nodes: Vec<Vec<Node>> = data.iter().enumerate()
                                .map(|(row, line)| {
                                    // loop over the items in each line
                                    line.iter().enumerate()
                                        .map(|(col, &height)| {
                                            let node = Node {
                                                id: id_counter, 
                                                height:height, 
                                                row:row, 
                                                col:col, 
                                                neighbors:
                                                 Neighbors {
                                                    right: None, 
                                                    left:None, 
                                                    up:None, 
                                                    down:None
                                                }};
                                            id_counter +=1 ;
                                            node
                                        })
                                        .collect()
                                })
                                .collect();  
    
    let height = nodes.len() as i32;
    let width = nodes[0].len() as i32;
    for r in 0..height {
        for c in 0..width { 
            let neighbors = Neighbors {
                right: if c + 1 < width  { Some((r, c + 1)) } else { None },
                left:  if c - 1 >= 0     { Some((r, c - 1)) } else { None },
                up:    if r - 1 >= 0     { Some((r - 1, c)) } else { None },
                down:  if r + 1 < height { Some((r + 1, c)) } else { None },
            }; 
            nodes[r as usize][c as usize].neighbors = neighbors;
        }
    }
    // println!("Nodes are {:?}", nodes);  
    let mut starting_points = Vec::new();
    let mut ending_points = Vec::new();
    let mut ending_points_id = Vec::new();

    for row in &nodes { // use ref so that we can use it also later.
        for node in row {
            match node.height {
                0 => starting_points.push(node.clone()),
                9 => {
                    ending_points.push(node.clone());
                    ending_points_id.push(node.id);
                },
                _ => {}
            }
        }
    }
    // -- parsing of the data complete
    // there can be multiple ending points (hills) and multiple starting points
    println!("Start nodes {:?}", starting_points);
    println!("End nodes {:?}", ending_points); 
    let slope = 1;
    let mut score: i32 = 0;
    for start_node in &starting_points {
        let mut ending_points_id_tmp = ending_points_id.clone();
        // println!("Ending points {:?}", ending_points_id_tmp);
        println!("-----\nStarting node: {:?}", start_node);
        for end_node in &ending_points {
            let mut visited: Vec<usize> = Vec::new();
            // for each starting point, we can end at all ending points in principle.d
            let mut node = start_node.clone();
            let mut possible: Vec<Node> = Vec::new(); // new positions to which we can move from current
            possible.push(node.clone());
            while possible.len() > 0 { 
                // visited.push(node.id);
                // this can be improved 
                if let Some((r, c)) = node.neighbors.right { // if available
                    // if possible
                    let right_neighbor = &nodes[r as usize][c as usize];
                    if right_neighbor.height - node.height == slope  {
                        // node = right_neighbor.clone();
                        if !visited.contains(&right_neighbor.id){
                            possible.push(right_neighbor.clone());
                        } 
                    }
                }
                if let Some((r, c)) = node.neighbors.left { // if available
                    let left_neighbor = &nodes[r as usize][c as usize];
                    if left_neighbor.height - node.height == slope {
                        // node = left_neighbor.clone();
                        if !visited.contains(&left_neighbor.id){
                            possible.push(left_neighbor.clone());
                        } 
                    }
                }
                if let Some((r, c)) = node.neighbors.up { // if available
                    let up_neighbor = &nodes[r as usize][c as usize];
                    if up_neighbor.height - node.height == slope {
                        // node = up_neighbor.clone();
                        if !visited.contains(&up_neighbor.id){
                            possible.push(up_neighbor.clone());
                        } 
                    }
                }
                if let Some((r, c)) = node.neighbors.down { // if available
                    let down_neighbor = &nodes[r as usize][c as usize];
                    if down_neighbor.height - node.height == slope { 
                        if !visited.contains(&down_neighbor.id){
                            possible.push(down_neighbor.clone());
                        }
                }
            }
            // println!("Possible: {:?}", possible.len());
            if let Some(next_node) = possible.pop() { //stack
                    // pick one randomly)  
                    node = next_node.clone();
                    visited.push(next_node.id);
                    // println!("Moved to {} ({},{}) | visited: {:?} | possible_len: {:?}", node.id, node.row, node.col, visited, possible.len()); 
                    if ending_points_id_tmp.contains(&next_node.id){
                        println!("\nReached exit at Node: {:?}", next_node);
                        possible = Vec::new();
                        ending_points_id_tmp.retain(|id| id != &next_node.id);
                        score +=1;
                        // println!("Ending points updated to {:?}\n", ending_points_id_tmp);
                    }
                        
            } else { 
                println!("No more possible positions!"); 
            }
            } 
        }
    }
    println!("Score is {}", score); 
    
}