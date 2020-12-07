use anyhow::{bail, Context};
use itertools::{self, Itertools};
use std::collections::HashMap;
use std::fs;

fn main() -> anyhow::Result<()> {
    let f = fs::read_to_string("input")?;

    let mut lines = f.lines();

    // Key: color, value: Vec<(color, count)>
    let mut rules = HashMap::new();

    while let Some(line) = lines.next() {
        let mut words = line.split_whitespace();

        let outer_color = words.by_ref().take(2).join(" ");

        // Skip "bags", "contain"
        let mut words = words.skip(2);

        loop {
            match words.next().unwrap() {
                // ... "no other bags."
                "no" => break,
                // <number> <color> bags, <...
                number => {
                    let count = number.parse::<i64>().unwrap();

                    let inner_color = words.by_ref().take(2).join(" ");

                    let rule = rules.entry(outer_color.clone()).or_insert(vec![]);
                    rule.push((inner_color, count));

                    match words.next().unwrap() {
                        // there is more
                        "bags," | "bag," => continue,
                        // there is no more
                        "bag." | "bags." => break,
                        word => panic!("impossible: {}", word),
                    }
                }
            }
        }
    }

    // Visit all rules
    let mut shiny_count = 0;
    for (outer_color, _contents) in &rules {
        if can_contain_shiny(outer_color, &rules) {
            shiny_count += 1;
        }
    }

    println!("{}", shiny_count);

    Ok(())
}

fn can_contain_shiny(outer_color: &str, rules: &HashMap<String, Vec<(String, i64)>>) -> bool {
    match rules.get(outer_color) {
        Some(contents) if contents.iter().any(|(color, _count)| color == "shiny gold") => true,
        Some(contents) => {
            for (color, _count) in contents {
                if can_contain_shiny(color, &rules) {
                    return true;
                }
            }
            false
        }
        // Bags with no contents cannot contain a shiny gold
        None => false,
    }
}
