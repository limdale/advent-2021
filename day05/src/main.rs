use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn is_diagonal(start: (i32, i32), end: (i32, i32)) -> bool {
    return (start.0 - end.0).abs() == (start.1 - end.1).abs();
}

fn create_line(start: (i32, i32), end: (i32, i32)) -> Vec<(i32, i32)> {
    let mut line: Vec<(i32, i32)> = Vec::new();
    if start.0 == end.0 {
        let mut y = start.1;
        let direction = (end.1 - start.1).signum();
        loop {
            line.push((start.0, y));
            y += direction;
            if y == end.1 {
                line.push((start.0, y));
                break;
            }
        }
    } else if start.1 == end.1 {
        let mut x = start.0;
        let direction = (end.0 - start.0).signum();
        loop {
            line.push((x, start.1));
            x += direction;
            if x == end.0 {
                line.push((x, start.1));
                break;
            }
        }
    } else if is_diagonal(start, end) {
        let mut x = start.0;
        let mut y = start.1;

        let x_direction = (end.0 - start.0).signum();
        let y_direction = (end.1 - start.1).signum();
        loop {
            line.push((x, y));
            x += x_direction;
            y += y_direction;
            if x == end.0 || y == end.1 {
                line.push((x, y));
                break;
            }
        }
    }

    return line;
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let lines = BufReader::new(file).lines();

    let lines_vec: Vec<String> = lines.map(|s| s.unwrap()).collect();

    let mut point_counts: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines_vec.iter() {
        let points = line
            .split("->")
            .map(|point| point.trim())
            .collect::<Vec<&str>>();

        let mut start_split = points.get(0).unwrap().split(",");
        let mut end_split = points.get(1).unwrap().split(",");

        let start = (
            start_split.next().unwrap().parse::<i32>().unwrap(),
            start_split.next().unwrap().parse::<i32>().unwrap(),
        );
        let end = (
            end_split.next().unwrap().parse::<i32>().unwrap(),
            end_split.next().unwrap().parse::<i32>().unwrap(),
        );

        let points_vec = create_line(start, end);

        for (x, y) in points_vec {
            *point_counts.entry((x, y)).or_insert(0) += 1;
        }

        //println!("{},{} -> {},{}", start.0, start.1, end.0, end.1);
        //println!("{:?}", create_line(start, end));
    }

    //println!("{:?}", point_counts);

    let mut overlaps = 0;

    for (_, value) in point_counts {
        if value > 1 {
            overlaps += 1;
        }
    }

    println!("point overlaps: {}", overlaps);

    Ok(())
}
