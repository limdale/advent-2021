use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("test-input")?;
    let lines = BufReader::new(file).lines();

    let lines_vec: Vec<String> = lines.map(|s| s.unwrap()).collect();

    let mut fishes = lines_vec.get(0).unwrap().split(",").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    println!("Initial state: {:?}", fishes);

    let days = 80;

    for day in 0..days {
        for i in 0..fishes.len() {
            let mut to_add = 0;
            fishes[i] = fishes[i] - 1;
            if fishes[i] < 0 {
                fishes[i] = 6;
                to_add += 1;
            }

            for _ in 0..to_add {
                fishes.push(8)
            }
        }

        println!("After {} day(s): {:?}", day+1, fishes);
        println!("Day {} - Fish count: {}", day+1, fishes.len());
    }
    
    println!("Day {} - Fish count: {}", days, fishes.len());

    Ok(())
}
