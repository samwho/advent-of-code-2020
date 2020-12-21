use std::{error::Error, fs::File, io::BufRead, io::BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/9")?;
    let reader = BufReader::new(file);

    let mut numbers: Vec<i64> = Vec::new();

    for result in reader.lines() {
        let line = result?;
        numbers.push(line.parse()?);
    }

    for i in 25..numbers.len() {
        let num = numbers.get(i).unwrap();
        if !is_sum_of_pair(*num, &numbers[i - 25..i]) {
            println!("{}", num);
            break;
        }
    }

    println!("weakness: {}", find_weakness(&numbers));

    Ok(())
}

fn is_sum_of_pair(num: i64, nums: &[i64]) -> bool {
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if num == nums.get(i).unwrap() + nums.get(j).unwrap() {
                return true;
            }
        }
    }

    false
}

fn find_weakness(nums: &[i64]) -> i64 {
    let target = 18272118;

    for i in 0..nums.len() {
        let mut total = 0;
        let mut smallest = 99999999;
        let mut largest = 0; // dirty hax
        for j in i..nums.len() {
            let num = *nums.get(j).unwrap();
            total += num;

            if num > largest {
                largest = num;
            }

            if num < smallest {
                smallest = num;
            }

            if total == target {
                return largest + smallest;
            }

            if total > target {
                break;
            }
        }
    }

    panic!("oh no");
}
