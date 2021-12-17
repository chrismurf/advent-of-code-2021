use std::fs::File;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
pub struct PolymerInstructions {
    // Histogram of current pairs of bytes in the string
    histogram: HashMap<(u8, u8), u64>,
    rules: HashMap<(u8, u8), [(u8, u8); 2]>,
    final_byte: u8,
}

// Given a string reference, returns a HashMap containing the count of 
// each byte pair found in the string, and the final byte.
fn pair_histogram(string: &str) -> HashMap<(u8, u8), u64> {
    let string = string.trim().to_owned();
    let mut histogram: HashMap<(u8, u8), u64> = HashMap::new();
    // convert template string to pair entries in histogram, and final_byte
    let left = string.bytes();
    let mut right = string.bytes();
    right.next();
    for pair in left.zip(right) {
        *histogram.entry(pair).or_insert(0) += 1;
    }
    histogram
}

impl PolymerInstructions {
    fn from_file(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // Get histogram of pairs in initial template, and final template byte
        let template = lines.next().unwrap().unwrap();
        let histogram = pair_histogram(&template);
        let final_byte = template.bytes().last().unwrap();
        
        // skip empty line
        lines.next();

        // parse rules
        let mut rules: HashMap<(u8, u8), [(u8, u8); 2]> = HashMap::new();
        for line_res in lines {
            let line = line_res.unwrap();
            let mut bytes = line.trim().bytes();
            // convert AC -> B\n into (A, C) -> (A, B), (B, C)
            let a = bytes.nth(0).unwrap();
            let c = bytes.nth(0).unwrap();
            let b = bytes.nth(4).unwrap();
            
            rules.insert((a,c), [(a, b), (b, c)]);

        }
        PolymerInstructions { histogram, rules, final_byte }
    }

    /// Step the current histogram forward once, based on the rules
    fn step(&mut self) {
        let mut new_histogram = HashMap::new();

        for (pair, &count) in self.histogram.iter() {
            let new_pairs = self.rules[pair];
            *new_histogram.entry(new_pairs[0]).or_insert(0) += count;
            *new_histogram.entry(new_pairs[1]).or_insert(0) += count;
        }
        self.histogram = new_histogram;
    }

    /// Get the current distribution of bytes
    pub fn byte_histogram(&self) -> HashMap<u8, u64> {
        let mut hist = HashMap::new();
        hist.insert(self.final_byte, 1);

        for ((byte, _), count) in self.histogram.iter() {
            *hist.entry(*byte).or_insert(0) += count;
        }
        hist
    }
}

fn main() -> io::Result<()> {
    let mut instructions = PolymerInstructions::from_file("input.txt");
    for _ in 0..10 { instructions.step(); }

    let histogram = instructions.byte_histogram();
    let (&max_char, max_qty) = histogram.iter()
        .max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let (&min_char, min_qty) = histogram.iter()
        .min_by(|a, b| a.1.cmp(&b.1)).unwrap();

    println!("After 10 steps, max char is {} and min is {}. Difference in quantity is {}.",
        max_char as char, min_char as char, max_qty - min_qty);

    for _ in 0..30 { instructions.step(); }

    let histogram = instructions.byte_histogram();
    let (&max_char, max_qty) = histogram.iter()
        .max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let (&min_char, min_qty) = histogram.iter()
        .min_by(|a, b| a.1.cmp(&b.1)).unwrap();

        println!("After 40 steps, max char is {} and min is {}. Difference in quantity is {}.",
        max_char as char, min_char as char, max_qty - min_qty);

    Ok(())
}
