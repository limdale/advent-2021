use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let lines = BufReader::new(file).lines();

    let lines_vec: Vec<String> = lines.map(|s| s.unwrap()).collect();

    let mut unique_count = 0;

    for line in lines_vec {
        let mut input = line.split("|");
        let signals = input
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .collect::<Vec<&str>>();

        let outputs = input
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .collect::<Vec<&str>>();

        let count: usize = outputs
            .iter()
            .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
            .count();

        unique_count += count;

        println!("{:?} {:?}", signals, outputs);
    }

    println!("Unique count: {}", unique_count);

    Ok(())
}
