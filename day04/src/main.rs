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
        self.marks.push(vec![false; BOARD_SIZE]);
    }

    fn mark_number(&mut self, number: i32) {
        for i in 0..self.numbers.len() {
            for j in 0..self.numbers[i].len() {
                if self.numbers[i][j] == number {
                    self.marks[i][j] = true;
                    return;
                }
            }
        }
    }

    fn is_bingo(&mut self) -> bool {
        // check each row
        'outer: for i in 0..self.marks.len() {
            let row = &self.marks[i];
            for j in 0..row.len() {
                if j == row.len() - 1 && row[j] {
                    return true;
                } else if row[j] == false {
                    continue 'outer;
                }
            }
        }

        // check each column
        'outer: for i in 0..self.marks.len() {
            for j in 0..BOARD_SIZE {
                if j == BOARD_SIZE - 1 && self.marks[j][i] {
                    return true;
                } else if self.marks[j][i] == false {
                    continue 'outer;    
                }
            }
        }

        return false;
    }

    fn compute_score(&mut self, final_number: i32) -> i32 {
        let mut sum = 0;
        for i in 0..self.marks.len() {
            let row = &self.marks[i];
            for j in 0..row.len() {
                if self.marks[i][j] == false {
                    sum += self.numbers[i][j];
                }
            }
        }

        return sum * final_number;
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
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

//    for board in boards.iter() {
//        dbg!(&board);
//    }

    // first answer
    'outer: for draw in draws {
        for board in boards.iter_mut() {
            board.mark_number(draw);
            if board.is_bingo() {
                println!("answer is: {}", board.compute_score(draw));
                break 'outer;
            }
        }
        println!("draw {}:  board: {:?}", draw, boards[0]);
    }

    Ok(())
}
