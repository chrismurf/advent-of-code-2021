use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// fn main() -> io::Result<()> {
//     let file = File::open("input.txt")?;
//     let reader = BufReader::new(file);
    
//     let mut total_distance = 0;
//     let mut depth = 0;

//     for line in reader.lines() {
//         let l = line.unwrap();
//         let mut iter = l.split_whitespace();
//         let direction = iter.next().unwrap();
//         let distance = iter.next().unwrap().parse::<i32>().unwrap();
//         match direction.as_ref() {
//             "forward" => total_distance += distance,
//             "down" => depth += distance,
//             "up" => depth -= distance,
//             _ => println!("Bad line!")
//         }
//     }
//     println!("Distance: {}, Depth: {}, Product: {}", total_distance, depth, total_distance*depth);

//     Ok(())
// }


fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    
    let mut aim = 0;
    let mut total_distance = 0;
    let mut depth = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        let mut iter = l.split_whitespace();
        let direction = iter.next().unwrap();
        let distance = iter.next().unwrap().parse::<i32>().unwrap();
        match direction.as_ref() {
            "forward" => {
                total_distance += distance;
                depth += distance * aim;

            },
            "down" => aim += distance,
            "up" => aim -= distance,
            _ => println!("Bad line!")
        }
    }
    println!("Distance: {}, Depth: {}, Product: {}", total_distance, depth, total_distance*depth);

    Ok(())
}
