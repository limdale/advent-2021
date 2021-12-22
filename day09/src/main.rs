use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let lines = BufReader::new(file).lines();

    let lines_vec: Vec<String> = lines.map(|s| s.unwrap()).collect();

    let mut heightmap: Vec<Vec<i32>> = Vec::new();

    for line in lines_vec {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            let num = c.to_string().parse::<i32>().unwrap();
            row.push(num);
        }
        heightmap.push(row);
    }

    let mut sum = 0;

    for i in 0..heightmap.len() {
        for j in 0..heightmap[i].len() {
            let top = if i > 0 { heightmap[i - 1][j] } else { 10 };

            let bottom = if i < heightmap.len() - 1 {
                heightmap[i + 1][j]
            } else {
                10
            };

            let left = if j > 0 { heightmap[i][j - 1] } else { 10 };

            let right = if j < heightmap[i].len() - 1 {
                heightmap[i][j + 1]
            } else {
                10
            };

            if heightmap[i][j] < top
                && heightmap[i][j] < bottom
                && heightmap[i][j] < left
                && heightmap[i][j] < right
            {
                //println!(
                //    "Low point: {} x: {}  y: {}  left: {}  right: {}  top: {}  bottom: {}",
                //    heightmap[i][j], i, j, left, right, top, bottom
                //);
                sum += heightmap[i][j] + 1;
            }
        }
    }

    //println!("{:?}", heightmap);
    println!("Sum: {}", sum);

    Ok(())
}
