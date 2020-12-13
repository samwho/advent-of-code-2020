use std::{
    collections::BTreeSet, error::Error, fs::File, io::BufRead, io::BufReader, str::FromStr,
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/5")?;
    let reader = BufReader::new(file);

    let mut ids: BTreeSet<isize> = BTreeSet::new();
    for result in reader.lines() {
        let line = result?;

        let value_str: String = line
            .chars()
            .map(|c| if c == 'F' || c == 'L' { '0' } else { '1' })
            .collect();

        let value = isize::from_str_radix(&value_str, 2)?;
        ids.insert(value);
    }

    let mut prev: Option<isize> = None;
    for id in ids {
        if prev.is_none() {
            prev = Some(id);
            continue;
        }

        if id - prev.unwrap() == 2 {
            println!("id: {}, prev: {}", id, prev.unwrap());
            break;
        }
        prev = Some(id);
    }

    Ok(())
}
