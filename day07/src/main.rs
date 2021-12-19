use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_fuel_consumption(crabs: &Vec<i32>, pos: i32) -> i32 {
    let mut fuel = 0;
    for crab in crabs {
        fuel += (crab - pos).abs();
    }
    return fuel;
}

fn get_fuel_consumption_2(crabs: &Vec<i32>, pos: i32) -> i32 {
    let mut fuel = 0;
    for crab in crabs {
        let diff = (crab - pos).abs();
        let fuel_consumption = diff * (diff + 1) / 2;
        fuel += fuel_consumption;
        //println!("{} - {} = {} {}", crab, pos, diff, fuel_consumption);
    }
    return fuel;
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let lines = BufReader::new(file).lines();

    let lines_vec: Vec<String> = lines.map(|s| s.unwrap()).collect();

    let crabs = lines_vec
        .get(0)
        .unwrap()
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();

    let mut min_fuel = i32::MAX;

    // was not sure if numbers not in the list are valid, just include them to be sure
    for i in *min..=*max {
        let fuel = get_fuel_consumption_2(&crabs, i);
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }

    println!("Min fuel: {}", min_fuel);

    Ok(())
}
