use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// Get Some(value) for neighboring squares, returning None for those out of bounds
pub fn neighbors<T: Clone>(data: &Vec<Vec<T>>, row: usize, col: usize) -> [Option<T>; 4] {
    let nullset: Vec<T> = vec![];
    [
        if row > 0 { data.get(row-1).unwrap_or(&nullset).get(col).cloned() } else { None },
        data.get(row+1).unwrap_or(&nullset).get(col).cloned(),
        if col > 0 { data.get(row).unwrap_or(&nullset).get(col-1).cloned() } else { None },
        data.get(row).unwrap_or(&nullset).get(col+1).cloned(),
    ]
}


pub struct HeightMap {
    heights: Vec<Vec<u8>>
}

impl HeightMap {
    pub fn from_file(filename : &str) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut heights: Vec<Vec<u8>> = Vec::new();
        for line in reader.lines() {
            heights.push(line
                .unwrap()
                .bytes()
                .map(|x| x-('0' as u8))
                .collect()
            );
        }
        HeightMap { heights }
    }

    pub fn from_vec(heights : Vec<Vec<u8>>) -> Self {
        HeightMap { heights }
    }

    pub fn find_low_points(&self) -> Vec<(usize, usize)> {
        let mut low_points = Vec::new();
        for row in 0..self.heights.len() {
            for col in 0..self.heights[row].len() {
                let value = self.heights[row][col];
                if neighbors(&self.heights, row, col).iter().all(
                    |x| x.is_none() || x.unwrap() > value
                ) {
                    low_points.push((row, col));
                }
            }
        }
        low_points
    }

    pub fn basin_map(&self) -> (Vec<u32>, Vec<Vec<Option<u32>>>) {
        let low_points = self.find_low_points();
        // Create an empty basin map, then populate the low_points with unique IDs
        let mut basins = vec![vec![None; self.heights[0].len()]; self.heights.len()];
        for (i, (x,y)) in low_points.iter().enumerate() {
            basins[*x][*y] = Some(i as u32);
        }
        let mut basin_sizes: Vec<u32> = vec![1; low_points.len()];

        // Now iterate through the map, over and over, until we stop growing the basins.  This is
        // a pretty dumb way to do this.  We should probably grow out from the seeds.
        loop {
            let mut filled_something_new = false;
            for row in 0..basins.len() {
                for col in 0..basins[0].len() {
                    if self.heights[row][col] != 9 && basins[row][col].is_none() {
                        for neighbor in neighbors(&basins, row, col) {
                            if neighbor.is_some() && neighbor.unwrap().is_some() {
                                basins[row][col] = neighbor.unwrap();
                                basin_sizes[neighbor.unwrap().unwrap() as usize] += 1;
                                filled_something_new = true;
                                break;
                            }
                        }
                    }
                }   
            }
            if !filled_something_new { break; }
        }
        (basin_sizes, basins)
    }

    pub fn sum_of_risks(&self) -> u32 {
        self.find_low_points().iter().map(|(x,y)| self.heights[*x][*y] as u32 + 1).sum()
    }
}

fn main() -> io::Result<()> {
    let height_map = HeightMap::from_file("input.txt");
    println!("Sum of risks: {}", height_map.sum_of_risks());
    println!("Number of low points: {}", height_map.find_low_points().len());
    let (sizes, _basins) = height_map.basin_map();
    let mut sorted_sizes = sizes.clone();
    sorted_sizes.sort();
    sorted_sizes.reverse();
    println!("Product of three largest basin areas: {}",
             sorted_sizes[0]*sorted_sizes[1]*sorted_sizes[2]);

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example() {
        let heights = HeightMap::from_vec(vec![
            vec![2,1,9,9,9,4,3,2,1,0],
            vec![3,9,8,7,8,9,4,9,2,1],
            vec![9,8,5,6,7,8,9,8,9,2],
            vec![8,7,6,7,8,9,6,7,8,9],
            vec![9,8,9,9,9,6,5,6,7,8],
        ]);

        let low_points = heights.find_low_points();
        assert_eq!(low_points.len(), 4);
        assert!(low_points.contains(&(0, 1)));
        assert!(low_points.contains(&(0, 9)));
        assert!(low_points.contains(&(2, 2)));
        assert!(low_points.contains(&(4, 6)));

        let (sizes, _basins) = heights.basin_map();
        assert_eq!(sizes.len(), 4);
        assert_eq!(sizes.iter().filter(|x| **x == 9).count(), 2);
        assert!(sizes.contains(&14));
        assert!(sizes.contains(&3));
    }
}