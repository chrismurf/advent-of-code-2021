use std::fs::File;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
pub struct PolymerInstructions {
    template: String,
    rules: HashMap<(u8, u8), u8>
}

impl PolymerInstructions {
    fn new_with_template(template: String) -> Self {
        PolymerInstructions {template, rules: HashMap::new()}
    }

    fn from_file(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut iter = reader.lines();
        let template = iter.next().unwrap().unwrap().trim().to_owned();
        let mut instructions = PolymerInstructions::new_with_template(template);
        iter.next(); // Skip empty line

        for line_res in iter {
            let line = line_res.unwrap();
            let mut bytes = line.bytes();
            // split AA -> B\n into (A, A) and B
            instructions.rules.insert(
                (bytes.nth(0).unwrap(), bytes.nth(0).unwrap()), 
                bytes.nth(4).unwrap()
            );

        }
        instructions
    }

    fn process_string(&self, s: &str) -> String {
        let mut new_bytes: Vec<u8> = Vec::new();
        let mut iter = s.bytes().peekable();
        loop {
            let a = iter.next().unwrap();
            match iter.peek() {
                Some(&b) => {
                    new_bytes.push(a);
                    new_bytes.push(self.rules[&(a,b)]);
                },
                None => {
                    new_bytes.push(a);
                    return String::from_utf8(new_bytes).unwrap();
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let instructions = PolymerInstructions::from_file("input.txt");

    let mut s = instructions.template.clone();
    for _ in 0..10 {
        s = instructions.process_string(&s);
    }

    let mut histogram: HashMap<u8,u32> = HashMap::new();

    for b in s.bytes() {
        *histogram.entry(b).or_insert(0) += 1;
    }

    let (max_char, max_qty) = histogram.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let (min_char, min_qty) = histogram.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();

    println!("{}", max_qty - min_qty);

    Ok(())
}

#[test]
fn test_example() {
    let instructions = PolymerInstructions::from_file("input_example.txt");
    let mut s = instructions.template.clone();
    s = instructions.process_string(&s);
    assert_eq!(s, "NCNBCHB");
    s = instructions.process_string(&s);
    assert_eq!(s, "NBCCNBBBCBHCB");
    s = instructions.process_string(&s);
    assert_eq!(s, "NBBBCNCCNBBNBNBBCHBHHBCHB");
    s = instructions.process_string(&s);
    assert_eq!(s, "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");
}