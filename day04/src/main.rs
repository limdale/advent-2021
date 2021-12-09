use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const BOARD_SIZE: i32 = 5;

#[derive(Debug)]
struct Board {
    numbers: Vec<Vec<u32>>,
    marks: Vec<Vec<bool>>
}

impl Board {
    fn push_row(&mut self, row: Vec<u32>) {
        self.numbers.push(row);
    }
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let mut board = Board {
        numbers: Vec::new(),
        marks: Vec::new(),
    };

    board.push_row(vec![1, 2, 3, 4, 5]);

    dbg!(&board);

    Ok(())
}
