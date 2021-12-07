use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let lines = BufReader::new(file).lines();

    let lines_vec = lines.map(|s| s.unwrap()).collect();

    let (gamma_rate, ep_rate) = get_gamma_ep_rate(&lines_vec);

    let gamma = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let ep = isize::from_str_radix(&ep_rate, 2).unwrap();

    println!(
        "lines:{}  gamma bits: {}  gamma_rate: {}  epsilon_rate: {}  ans: {}",
        lines_vec.len(),
        gamma_rate,
        gamma,
        ep,
        gamma * ep
    );

    let mut o2_rate = 0;
    let mut co2_rate = 0;

    let mut o2_set = lines_vec.clone();
    for i in 0..lines_vec[0].len() {
        // very unoptimized since there is no need to check the whole string, but
        // just use this to reuse getting the gamma/ep from the first prob.
        let (o2_gamma, _) = get_gamma_ep_rate(&o2_set);
        let bit = o2_gamma.chars().nth(i).unwrap();

        o2_set = o2_set
            .into_iter()
            .filter(|binary_string| binary_string.chars().nth(i).unwrap() == bit)
            .collect();

        if o2_set.len() == 1 {
            println!("final o2: {}", o2_set[0]);
            o2_rate = isize::from_str_radix(&o2_set[0], 2).unwrap();
            break;
        }
    }

    let mut co2_set = lines_vec.clone();
    for i in 0..lines_vec[0].len() {
        let (_, co2_ep) = get_gamma_ep_rate(&co2_set);
        let bit = co2_ep.chars().nth(i).unwrap();

        co2_set = co2_set
            .into_iter()
            .filter(|binary_string| binary_string.chars().nth(i).unwrap() == bit)
            .collect();

        if co2_set.len() == 1 {
            println!("final co2: {}", co2_set[0]);
            co2_rate = isize::from_str_radix(&co2_set[0], 2).unwrap();
            break;
        }
    }

    println!("final answer: {}", o2_rate * co2_rate);

    Ok(())
}

fn get_gamma_ep_rate(lines: &Vec<String>) -> (String, String) {
    let mut num_lines = 0;
    let mut zero_counts = Vec::new();

    for binary_string in lines.iter() {
        if zero_counts.is_empty() {
            for _i in 0..binary_string.len() {
                zero_counts.push(0)
            }

            println!("init: {} {}", zero_counts.len(), binary_string.len());
        }

        for i in 0..binary_string.len() {
            let bit = binary_string.chars().nth(i).unwrap();
            if bit == '0' {
                zero_counts[i] += 1
            }
        }

        num_lines += 1;
    }

    let mut gamma_rate = String::new();
    let mut ep_rate = String::new();

    for i in 0..zero_counts.len() {
        if zero_counts[i] > num_lines / 2 {
            gamma_rate.push('0');
            ep_rate.push('1');
        } else {
            gamma_rate.push('1');
            ep_rate.push('0');
        }
    }

    return (gamma_rate, ep_rate);
}
