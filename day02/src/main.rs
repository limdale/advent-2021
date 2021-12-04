use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

const FORWARD: &str = "forward";
const UP: &str = "up";
const DOWN: &str = "down";

fn main() -> std::io::Result<()> {
    second(String::from("input"))
}

fn first(filename: String) -> std::io::Result<()> {

    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    let mut position = 0;
    let mut depth = 0;

    for line in lines {
        let command = line.unwrap();
        let commandVec: Vec<&str> = command.split(" ").collect();

        let direction = commandVec[0];
        let moveBy: i32 = commandVec[1].parse().unwrap();

        if direction.eq(FORWARD) {
            position += moveBy
        } else if direction.eq(UP) {
            depth -= moveBy
        } else if direction.eq(DOWN) {
            depth += moveBy
        } else {
            println!("Invalid direction {}  moveBy {}", direction, moveBy)
        }

        println!("command: {}  position: {}  depth: {}", command, position, depth);
    }

    println!("answer: {} * {} = {}", position, depth, position * depth);

    Ok(())
}

fn second(filename: String) -> std::io::Result<()> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    let mut position = 0;
    let mut aim = 0;
    let mut depth = 0;

    for line in lines {
        let command = line.unwrap();
        let commandVec: Vec<&str> = command.split(" ").collect();

        let direction = commandVec[0];
        let moveBy: i32 = commandVec[1].parse().unwrap();

        if direction.eq(FORWARD) {
            position += moveBy;
            depth += moveBy * aim;
        } else if direction.eq(UP) {
            aim -= moveBy;
        } else if direction.eq(DOWN) {
            aim += moveBy;
        } else {
            println!("Invalid direction {}  moveBy {}", direction, moveBy)
        }

        println!("command: {}  position: {}  aim: {}  depth: {}", command, position, aim, depth);
    }

    println!("answer: {} * {} = {}", position, depth, position * depth);
    Ok(())
}

