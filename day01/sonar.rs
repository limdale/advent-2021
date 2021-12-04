use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let lines = BufReader::new(file).lines();

    let mut increase_count = 0;
    let mut prev = -1;

    for line in lines {
        let num: i32 = line.unwrap().parse().unwrap();

        println!("comparing {} {}", prev, num);

        if prev != -1 && num > prev {
            increase_count += 1;
        }

        prev = num;
    }

    println!("answer: {}", increase_count);

    Ok(())
}
