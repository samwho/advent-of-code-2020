use std::{
    collections::{BTreeSet, HashMap, HashSet},
    error::Error,
    fs::File,
    io::BufRead,
    io::BufReader,
};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let r = Regex::new(r"^(.+?) bags contain (\d+ .+? bags(?:, )?)+\.$")?;

    let file = File::open("data/7")?;
    let reader = BufReader::new(file);
    let mut graph: HashMap<String, Vec<(usize, String)>> = HashMap::new();

    for result in reader.lines() {
        let line = result?;

        let mut i = line.split(" bags contain ");
        let bag_color = i.next().unwrap();
        let can_contain = i.next().unwrap();

        let v = graph.entry(bag_color.to_owned()).or_default();

        for contain in can_contain.split(", ") {
            if contain != "no other bags." {
                let mut iter = contain.split_whitespace();
                let num_str = iter.next().unwrap();
                let num: usize = num_str.parse()?;
                let color = format!("{} {}", iter.next().unwrap(), iter.next().unwrap());
                v.push((num, color));
            }
        }
    }

    let mut result: BTreeSet<String> = BTreeSet::new();
    for (color, _) in graph.iter() {
        if can_contain_bag(&graph, color, "shiny gold") {
            result.insert(color.to_owned());
        }
    }

    println!("can contain shiny gold: {}", result.len());
    println!(
        "shiny gold can contain: {}",
        count_total_bags(&graph, "shiny gold")
    );

    Ok(())
}

fn can_contain_bag(
    graph: &HashMap<String, Vec<(usize, String)>>,
    current_color: &str,
    candidate_color: &str,
) -> bool {
    if current_color == candidate_color {
        return true;
    }

    match graph.get(current_color) {
        Some(children) => {
            for (_, child_color) in children {
                if can_contain_bag(graph, child_color, candidate_color) {
                    return true;
                }
            }
            false
        }
        None => false,
    }
}

fn count_total_bags(graph: &HashMap<String, Vec<(usize, String)>>, color: &str) -> usize {
    match graph.get(color) {
        Some(children) => {
            if children.is_empty() {
                return 1;
            }
            let mut sum = 1;
            for (num, child_color) in children {
                sum += num * count_total_bags(graph, child_color);
            }
            sum
        }
        None => 0,
    }
}
