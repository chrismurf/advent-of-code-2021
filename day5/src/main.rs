use std::default::Default;
use std::fs::File;
use std::str::FromStr;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

// We're using a sparse grid so we are size independent, and since there are likely
// a bazillion zeros.  Hooray for the does-everything HashMap.
#[derive(PartialEq, Eq, Debug)]
pub struct VentGrid {
    vents : HashMap<(i32,i32),u32>,
}

impl VentGrid {
    pub fn new_empty() -> VentGrid {
        VentGrid {
          vents: Default::default()
        }
    }

    pub fn add(&mut self, gl : &GridLine, skip_diagonal: bool) {
        if gl.x[0] != gl.x[1] && gl.y[0] != gl.y[1] && skip_diagonal { return; }

        let dx: i32 = if gl.x[0] < gl.x[1] { 1 } else if gl.x[0] > gl.x[1] { -1 } else { 0 };
        let dy: i32 = if gl.y[0] < gl.y[1] { 1 } else if gl.y[0] > gl.y[1] { -1 } else { 0 };
        let mut x : i32 = gl.x[0];
        let mut y : i32 = gl.y[0];
        loop {
            *self.vents.entry((x, y)).or_insert(0) += 1;
            if x == gl.x[1] && y == gl.y[1] { break; }
            x += dx;
            y += dy;
        }
    }

    pub fn count_atleast(&self, threshold : u32) -> u32 {
        let mut count : u32 = 0;
        for v in self.vents.values() {
            if *v >= threshold { count += 1}
        }
        count
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct GridLine {
    x : [i32; 2],
    y : [i32; 2],
}

#[derive(Debug)]
pub struct ParseError;

impl FromStr for GridLine {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut gridline : GridLine = Default::default();

        let mut i : usize = 0;
        for point_str in s.split(" -> ") {
            let coords : Vec<i32> = point_str.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            if coords.len() != 2 {
                println!("Bad coord len");
                return Err(Self::Err{})
            }
            gridline.x[i] = coords[0];
            gridline.y[i] = coords[1];
            i += 1;
        }
        Ok(gridline)
    }
}

fn read_input_file(filename : &str) -> Vec<GridLine> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut gridlines : Vec<GridLine> = Vec::new();
    for line in reader.lines() {
        gridlines.push(GridLine::from_str(line.unwrap().trim()).unwrap());
    }
    gridlines
}


fn main() -> io::Result<()> {
    let gridlines = read_input_file("input.txt");
    let mut grid_hv = VentGrid::new_empty();
    let mut grid_hvd = VentGrid::new_empty();
    for gridline in &gridlines {
        grid_hv.add(gridline, true);
        grid_hvd.add(gridline, false);
    }

    println!("Skipping diagonal, # of cells is : {:?}", grid_hv.count_atleast(2));
    println!("Counting diagonal, # of cells is : {:?}", grid_hvd.count_atleast(2));
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example() {
        let gridlines = read_input_file("example_input.txt");
        let mut grid_hv = VentGrid::new_empty();
        let mut grid_hvd = VentGrid::new_empty();
        for gridline in &gridlines {
            grid_hv.add(gridline, true);
            grid_hvd.add(gridline, false);
        }
        assert_eq!(grid_hv.count_atleast(2), 5);
        assert_eq!(grid_hvd.count_atleast(2), 12);
    }
}