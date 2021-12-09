use std::fs::File;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;

fn segments_to_bits(segments: &str) -> u8 {
    let mut bitmask = 0;
    for b in segments.bytes() {
        let places = b-('a' as u8);
        bitmask |= 1 << places;
    }
    return bitmask
}

fn count_bits(mask: u8) -> u32 {
    (0..8u8).map(|i| ((mask & (1u8 << i)) != 0) as u32).sum()
}

type SegmentMask = u8;
type Digit = u8;

#[derive(Debug)]
pub struct DisplayObservation {
    input_masks: Vec<SegmentMask>,
    inputs : Vec<Digit>,
    output_masks: Vec<SegmentMask>,
    outputs : Vec<Digit>,
    digits: HashMap<SegmentMask, Digit>,
    segments: HashMap<Digit, SegmentMask>
}

#[derive(Debug)]
pub struct ParseError;

impl FromStr for DisplayObservation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts : Vec<&str> = s.split_whitespace().collect();
        let (input_parts, output_parts) = parts.split_at(11);
        let input_masks: Vec<u8> = (*input_parts)[..10].iter().map(|segments| segments_to_bits(segments)).collect();
        let output_masks: Vec<u8> = (*output_parts)[..4].iter().map(|segments| segments_to_bits(segments)).collect();
        assert_eq!(input_masks.len(), 10);
        assert_eq!(output_masks.len(), 4);
        Ok(DisplayObservation{
            input_masks:input_masks,
            inputs: Vec::new(),
            output_masks:output_masks,
            outputs: Vec::new(),
            digits: HashMap::new(),
            segments: HashMap::new()
        })
    }
}

impl DisplayObservation {
    fn set_mapping(&mut self, digit: Digit, mask: SegmentMask) {
        self.digits.insert(mask, digit);
        self.segments.insert(digit, mask);
    }

    fn compute_mapping(&mut self) {
        // Start by finding 1/4/7/8, which have unique segment counts
        for mask in self.input_masks.clone() {
            match count_bits(mask) {
                2 => { self.set_mapping(1, mask); },
                4 => { self.set_mapping(4, mask); },    
                3 => { self.set_mapping(7, mask); },
                7 => { self.set_mapping(8, mask); },
                _ => { },
            }
        }
        // Top Left and Middle segments are only difference between 1 and 4
        let tl_and_middle_segments : SegmentMask = self.segments.get(&4).unwrap() - self.segments.get(&1).unwrap();

        // Now use 1,4,7,8 to derive the rest
        for mask in self.input_masks.clone() {
            match count_bits(mask) {
                5 => {
                    // 2,3,5 all have five segments
                    if mask & self.segments[&1] == self.segments[&1] {
                        // 3 contains all segments in 1
                        self.set_mapping(3, mask);
                    } else if mask & tl_and_middle_segments == tl_and_middle_segments {
                        // 5 contains middle and top left segments
                        self.set_mapping(5, mask);
                    } else {
                        self.set_mapping(2, mask);
                    }
                },
                6 => {
                    // 6,9,0 all have six segments
                    if mask & self.segments[&4] == self.segments[&4] {
                        // 4 is a strict subset of 9
                        self.set_mapping(9, mask);
                    } else if mask & self.segments[&1] == self.segments[&1] {
                        // 1 is a strict subset of 0
                        self.set_mapping(0, mask);
                    } else {
                        self.set_mapping(6, mask);
                    }
                },
                _ => { }
            }
        }
        // Now run through one last time, and set inputs and outputs based on mapping
        for mask in &self.input_masks { self.inputs.push(self.digits[&mask]); }
        for mask in &self.output_masks { self.outputs.push(self.digits[&mask]); }
    }
}

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    // Accumulate count and sum as we parse through the inputs.
    let mut count1478: u64 = 0;
    let mut sum: u32 = 0;
    for line in reader.lines() {
        let mut observation : DisplayObservation = line.unwrap().parse().unwrap();
        observation.compute_mapping();
        for (i, digit) in observation.outputs.iter().enumerate() {
            if *digit == 1 || *digit == 4 || *digit == 7 || *digit == 8 { count1478 += 1; }
            sum += (*digit as u32) * 10u32.pow(3-i as u32);
        }
    }

    println!("Count of 1,4,7,8 is {}.  Sum of outputs is {}.", count1478, sum);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example() {
        let mut observation: DisplayObservation = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".parse().unwrap();
        observation.compute_mapping();
        assert_eq!(observation.inputs, vec![8,5,2,3,7,9,6,4,0,1]);
        assert_eq!(observation.outputs, vec![5,3,5,3]);
    }
}
