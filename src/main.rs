use itertools::{self, Itertools};
use std::collections::HashMap;
use std::fs;

fn main() -> anyhow::Result<()> {
    let f = fs::read_to_string("input")?;

    let mut lines = f.lines();

    // Key: outer color, Value: Vec<(inner color, count)>
    let mut rules = HashMap::new();

    // Parse input into rules like "outer color can contain [N inner color..]"
    while let Some(line) = lines.next() {
        let mut words = line.split_whitespace();

        // Take the first two words and join them into the outer color
        let outer_color = words.by_ref().take(2).join(" ");

        // Skip the next two words: "bags", "contain"
        let mut words = words.skip(2);

        loop {
            match words.next().unwrap() {
                // ... "no other bags."
                "no" => break,
                // <number> <color> bags, <...
                number => {
                    let count = number.parse::<i64>().unwrap();

                    // Take the next to words and join them into the inner color
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

    // Figure out how many bags there are in the shiny gold bag
    let bag_count = count_bags("shiny gold", &rules);
    println!("{}", bag_count);

    Ok(())
}

/// Counts how many bags there are in the given bag
fn count_bags(outer_color: &str, rules: &HashMap<String, Vec<(String, i64)>>) -> i64 {
    match rules.get(outer_color) {
        None => 0,
        Some(contents) => {
            let mut this_count = 0;
            for (color, inner_count) in contents {
                this_count += inner_count;
                this_count += inner_count * count_bags(color, &rules);
            }
            this_count
        }
    }
}
