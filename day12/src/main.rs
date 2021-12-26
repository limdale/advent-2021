use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn traverse(
    node: &String,
    adj_list: &HashMap<String, Vec<String>>,
    visited: &HashSet<&String>,
    path: &Vec<&String>,
) -> i32 {
    let mut new_vec = Vec::new();
    new_vec.extend(path);
    new_vec.push(node);

    let mut new_visited = HashSet::new();
    new_visited.extend(visited);
    new_visited.insert(node);

    if node == "end" {
        println!("Final path: {:?}", new_vec);
        return 1;
    }

    if !adj_list.contains_key(node) {
        println!("Final path: {:?}", new_vec);
        return 1;
    }

    let edges = &adj_list[node];
    //println!("node: {}, edges to visit: {:?}, visited: {:?}, path: {:?}", node, edges, visited, path);

    let mut count = 0;
    for next in edges {
        let is_uppercase = &next.to_uppercase() == next;
        if next != "start" && (is_uppercase || (!is_uppercase && !visited.contains(next))) {
            count += traverse(next, adj_list, &new_visited, &new_vec);
        }
    }

    return count;
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let lines = BufReader::new(file).lines();

    let lines_vec: Vec<String> = lines.map(|s| s.unwrap()).collect();

    let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines_vec {
        let nodes: Vec<&str> = line.split("-").collect();
        adj_list
            .entry(nodes[0].to_string())
            .or_insert(Vec::new())
            .push(nodes[1].to_string());

        adj_list
            .entry(nodes[1].to_string())
            .or_insert(Vec::new())
            .push(nodes[0].to_string());
        println!("{} - {}", nodes[0], nodes[1]);
    }

    let count = traverse(
        &"start".to_string(),
        &adj_list,
        &HashSet::new(),
        &Vec::new(),
    );

    println!("{:?}", adj_list);
    println!("paths: {}", count);

    Ok(())
}
