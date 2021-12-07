use std::fs::File;
use std::cmp::{min, max};
use std::io::{self, prelude::*, BufReader};

/// Implement absolute difference of two unsigned values.
/// int_abs_diff is in unstable: https://github.com/rust-lang/rust/issues/89492
fn abs_diff(x: u64, y: u64) -> u64 { max(x, y) - min(y, x) }
/// Cost function for part one - the distance is the cost (1:1)
fn identity(dx: u64) -> u64 { dx }
/// Cost function for parttwo - the distance is the Nth triangular number, which is
/// 1 + 2 + 3 + 4 + ... - each step away increases the cost by one.
fn triangular_number(dx: u64) -> u64 { dx*(dx+1)/2 }

pub struct CrabArmada {
    positions : Vec<u64>
}

impl CrabArmada {
    /// Create a CrabArmada from a single (first) line of comma separated values in a file.
    pub fn from_file(filename : &str) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let line = reader.lines().next().unwrap().unwrap();
        CrabArmada {
            positions: line.trim().split(',').map(|x| x.parse::<u64>().unwrap()).collect()
        }
    }

    /// Create a CrabArmada from a vector.
    pub fn from_vec(positions : Vec<u64>) -> Self {
        CrabArmada { positions }
    }

    /// Compute the cost of a given 'blast point', using a given cost function.  A 'best_cost' can
    /// be provided, which will allow the function to bail out early (returning None) if exceeded.
    pub fn compute_blast_point_cost(&self,
        blast_point : u64,
        best_cost : Option<u64>,
        cost_fn : fn(u64) -> u64
    ) -> Option<u64> {

        let mut cost: u64 = 0;
        for pos in &self.positions {
            cost += cost_fn(abs_diff(blast_point, *pos));
            if best_cost.is_some() && cost >= best_cost.unwrap() { return None; }
        }
        return Some(cost);
    }

    /// Find the minimal cost 'blast point', using a given cost function.
    pub fn find_blast_point(&self, cost_fn : fn(u64) -> u64) -> (u64, u64) {
        let minima = self.positions.iter().min().unwrap();
        let maxima = self.positions.iter().max().unwrap();
        let mut best_point : Option<u64> = None;
        let mut best_cost : Option<u64> = None;

        for blast_point in *minima..=*maxima {
            match self.compute_blast_point_cost(blast_point, best_cost, cost_fn) {
                Some(new_cost) => {
                    best_cost = Some(new_cost);
                    best_point = Some(blast_point);
                },
                None => { }
            }
        }

        (best_point.unwrap(), best_cost.unwrap())
    }
}

fn main() -> io::Result<()> {
    let armada = CrabArmada::from_file("input.txt");
    let (best_point, best_cost) = armada.find_blast_point(identity);
    println!("Pt1: Blast Point {} with cost {}", best_point, best_cost);

    let (best_point, best_cost) = armada.find_blast_point(triangular_number);
    println!("Pt2: Blast Point {} with cost {}", best_point, best_cost);

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example() {
        let armada = CrabArmada::from_vec(vec![16,1,2,0,4,2,7,1,2,14]);
        // Part 1
        assert_eq!(armada.find_blast_point(identity), (2, 37));
        assert_eq!(armada.compute_blast_point_cost(2, None, identity), Some(37));
        assert_eq!(armada.compute_blast_point_cost(1, None, identity), Some(41));
        assert_eq!(armada.compute_blast_point_cost(3, None, identity), Some(39));
        assert_eq!(armada.compute_blast_point_cost(10, None, identity), Some(71));

        // Part 2
        assert_eq!(armada.find_blast_point(triangular_number), (5, 168));
        assert_eq!(armada.compute_blast_point_cost(2, None, triangular_number), Some(206));
    }
}