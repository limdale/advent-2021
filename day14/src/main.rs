use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn insert_middle(src: String, to_insert: String) -> String {
    let first = &src[0..1];
    let last = &src[1..2];
    return format!("{}{}{}", first, to_insert, last);
}

fn generate_template(template: String, rules: &HashMap<String, String>) -> String {
    let mut template_vec: Vec<String> = Vec::new();
    for i in 1..template.len() {
        template_vec.push(rules[&template[i-1..=i]].to_string());
    }

    let mut new_template = String::new();
    for (i, s) in template_vec.iter().enumerate() {
        if i == 0 {
            new_template.push_str(s);
        } else {
            new_template.push_str(&s[1..s.len()]);
        }
    }

    return new_template;
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let lines = BufReader::new(file).lines();

    let lines_vec: Vec<String> = lines.map(|s| s.unwrap()).collect();

    let template = lines_vec[0].to_string();
    let mut rules: HashMap<String, String> = HashMap::new();
    for line in lines_vec.iter() {
        if line.trim().is_empty() {
            continue;
        }

        if !line.contains("->") {
            continue;
        }

        let rule = line
            .split("->")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        rules.insert(rule[0].to_string(), insert_middle(rule[0].to_string(), rule[1].to_string()));
    }

    let mut current_template = template;
    for i in 0..40 {
        current_template = generate_template(current_template, &rules);
        println!("step {}: {}", i+1, current_template.len());
    }

    let mut char_counts: HashMap<char, i32> = HashMap::new();
    for c in current_template.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }

    println!("{:?}", char_counts);

    let mut min = i32::MAX;
    let mut max = 0;

    for key in char_counts.keys() {
        if char_counts[key] > max {
            max = char_counts[key];
        }
        if char_counts[key] < min {
            min = char_counts[key];
        }
    }

    println!("max: {}, min: {}, diff: {}", max, min, max - min);

    Ok(())
}
