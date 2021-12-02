use std::env;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn moving_average_increase_count(values : &[i32], window_size: usize) -> i32 {
    let history_length = window_size + 1;
    let mut increase_count = 0;
    let mut history = VecDeque::<i32>::with_capacity(history_length);

    for value in values {
        history.push_back(*value);
        // limit the history
        if history.len() > history_length { history.pop_front(); }

        if history.len() == history_length {
            if history.range(1..).sum::<i32>() > history.range(..window_size).sum::<i32>() {
                increase_count += 1;
            }
        }
    }
    return increase_count;
}

fn main() -> io::Result<()> {
    let args : Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let values : Vec<i32> = reader.lines().map(|x| x.unwrap().parse::<i32>().unwrap()).collect();

    let increase = moving_average_increase_count(&values, 3);
    
    println!("Increase: {}", increase);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_moving_average_increase_count() {
        let values = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, moving_average_increase_count(&values, 1));
        assert_eq!(5, moving_average_increase_count(&values, 3));
    }
}