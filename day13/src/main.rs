use std::fs::File;
use std::collections::HashSet;
use std::str::FromStr;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug,Clone,Copy)]
enum Fold {
    Vertical(u32),
    Horizontal(u32)
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct Dot {
    x: u32,
    y: u32,
}

#[derive(Debug)]
pub struct ParseError;
impl FromStr for Dot {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<u32> = s
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        Ok(Dot {x: parts[0], y: parts[1]})
    }
}

#[derive(Debug)]
struct Page {
    dots: HashSet<Dot>,
    folds: Vec<Fold>,
}

impl Page {
    fn new() -> Self {
        Page {dots: HashSet::new(), folds: Vec::new()}
    }

    fn from_file(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut page: Page = Page::new();
        for line in reader
            .lines()
            .map(|l| l.unwrap()) {

            if line.trim().is_empty() {
                // Do nothing
            } else if &line[..4] == "fold" {
                let foldpoint: u32 = line[13..].parse().unwrap();

                if &line[11..12] == "x" {
                    page.folds.push(Fold::Vertical(foldpoint));
                } else {
                    page.folds.push(Fold::Horizontal(foldpoint));
                }
            } else {
                page.dots.insert(line.parse().unwrap());
            };
        }
        page
    }

    fn fold(&self) -> Self {
        let fold = *self.folds.first().unwrap();
        let dots = self.dots.iter().map(|dot| {
            match fold {
                Fold::Horizontal(fold_y) => {
                    if dot.y < fold_y { Dot{ x: dot.x, y: dot.y } }
                    else {
                        Dot { x: dot.x, y: 2*fold_y - dot.y }
                    }
                },
                Fold::Vertical(fold_x) => {
                    if dot.x < fold_x { Dot{ x: dot.x, y: dot.y } }
                    else { Dot { x: 2*fold_x - dot.x, y: dot.y } }    
                }
            }
        }).collect();

        return Self { dots, folds: self.folds[1..].to_owned() }
    }

    fn display(&self) {
        let first_dot = *self.dots.iter().nth(0).unwrap();
        let mut min_x = first_dot.x;
        let mut max_x = first_dot.x;
        let mut min_y = first_dot.y;
        let mut max_y = first_dot.y;

        for dot in self.dots.iter() {
            if dot.x < min_x { min_x = dot.x; }
            if dot.y < min_y { min_y = dot.y; }
            if dot.x > max_x { max_x = dot.x; }
            if dot.y > max_y { max_y = dot.y; }
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.dots.contains(&Dot{x, y}) {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

fn main() -> io::Result<()> {
    let mut page = Page::from_file("input.txt");
    while !page.folds.is_empty() {
        page = page.fold();
        println!("* Page has {} dots after fold.", page.dots.len());
    }
    println!();

    page.display(); 
    Ok(())
}
