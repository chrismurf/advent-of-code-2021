use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const OPEN : [char; 4] = ['[', '{', '(', '<'];
const CLOSE : [char; 4] = [']', '}', ')', '>'];

// Either returns Ok(stack_of_expected_characters) or Err(first_bad_character)
fn validate(s: &str) -> Result<Vec<char>, char> {
    let mut stack = Vec::new();
    for c in s.chars() {
        if let Some(position) = OPEN.iter().position(|&x| x == c) {
            // For every 'opening' character, push the corresponding closing character onto 'stack'
            stack.push(CLOSE[position]);
        } else {
            // Make sure 'closing' characters match the stack, or we have corruption
            let should_be = stack.pop();
            if should_be != Some(c) {
                return Err(c);
            }
        }
    }
    Ok(stack)
}

// Compute answer to part 1, and simultaneously get completion scores for part 2
fn compute_scores(lines: Vec<String>) -> (u64, Vec<u64>) {
    let mut error_score = 0u64;

    let completion_scores : Vec<u64> = lines.iter()
    // Filter out corrupted lines, generating an error score
    .filter_map(|line|
        match validate(line.trim()) {
            Err(')') => { error_score += 3; None },
            Err(']') => { error_score += 57; None },
            Err('}') => { error_score += 1197; None },
            Err('>') => { error_score += 25137; None },
            Err(c) => { panic!("Found unexpected character {}", c); },
            Ok(stack) => { Some(stack) },
        }
    // Then for each incomplete line, compute per-row completness score
    ).map(|stack| {
        let mut score = 0u64;
        for &c in stack.iter().rev() {
            score *= 5;
            score += 1 + ")]}>"
                .chars()
                .position(|p| p == c)
                .unwrap() as u64;
        }
        score
    }).collect();

    (error_score, completion_scores)
}



fn main() -> io::Result<()> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|x| x.unwrap()).collect();

    let (error_score, mut incomplete_scores) = compute_scores(lines);

    // Find the 'middle' value
    incomplete_scores.sort();
    let incomplete_score = incomplete_scores[(incomplete_scores.len()-1) / 2];

    // Answer to part 1
    println!("Corruption Score: {}", error_score);
    println!("Incomplete Score: {}", incomplete_score);

    Ok(())
}

#[test]
fn test_examples() {
    let lines: Vec<String> = vec![
        "[({(<(())[]>[[{[]{<()<>>".to_string(),
        "[(()[<>])]({[<{<<[]>>(".to_string(),
        "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
        "(((({<>}<{<{<>}{[]{[]{}".to_string(),
        "[[<[([]))<([[{}[[()]]]".to_string(),
        "[{[{({}]{}}([{[{{{}}([]".to_string(),
        "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
        "[<(<(<(<{}))><([]([]()".to_string(),
        "<{([([[(<>()){}]>(<<{{".to_string(),
        "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
    ];
    let (error_score, incomplete_scores) = compute_scores(lines);
    assert_eq!(error_score, 26397);
    assert_eq!(incomplete_scores, vec![288957, 5566, 1480781, 995444, 294]);
}