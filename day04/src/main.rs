use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Copy, Clone, PartialEq, Debug)]
enum Square {
    Called(u32),
    Uncalled(u32),
    Uninitialized
}

type Board = [[Square; 5]; 5];

fn read_input_file(filename : &str) -> (Vec<u32>, Vec<Board>) {
    // Read file into a sorted list
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut it = reader.lines();

    // First parse the called order
    let called_order : Vec<u32> = it.next().unwrap().unwrap().trim().split(",").map(|x| x.parse::<u32>().unwrap()).collect();

    // Now parse the boards
    let mut boards : Vec<Board> = Vec::new();

    while let Some(_) = it.next() { // Skip blank line, or assume done
        let mut board : Board = [[Square::Uninitialized; 5]; 5];
        for i in 0..5 {
            let line = it.next().unwrap().unwrap().trim().to_string();
            for j in 0..5 {
                let values : Vec<Square> = line.split_whitespace().map(|x| Square::Uncalled(x.parse::<u32>().unwrap())).collect();
                board[i][j] = values[j];
            }
        }
        boards.push(board);
    }
    (called_order, boards)
}

fn is_winning_board(board : &Board) -> bool {
    for i in 0..5 {
        let mut row_match = true;
        let mut col_match = true;
        for j in 0..5 {
            if !matches!(board[i][j], Square::Called(_)) { row_match = false; }
            if !matches!(board[j][i], Square::Called(_)) { col_match = false; }
        }
        if row_match || col_match { return true; }
    }
    return false
}

fn find_winner(called_order : &Vec<u32>, mut boards : Vec<Board>) -> Option<(u32, Board)> {
    for called_value in called_order {
        for board in &mut boards {
            for i in 0..5 {
                for j in 0..5 {
                    if board[i][j] == Square::Uncalled(*called_value) {
                        board[i][j] = Square::Called(*called_value);
                    }
                }
            }

            if is_winning_board(board) { return Some((*called_value, *board)); }
        }
    }
    None
}

fn find_last_place(called_order : &Vec<u32>, mut boards : Vec<Board>) -> Option<(u32, Board)> {
    for called_value in called_order {
        let mut winners = Vec::<usize>::new();
        let mut board_number = 0;
        let boards_left = boards.len();
        for board in &mut boards {
            for i in 0..5 {
                for j in 0..5 {
                    if board[i][j] == Square::Uncalled(*called_value) {
                        board[i][j] = Square::Called(*called_value);
                    }
                }
            }

            if is_winning_board(board) {
                if boards_left == 1 {
                    return Some((*called_value, *board));
                }
                winners.push(board_number);
            }
            board_number += 1;
        }
        for b in winners.iter().rev() { boards.remove(*b); }
    }
    None
}

fn sum_uncalled(board : &Board) -> u32 {
    let mut sum : u32 = 0;
    for i in 0..5 {
        for j in 0..5 {
            if let Square::Uncalled(value) = board[i][j] {
                sum += value;
            }
        }
    }

    return sum;
}

fn main() -> io::Result<()> {
    // Load some boards
    let (called_order, boards) = read_input_file("input.txt");
    println!("Read {} boards.", boards.len());
    let (last_called, winning_board) = find_winner(&called_order, boards.clone()).unwrap();
    println!("Winning value is {}.", last_called * sum_uncalled(&winning_board));

    let (last_called, winning_board) = find_last_place(&called_order, boards.clone()).unwrap();
    println!("Last place value is {}.", last_called * sum_uncalled(&winning_board));

    Ok(())
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_winning_board() {
        let board_bad = [[Square::Uncalled(0); 5]; 5];
        let mut board_row = [[Square::Uncalled(0); 5]; 5];
        let mut board_col = [[Square::Uncalled(0); 5]; 5];
        for i in 0..5 {
            board_row[4][i] = Square::Called(23);
            board_col[i][2] = Square::Called(5);
        }
        assert_eq!(is_winning_board(&board_row), true);
        assert_eq!(is_winning_board(&board_col), true);
        assert_eq!(is_winning_board(&board_bad), false);
    }

    #[test]
    fn test_examples() {
        let (called_order, boards) = read_input_file("example_input.txt");
        assert_eq!(called_order[0], 7);
        assert_eq!(boards.len(), 3);
        let (last_called, winning_board) = find_winner(&called_order, boards.clone()).unwrap();
        assert_eq!(last_called, 24);
        assert_eq!(sum_uncalled(&winning_board), 188);

        let (last_called, winning_board) = find_last_place(&called_order, boards.clone()).unwrap();
        assert_eq!(last_called, 13);
        assert_eq!(sum_uncalled(&winning_board), 148);
    }
}