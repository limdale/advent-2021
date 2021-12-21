use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn sort_string(s: &str) -> String {
    let word = s.to_string();
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    return chars.iter().collect();
}

fn is_subset(big: &String, small: &String) -> bool {
    for c in small.chars() {
        if !big.contains(c) {
            return false;
        }
    }
    return true;
}

// returns the number of chars in small that are missing from big
fn missing_chars(big: &String, small: &String) -> i32 {
    let mut count = 0;
    for c in small.chars() {
        if !big.contains(c) {
            count += 1;
        }
    }
    return count;
}

fn merge_digits(num_vec: &Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut exp = num_vec.len() as u32;
    for x in num_vec {
        println!("{} {}", x, exp);
        exp -= 1;
        sum += x * i32::pow(10, exp);
    }

    return sum;
}

fn get_mapping(signals: Vec<String>) -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();

    // get first 4 unique ones
    let one = signals.iter().find(|s| s.len() == 2).unwrap().to_string();
    let seven = signals.iter().find(|s| s.len() == 3).unwrap().to_string();
    let four = signals.iter().find(|s| s.len() == 4).unwrap().to_string();
    let eight = signals.iter().find(|s| s.len() == 7).unwrap().to_string();

    // get 6
    let six = signals
        .iter()
        .find(|s| s.len() == 6 && !is_subset(&s, &one))
        .unwrap()
        .to_string();

    // get 9
    let nine = signals
        .iter()
        .find(|s| s.len() == 6 && is_subset(&s, &four))
        .unwrap()
        .to_string();

    // get 0
    let zero = signals
        .iter()
        .find(|s| s.len() == 6 && *s != &six && *s != &nine)
        .unwrap()
        .to_string();

    // get 3
    let three = signals
        .iter()
        .find(|s| s.len() == 5 && is_subset(&s, &seven))
        .unwrap()
        .to_string();

    // get 2
    let two = signals
        .iter()
        .find(|s| s.len() == 5 && missing_chars(s, &four) == 2)
        .unwrap()
        .to_string();

    // get 5
    let five = signals
        .iter()
        .find(|s| s.len() == 5 && *s != &three && *s != &two)
        .unwrap()
        .to_string();

    //println!("{} {} {} {} {} {} {} {} {} {}", zero, one, two, three, four, five, six, seven, eight, nine);

    map.insert(zero, 0);
    map.insert(one, 1);
    map.insert(two, 2);
    map.insert(three, 3);
    map.insert(four, 4);
    map.insert(five, 5);
    map.insert(six, 6);
    map.insert(seven, 7);
    map.insert(eight, 8);
    map.insert(nine, 9);

    return map;
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let lines = BufReader::new(file).lines();

    let lines_vec: Vec<String> = lines.map(|s| s.unwrap()).collect();

    let mut unique_count = 0;
    let mut sum = 0;

    for line in lines_vec {
        let mut input = line.split("|");
        let signals = input
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|s| sort_string(s))
            .collect::<Vec<String>>();

        let outputs = input
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|s| sort_string(s))
            .collect::<Vec<String>>();

        let count: usize = outputs
            .iter()
            .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
            .count();

        unique_count += count;

        println!("{:?} {:?}", signals, outputs);

        let mapping = get_mapping(signals);
        println!("mapping: {:?}", mapping);

        let output_numbers = outputs
            .iter()
            .map(|num_str| mapping[num_str])
            .collect::<Vec<i32>>();

        let merged = merge_digits(&output_numbers);

        println!("output: {:?} {:?}", output_numbers, merged);

        sum += merged;
    }

    println!("Unique count: {}", unique_count);
    println!("Sum: {}", sum);

    Ok(())
}
