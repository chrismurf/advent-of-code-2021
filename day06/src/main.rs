use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::VecDeque;

const MATURE_PERIOD : usize = 2;
const CYCLE_PERIOD : usize = 7;

fn read_input_file(filename : &str) -> VecDeque<u64> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut fish_dist = VecDeque::<u64>::from([0u64; CYCLE_PERIOD + MATURE_PERIOD]);
    let line = reader.lines().next().unwrap().unwrap();
    for age in line.trim().split(',').map(|x| x.parse::<u64>().unwrap()) {
        fish_dist[age.try_into().unwrap()] += 1;
    }
    fish_dist
}

fn step(fish_dist: &mut VecDeque<u64>) {
    let gave_birth = fish_dist.pop_front().unwrap();
    fish_dist.push_back(gave_birth); // These are all the new fishies
    fish_dist[CYCLE_PERIOD-1] += gave_birth; // These are the original fishies
    assert_eq!(fish_dist.len(), CYCLE_PERIOD + MATURE_PERIOD);
}

fn main() -> io::Result<()> {
    let mut fish_dist = read_input_file("input.txt");
    for day in 1..=256 {
        step(&mut fish_dist);
        let num_fish: u64 = fish_dist.iter().sum();
        println!("Day {} has {} fish", day, num_fish);
    }

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example() {
        let mut fish_dist = VecDeque::from([0, 1, 1, 2, 1, 0, 0, 0, 0]);
        step(&mut fish_dist);
        assert_eq!(fish_dist, VecDeque::from([1, 1, 2, 1, 0, 0, 0, 0, 0]));
        step(&mut fish_dist);
        assert_eq!(fish_dist, VecDeque::from([1, 2, 1, 0, 0, 0, 1, 0, 1]));
    }
}