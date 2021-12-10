use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // Read file into a sorted list
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut lines : Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    lines.sort();

    // Brute forcedly run through every line, counting bit values.
    let mut line_count = 0;
    let mut bit_count = vec![0; 12];
    
    for line in &lines {
        line_count += 1;

        for (c, bit) in bit_count.iter_mut().zip(line.trim().chars()) {
            if bit == '1' { *c += 1; }
        }
    }

    // bit-by-bit, convert bit_count to a string
    let majority_str : String = bit_count.iter().map(|x| {
        if *x > (line_count / 2) { return '1'; }
        else { return '0'; }
    }).collect();

    let majority_value = i64::from_str_radix(&majority_str, 2).unwrap();
    let minority_value = (!majority_value) & 0x0FFF;
    println!("Part 1: Majority={}, Minority={}, Product={}", majority_value, minority_value,
             majority_value * minority_value);

    /////////////////////////////
    // Using a different strategy for part 2 - We know that the leading bits are all the
    // same for remaining values in the list, so we can find which bit value is in the
    // majority by looking at the midpoint of the ordered list.

    let mut remaining_lines = lines.clone();
    for i in 0..12 {
        remaining_lines.sort();
        let midpoint = remaining_lines.len() / 2;
        let char_to_match = remaining_lines[midpoint].as_bytes()[i];
        remaining_lines.retain(|x| x.as_bytes()[i] == char_to_match);
        if remaining_lines.len() == 1 { break; }
    }
    let oxygen_gen_rating = i64::from_str_radix(&remaining_lines[0], 2).unwrap();

    let mut remaining_lines = lines.clone();
    for i in 0..12 {
        remaining_lines.sort();
        let midpoint = remaining_lines.len() / 2;
        let c = remaining_lines[midpoint].as_bytes()[i];
        remaining_lines.retain(|x| x.as_bytes()[i] != c);
        if remaining_lines.len() == 1 { break; }
    }
    let co2_scrubber_rating = i64::from_str_radix(&remaining_lines[0], 2).unwrap();
    println!("Part 2: O2={}, CO2={}, Product={}", oxygen_gen_rating, co2_scrubber_rating, oxygen_gen_rating*co2_scrubber_rating);

    Ok(())
}