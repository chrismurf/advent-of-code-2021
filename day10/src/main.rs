use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const OPEN : &str = "([{<";
const CLOSE : &str = ")]}>";
const ERROR_SCORES : [u64; 4] = [3, 57, 1197, 25137];

// Either returns Ok(stack_of_expected_characters) or Err(first_bad_character)
fn validate(string_to_validate: &str) -> Result<String, char> {
    let mut expected = String::with_capacity(128);
    for char_to_validate in string_to_validate.chars() {
        if let Some(position) = OPEN.chars().position(|c| c == char_to_validate) {
            // For every 'opening' character, push the corresponding closing character onto 'stack'
            expected.push(CLOSE.chars().nth(position).unwrap());
        } else {
            // Make sure 'closing' characters match the stack, or we have corruption
            let should_be = expected.pop();
            if should_be != Some(char_to_validate) {
                return Err(char_to_validate);
            }
        }
    }
    Ok(expected)
}

// Compute answer to part 1, and simultaneously get completion scores for part 2
fn compute_scores(lines: Vec<String>) -> (u64, Vec<u64>) {
    let mut error_score = 0u64;

    let completion_scores : Vec<u64> = lines.iter()
    // Filter out corrupted lines, generating an error score
    .filter_map(|line|
        match validate(line.trim()) {
            Err(bad_char) => {
                if let Some(bad_char_pos) = CLOSE.chars().position(|x| x == bad_char) {
                    error_score += ERROR_SCORES[bad_char_pos];
                    None
                } else {
                    panic!("Unexpected character {} found!", bad_char);
                }
            },
            Ok(stack) => { Some(stack) },
        }
    // Then for each incomplete line, compute per-row completeness score
    ).map(|expected| {
        let mut score = 0u64;
        for c in expected.chars().rev() {
            score *= 5;
            score += 1 + CLOSE.chars()
                .position(|p| p == c)
                .unwrap_or_else(|| panic!("Somehow expected character '{}'?", c)) as u64;
        }
        score
    }).collect();

    (error_score, completion_scores)
}


fn main() -> io::Result<()> {
    let file = File::open("input.txt")
        .unwrap_or_else(|_| panic!("File 'input.txt' not readable.") );
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|x| x.unwrap_or_else(|err| panic!("IO Error with input.txt: {}", err)))
        .collect();

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