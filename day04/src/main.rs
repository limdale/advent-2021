use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const BOARD_SIZE: usize = 5;

#[derive(Debug)]
struct Board {
    numbers: Vec<Vec<i32>>,
    marks: Vec<Vec<bool>>,
}

impl Board {
    fn push_row(&mut self, row: Vec<i32>) {
        self.numbers.push(row);
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("test-input")?;
    let lines = BufReader::new(file).lines();

    let lines_vec: Vec<String> = lines.map(|s| s.unwrap()).collect();

    let mut counter = 0;
    let mut draws: Vec<i32> = Vec::new();
    let mut current_board: Board = Board {
        numbers: Vec::new(),
        marks: Vec::new(),
    };
    let mut boards: Vec<Board> = Vec::new();

    for line in lines_vec.iter() {
        if counter == 0 {
            draws = line
                .split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
        } else if !(line.trim().is_empty()) {
            let row = line
                .split(" ")
                .filter(|str| str.trim().is_empty() == false)
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            println!("processing row: {:?}", row);
            current_board.push_row(row);

            if current_board.numbers.len() >= BOARD_SIZE {
                boards.push(current_board);
                current_board = Board {
                    numbers: Vec::new(),
                    marks: Vec::new(),
                };
            }
        }

        println!("{} {}", line, counter);
        counter += 1;
    }

    println!("draws: {:?}", draws);

    for board in boards.iter() {
        dbg!(&board);
    }

    Ok(())
}
