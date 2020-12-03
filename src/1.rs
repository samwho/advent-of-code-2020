use std::{error::Error, fs::File, io::BufRead, io::BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/1")?;
    let reader = BufReader::new(file);

    let mut numbers = Vec::new();

    for result in reader.lines() {
        let line = result?;
        let number: i32 = line.parse()?;
        numbers.push(number);
    }

    for i in numbers.iter() {
        for j in numbers.iter() {
            for k in numbers.iter() {
                if i + j + k == 2020 {
                    println!("{}", i * j * k);
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}
