use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::BinaryHeap;

fn main() -> std::io::Result<()> {
    let file = File::open("test-input")?;
    let lines = BufReader::new(file).lines();

    let lines_vec: Vec<String> = lines.map(|s| s.unwrap()).collect();

    let mut maze: Vec<Vec<usize>> = Vec::new();

    for line in lines_vec {
        let mut row: Vec<usize> = Vec::new();
        for c in line.chars() {
            let num = c.to_string().parse::<usize>().unwrap();
            row.push(num);
        }
        maze.push(row);
    }

    // Dijkstras
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut to_visit: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    let mut visited: Vec<(usize, usize)> = Vec::new();

    for y in 0..maze.len() {
        for x in 0..maze[y].len() {
            if x == 0 && y == 0 {
                distances.insert((x, y), maze[y][x]);
            } else {
                distances.insert((x, y), usize::MAX);
            }
        }
    }

    to_visit.push((0, 0));

    while !to_visit.is_empty() {
        let (x, y) = to_visit.pop().unwrap();
        let current_distance = distances[&(x, y)];

        let mut adj: Vec<(usize, usize)> = Vec::new();

        if x > 0 {
            adj.push((x-1, y));   
        }

        if x < maze[0].len() - 1 {
            adj.push((x+1, y));   
        }

        if y > 0 {
            adj.push((x, y-1));
        }

        if y < maze.len() - 1 {
            adj.push((x, y+1));   
        }

        for (adj_x, adj_y) in adj.iter() {
            if !visited.contains(&(*adj_x, *adj_y)) {
                let new_dist = current_distance + maze[*adj_y][*adj_x];
                if new_dist < distances[&(*adj_x, *adj_y)] {
                    distances.remove_entry(&(*adj_x, *adj_y));
                    distances.insert((*adj_x, *adj_y), new_dist);
                }
                to_visit.push((*adj_x, *adj_y));
            }
        }

        visited.push((x, y));

        println!("to_visit: {:?}", to_visit);
        println!("visited: {:?}", visited);
        println!("distances: {:?}", distances);
    }

    println!("{:?}", maze);

    Ok(())
}
