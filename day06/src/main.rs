use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn generated_fishes(
    starting_fish: i32,
    days: i32,
    cache: &mut HashMap<(i32, i32), usize>,
) -> usize {
    if starting_fish >= days {
        return 0;
    }
    if cache.contains_key(&(starting_fish, days)) {
        return cache[&(starting_fish, days)];
    }

    let first = generated_fishes(6, days - starting_fish - 1, cache);
    let second = generated_fishes(8, days - starting_fish - 1, cache);

    let fishes = 1 + first + second;
    cache.insert((starting_fish, days), fishes);
    return fishes;
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let lines = BufReader::new(file).lines();

    let lines_vec: Vec<String> = lines.map(|s| s.unwrap()).collect();

    let fishes = lines_vec
        .get(0)
        .unwrap()
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("Initial state: {:?}", fishes);

    let days = 256;
    let mut fish_count = 0;

    let mut count_map: HashMap<(i32, i32), usize> = HashMap::new();

    for fish in fishes {
        fish_count += generated_fishes(fish, days, &mut count_map) + 1;
    }

    println!("Day {} - Fish count: {}", days, fish_count);

    Ok(())
}
