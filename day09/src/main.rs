use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_basin_size(heightmap: &Vec<Vec<usize>>, low_point: (usize, usize)) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut to_visit: Vec<(usize, usize)> = Vec::new();
    to_visit.push(low_point);

    while !to_visit.is_empty() {
        let (x, y) = to_visit.pop().unwrap();

        if !visited.contains(&(x, y)) {
            visited.insert((x, y));
        } else {
            continue;
        }

        if y > 0 && heightmap[(y - 1)][x] < 9 {
            to_visit.push((x, y - 1));
        }

        if y < heightmap.len() - 1 && heightmap[y + 1][x] < 9 {
            to_visit.push((x, y + 1));
        }

        if x > 0 && heightmap[y][x - 1] < 9 {
            to_visit.push((x - 1, y));
        }

        if x < heightmap[y].len() - 1 && heightmap[y][x + 1] < 9 {
            to_visit.push((x + 1, y));
        }
    }

    return visited.len();
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let lines = BufReader::new(file).lines();

    let lines_vec: Vec<String> = lines.map(|s| s.unwrap()).collect();

    let mut heightmap: Vec<Vec<usize>> = Vec::new();

    for line in lines_vec {
        let mut row: Vec<usize> = Vec::new();
        for c in line.chars() {
            let num = c.to_string().parse::<usize>().unwrap();
            row.push(num);
        }
        heightmap.push(row);
    }

    let mut sum = 0;
    let mut low_points: Vec<(usize, usize)> = Vec::new();

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
                low_points.push((j as usize, i as usize));
            }
        }
    }

    println!("Sum: {}", sum);
    //println!("Low points: {:?}", low_points);

    let mut basin_sizes: Vec<usize> = Vec::new();

    for low_point in low_points {
        let basin_size = get_basin_size(&heightmap, low_point);
        //println!(
        //    "Basin size for low point ({}, {}): {}",
        //    low_point.0, low_point.1, basin_size
        //);
        basin_sizes.push(basin_size);
    }

    basin_sizes.sort();

    let length = basin_sizes.len();
    println!("Answer: {}", basin_sizes[length - 1] * basin_sizes[length - 2] * basin_sizes[length - 3]);

    Ok(())
}
