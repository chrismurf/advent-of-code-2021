use std::fs::File;
use std::io::{prelude::*, BufReader};

type Window3x3 = [[bool; 3]; 3];

struct FilterableImage {
    mapping: Vec<bool>,
    image : Vec<Vec<bool>>,
    width: usize,
    height: usize,
    background: bool
}

impl FilterableImage {
    fn from_file(path: &str) -> Self {
        let file = File::open(path)
            .unwrap_or_else(|_| panic!("File 'input.txt' not readable.") );
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        // Read filter mapping
        let mapping = lines.next()
            .unwrap_or_else(|| panic!("Empty file!"))
            .unwrap_or_else(|err| panic!("IO Error with input.txt: {}", err))
            .trim()
            .bytes()
            .map(|x| x == '#' as u8)
            .collect();

        // consume empty line
        lines.next()
            .unwrap_or_else(|| panic!("Empty file!"))
            .unwrap_or_else(|err| panic!("IO Error with input.txt: {}", err));

        // Read image
        let mut image: Vec<Vec<bool>> = Vec::new();
        for line in lines {
            let row = line
                .unwrap_or_else(|err| panic!("IO Error with input.txt: {}", err))
                .trim()
                .bytes()
                .map(|x| x == '#' as u8)
                .collect();
            image.push(row);
        }

        // Store height and width for convenience
        let height = image.len();
        let width = if image.len() > 0 { image[0].len() } else { 0 };
        
        FilterableImage {mapping, image, width, height, background: false}
    }

    fn get(&self, x: isize, y: isize) -> bool {
        if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
            self.background
        } else {
            self.image[y as usize][x as usize]
        }
    }

    fn get_3x3(&self, x: isize, y: isize) -> Window3x3 {
        [
            [self.get(x-1, y-1), self.get(x, y-1), self.get(x+1, y-1)],
            [self.get(x-1, y), self.get(x, y), self.get(x+1, y)],
            [self.get(x-1, y+1), self.get(x, y+1), self.get(x+1, y+1)],
        ]
    }

    fn get_filtered_3x3(&self, x: isize, y: isize) -> bool {
        let data = self.get_3x3(x, y);

        let filter_index = data.iter()
            .flatten()
            .enumerate()
            .fold(0usize, |acc, (i, el) | {
                acc + ((*el as usize) << (8-i))
            });
        self.mapping[filter_index]
    }

    fn filter(&self) -> Self {
        let mut filtered : Vec<Vec<bool>> = Vec::with_capacity(self.height+2);
        for y in -1..=self.height as isize {
            let mut row: Vec<bool> = Vec::with_capacity(self.width+2);
            for x in -1..=self.width as isize {
                row.push(self.get_filtered_3x3(x, y));
            }
            filtered.push(row);
        }

        let background = if self.background { self.mapping[511] } else { self.mapping[0] };

        FilterableImage {
            mapping: self.mapping.clone(),
            image: filtered,
            width: self.width + 2,
            height: self.height + 2,
            background
        }
    }

    fn draw(&self) {
        for y in 0..self.height as isize {
            for x in 0..self.width as isize {
                print!("{}", if self.get(x, y) { "█" } else { " " });
            }
            println!();
        }

    }

    fn count_lit(&self) -> usize {
        self.image.iter()
            .flatten()
            .filter(|&&x| x)
            .count()
    }
}

fn main() {
    let mut lit_after_two: usize = 0;
    let mut image = FilterableImage::from_file("input.txt");
    for i in 0..50 {
        if i == 2 { lit_after_two = image.count_lit(); }
        println!("");
        image.draw();
        image = image.filter();
    }
    println!("");
    image.draw();

    println!("After two: {}, after 50: {}", lit_after_two, image.count_lit());
}

#[test]
fn test() {
    let image = FilterableImage::from_file("input_example.txt");
    let filtered = image.filter();
    let filtered = filtered.filter();
    assert_eq!(filtered.count_lit(), 35);
}