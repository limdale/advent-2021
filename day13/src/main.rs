use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let lines = BufReader::new(file).lines();

    let lines_vec: Vec<String> = lines.map(|s| s.unwrap()).collect();

    let mut dots: HashSet<(i32, i32)> = HashSet::new();
    let mut folds: Vec<(char, i32)> = Vec::new();

    for line in lines_vec {
        if line.trim().is_empty() {
            continue;
        }
        if !line.contains("fold") {
            let point: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            dots.insert((point[0], point[1]));
        } else {
            let fold: Vec<&str> = line.split("=").collect();
            folds.push((
                fold[0].chars().last().unwrap(),
                fold[1].parse::<i32>().unwrap(),
            ));
        }
    }

    //println!("dots: {:?}", dots);

    for fold in folds.iter() {
        let mut to_add: HashSet<(i32, i32)> = HashSet::new();

        // fold y
        if fold.0 == 'y' {
            for dot in dots.iter() {
                if dot.1 >= fold.1 {
                    let new_dot = (dot.0, 2 * fold.1 - dot.1);
                    to_add.insert(new_dot);

                    //println!("{:?} -> {:?}", dot, new_dot);
                }
            }
            dots.retain(|dot| dot.1 < fold.1);
        }

        // fold x
        if fold.0 == 'x' {
            for dot in dots.iter() {
                if dot.0 >= fold.1 {
                    let new_dot = (2 * fold.1 - dot.0, dot.1);
                    to_add.insert(new_dot);

                    //println!("{:?} -> {:?}", dot, new_dot);
                }
            }
            dots.retain(|dot| dot.0 < fold.1);
        }

        dots.extend(to_add);

        //println!("dots after fold {:?}: {:?}   count: {}", fold, dots, dots.len());
        println!("count: {}", dots.len());
    }

    let mut max_x = 0;
    let mut max_y = 0;

    for dot in dots.iter() {
        if max_x < dot.0 {
            max_x = dot.0;
        }
        if max_y < dot.1 {
            max_y = dot.1;
        }
    }

    for y in 0..max_y + 1 {
        let mut print_str = String::new();
        for x in 0..max_x + 1 {
            if dots.contains(&(x, y)) {
                print_str.push_str("#");
            } else {
                print_str += ".";
            }
        }
        println!("{}", print_str);
    }

    Ok(())
}
