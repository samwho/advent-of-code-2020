use regex::Regex;
use std::{error::Error, fs::File, io::BufRead, io::BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let r = Regex::new(r"^(\d+)-(\d+) ([a-z]): (.+)$")?;

    let file = File::open("data/2")?;
    let reader = BufReader::new(file);

    let mut valid = 0;

    for result in reader.lines() {
        let line = result?;
        let captures = r.captures(&line).unwrap();
        let i1: usize = captures[1].parse()?;
        let i2: usize = captures[2].parse()?;
        let letter = captures[3].to_string().chars().next().unwrap();
        let password = captures[4].to_string();

        let first = password.chars().nth(i1 - 1).unwrap() == letter;
        let second = password.chars().nth(i2 - 1).unwrap() == letter;

        if first ^ second {
            valid += 1;
        }
    }

    println!("{}", valid);

    Ok(())
}
