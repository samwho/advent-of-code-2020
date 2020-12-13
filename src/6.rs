use std::{collections::HashMap, error::Error, fs::File, io::BufRead, io::BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/6")?;
    let reader = BufReader::new(file);

    let mut group: HashMap<char, usize> = HashMap::new();
    let mut group_size = 0;
    let mut total = 0;
    for result in reader.lines() {
        let line = result?;

        if line.is_empty() {
            for (_, v) in group.iter() {
                if *v == group_size {
                    total += 1;
                }
            }
            group_size = 0;
            group.clear();
        } else {
            group_size += 1;
            for c in line.chars() {
                let val = group.get(&c).unwrap_or(&0) + 1;
                group.insert(c, val);
            }
        }
    }

    println!("{}", total);

    Ok(())
}
