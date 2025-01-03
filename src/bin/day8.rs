/* 
- upper, lower or digits are antennas 
- antinodes appear in a line in line with the two other antennas of same frequency 
- only when one of the antennas is twice as far away as the other, ie distance from antinode to antenna is twice the distance between the other antenna and the antinode
- Overlapped antennas and antinode also ocounted
- loop over all antennas, for each antenna, find another antenna of same frequency 
- check if an antinode can be placed with the distance constraint and also in the map of the allowable positions (point contanined in box)
*/

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::{HashSet, HashMap}, 
};  


fn main(){ 
    
    let input_result = File::open("data/day8_a.txt");
    let input = match input_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Contents of file: {:?}", input);
    let mut reader = BufReader::new(input);
    let mut a = Vec::<Vec<String>>::new();
    let mut antennas = Vec::new();
    let mut antennas_pos = Vec::new();
    let mut has_antenna_idx = Vec::<i32>::new();
    for (row, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut b = Vec::new();
        for (col, word) in line.chars().into_iter().enumerate() {
            b.push(word.to_string());
            if word != '.' {
                antennas.push((row as i32, col as i32, word));
                antennas_pos.push((row as i32, col as i32));
                has_antenna_idx.push(row as i32);
            }  
        }
        a.push(b);
    }
    let h: i32 = a.len() as i32;
    let w: i32 = a[0].len() as i32;
    println!("Input: {:?} with h: {}, w: {}", a, h, w);
    println!("Antennas: {:?}", antennas);
    println!("Has antenna idx: {:?}", has_antenna_idx);
    let mut antinodes = HashSet::new();
    print_state(&a);
    // loop pairs of antennas with the same frequency
    let mut count: i32=0;

    for i in 0..antennas.len() {
        let (x1, y1, freq1) = antennas[i];
        for j in 0..antennas.len() {
            let (x2, y2, freq2) = antennas[j]; 
            if i == j {
                // skip if the same antenna
                continue;
            }
            if freq1 == freq2 {
                let mut found_valid_point = false;
                // distnace between points should be 
                // - 2xdistance bewteen antinode and one antenna
                // - 1xdistance between antinode and other antenna
                let dx = x2 - x1;
                let dy = y2 - y1;
                println!("Distance between antennas at ({}, {}) and ({}, {}) is: ({}, {})", x1, y1, x2, y2, dx, dy);
                // find new coordinate at 2x of x1, y1
                for (factor1, factor2) in &[(-1.0, 1.0), (1.0, -1.0), (-1.0, -1.0), (1.0, 1.0)] { 
                    let ax1 = x1 + (factor1 * dx as f64) as i32;
                    let ay1 = y1 + (factor1 * dy as f64) as i32;
                    // check if the new coordinate is within the map 
                    if antinodes.contains(&(ax1, ay1)) || (x2==ax1 && y2==ay1) {
                        // if antinode is same as x1, y1, skip
                        continue;
                    }
                    
                    if check_pos_box((0, 0), (h - 1, w - 1), (ax1, ay1)) {
                        if is_collinear((x1, y1), (x2, y2), (ax1, ay1)) {
                            antinodes.insert((ax1, ay1));
                            count += 1;
                            found_valid_point = true;
                            println!("Accepted antinode at: ({}, {})", ax1, ay1);
                        }
                    } 
                    // print_state(&a);
                    let d1 = squared_distance((ax1, ay1), (x1, y1));
                    let d2 = squared_distance((ax1, ay1), (x2, y2));
                    println!("Distance between antinode and antenna1: {}, Distance between antinode and antenna2: {}", d1, d2);
                    if d1 == d2*2 || d2 == d1*2 {
                        println!("Distance ratio is not 2:1 or 1:2; antinode at: ({}, {})", ax1, ay1);
                        count -= 1;
                        antinodes.remove(&(ax1, ay1));
                    }

                    // find new coordinate at 2x of x2, y2
                    let ax2 = x2 + (factor2 * dx as f64) as i32;
                    let ay2 = y2 + (factor2 * dy as f64) as i32;
                    
                    //dont add if already in antonodes 
                    if antinodes.contains(&(ax2, ay2)) || (x1==ax2 && y1==ay2) {
                        // if antinode is same as x2, y2, skip
                        // break;
                        continue;
                    }

                    // check if the new coordinate is within the map 
                    if check_pos_box((0, 0), (h - 1, w - 1), (ax2, ay2)) {
                        if is_collinear((x1, y1), (x2, y2), (ax2, ay2)) {
                            antinodes.insert((ax2, ay2));
                            count += 1;
                            found_valid_point = true;
                            println!("Accepted antinode at: ({}, {})", ax2, ay2);
                        }
                    } 
                    let d1 = squared_distance((ax2, ay2), (x1, y1));
                    let d2 = squared_distance((ax2, ay2), (x2, y2));
                    if d1 == d2*2 || d2 == d1*2 {
                        println!("Distance ratio is not 2:1 or 1:2; antinode at: ({}, {})", ax2, ay2);
                        count -= 1;
                        antinodes.remove(&(ax2, ay2));
                    }
                    
                    // print_state(&a);
                    
                    if ax1 > a.len() as i32 || ax2 > a.len() as i32 || ay1 > a[0].len() as i32 || ay2 > a[0].len() as i32 {
                        // if the factor is greater than the length of the map, break
                        println!("Factor greater than length of map, breaking");
                        break;
                    }
                    if ax1 < 0 || ax2 < 0 || ay1 < 0 || ay2 < 0 {
                        // if the factor is less than 0, break
                        println!("Factor less than 0, breaking");
                        break;
                    }
                    if !found_valid_point {
                        break;
                    }  
                    
                //     factor += 1.0;
                }
            }
        }
    }

    println!("Antinodes: {:?}", antinodes);
    println!("Len of antinodes: {:?}", antinodes.len());
    // print in result format 
    let mut non_overlapping_count = 0;
    let mut overlapping_count = 0;

    for v in antinodes.iter() {
        if antennas_pos.contains(v) {
            println!("Antinode overlaps with antenna at position: {:?}", v);
            a[v.0 as usize][v.1 as usize] = "?".to_string();
            overlapping_count += 1;
        } else {
            a[v.0 as usize][v.1 as usize] = "#".to_string();
            non_overlapping_count += 1;
        }
    }
    println!(
        "Non-overlapping antinodes: {}, Overlapping antinodes: {}",
        non_overlapping_count, overlapping_count
    );
    // count = non_overlapping_count + overlapping_count; 
    print_state(&a);
    println!("{}", count);
    println!("Antinodes: {:?}", count + antinodes.len() as i32);   
}


fn check_pos_box(top_left: (i32, i32), bottom_right: (i32, i32), point: (i32, i32)) -> bool {
    if point.0 >= top_left.0 && point.0 <= bottom_right.0 && point.1 >= top_left.1 && point.1 <= bottom_right.1 {
        return true;
    }
    return false;
}

fn print_state(state: &Vec<Vec<String>>) {
    for line in state.iter() {
        for c in line.iter() {
            print!("{}", c);
        }
        println!();
    }
}

fn is_collinear(a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> bool {
    // Area of triangle formed by three points
    // If area is zero (or very close to zero), points are collinear
    let area = (a.0 * (b.1 - c.1) + b.0 * (c.1 - a.1) + c.0 * (a.1 - b.1)).abs();
    area == 0
}
 
fn squared_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    dx * dx + dy * dy
}