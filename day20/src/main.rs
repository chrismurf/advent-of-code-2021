use std::fs::File;
use std::io::{prelude::*, BufReader};

type Window3x3 = [[bool; 3]; 3];

struct FilterableImage {
    filter: Vec<bool>,
    image : Vec<Vec<bool>>,
    width: usize,
    height: usize
}

impl FilterableImage {
    fn from_file(path: &str) -> Self {
        let file = File::open(path)
            .unwrap_or_else(|_| panic!("File 'input.txt' not readable.") );
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        // Read filter
        let filter = lines.next()
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
        
        FilterableImage {filter, image, width, height}
    }

    fn pixel(&self, x: isize, y: isize) -> bool {
        if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
            false
        } else {
            self.image[y as usize][x as usize]
        }
    }

    fn get_3x3(&self, x: isize, y: isize) -> Window3x3 {
        [
            [self.pixel(x-1, y-1), self.pixel(x, y-1), self.pixel(x+1, y-1)],
            [self.pixel(x-1, y), self.pixel(x, y), self.pixel(x+1, y)],
            [self.pixel(x-1, y+1), self.pixel(x, y+1), self.pixel(x+1, y+1)],
        ]
    }

    fn filter_3x3(&self, data: &Window3x3) -> bool {
        let index = data.iter()
            .flatten()
            .enumerate()
            .fold(0usize, |acc, (i, el) | {
                acc + ((*el as usize) << (8-i))
            });
        self.filter[index]
    }

    fn filter(&self) -> Self {
        let mut filtered : Vec<Vec<bool>> = Vec::with_capacity(self.height+2);
        for y in -1..=self.height as isize {
            let mut row: Vec<bool> = Vec::with_capacity(self.width+2);
            for x in -1..=self.width as isize {
                row.push(self.filter_3x3(&self.get_3x3(x, y)));
            }
            filtered.push(row);
        }

        FilterableImage {
            filter: self.filter.clone(),
            image: filtered,
            width: self.width + 2,
            height: self.height + 2
        }
    }

    fn draw(&self) {
        for y in 0..self.height as isize {
            for x in 0..self.width as isize {
                print!("{}", if self.pixel(x, y) { "█" } else { " " });
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
    let image = FilterableImage::from_file("input.txt");
    let filtered = image.filter();
    let filtered2 = filtered.filter();
    println!("{}", filtered2.count_lit());
}

#[test]
fn test() {
    let image = FilterableImage::from_file("input_example.txt");
    println!();
    image.draw();
    println!();
    let filtered = image.filter();
    filtered.draw();
    println!();
    let filtered = filtered.filter();
    filtered.draw();
    println!();
    assert_eq!(filtered.count_lit(), 35);
}