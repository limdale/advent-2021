use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let lines = BufReader::new(file).lines();

    let lines_vec: Vec<String> = lines.map(|s| s.unwrap()).collect();

    let open = HashSet::from(['(', '[', '{', '<']);
    let close = HashSet::from([')', ']', '}', '>']);
    let map = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let scores_map = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let incomplete_scores_map = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let mut incomplete_scores: Vec<usize> = Vec::new();

    let mut sum = 0;

    for line in lines_vec {
        let mut symbols: VecDeque<char> = VecDeque::new();
        for c in line.chars() {
            symbols.push_back(c);
        }

        //println!("Processing: {:?}", symbols);

        let mut expected: Vec<char> = Vec::new();
        while !symbols.is_empty() {
            let symbol = symbols.pop_front().unwrap();

            if open.contains(&symbol) {
                expected.push(map[&symbol]);
            } else if close.contains(&symbol) {
                match expected.pop() {
                    None => {
                        println!("Syntax error: expected none but popped {}", symbol);
                        sum += scores_map[&symbol];
                        symbols.clear();
                        expected.clear();
                        break;
                    }
                    Some(expected_symbol) => {
                        if expected_symbol != symbol {
                            println!(
                                "Syntax error on expected: {}, actual: {}",
                                expected_symbol, symbol
                            );
                            sum += scores_map[&symbol];
                            symbols.clear();
                            expected.clear();
                            break;
                        }
                    }
                }
            }
        }

        // incomplete line
        let mut incomplete_score = 0;
        if !expected.is_empty() {
            println!("Line {} is incomplete w/ {:?}", line, expected);
            for i in (0..expected.len()).rev() {
                incomplete_score *= 5;
                incomplete_score += incomplete_scores_map[&expected[i]];
            }
            //println!("Incomplete score for {}: {}", line, incomplete_score);

            incomplete_scores.push(incomplete_score);
        }
    }

    incomplete_scores.sort();
    println!("Corrupted lines score: {}", sum);
    println!(
        "Middle incomplete score: {}",
        incomplete_scores[incomplete_scores.len() / 2]
    );

    Ok(())
}
