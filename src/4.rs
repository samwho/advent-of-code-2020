use lazy_static::lazy_static;
use regex::Regex;
use std::{error::Error, fs::File, io::BufRead, io::BufReader, str::FromStr};

lazy_static! {
    static ref HEIGHT_REGEX: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    static ref COLOR_REGEX: Regex = Regex::new(r"^#([0-9a-f]{6})$").unwrap();
    static ref NINE_DIGIT_REGEX: Regex = Regex::new(r"^([0-9]{9})$").unwrap();
    static ref VALID_EYE_COLORS: &'static [&'static str] =
        &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
}

#[derive(Debug)]
enum Height {
    Centimeters(i32),
    Inches(i32),
}

#[derive(Default, Debug)]
struct Passport {
    byr: Option<i32>,
    iyr: Option<i32>,
    eyr: Option<i32>,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl FromStr for Passport {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport = Passport {
            ..Default::default()
        };

        for p in s.split_whitespace() {
            let s: Vec<&str> = p.split(":").collect();
            if s.len() != 2 {
                return Err("invalid passport format".into());
            }

            let key = s.get(0).unwrap().to_owned();
            let value = s.get(1).unwrap().to_string();

            match key {
                "byr" => {
                    let byr: i32 = value.parse()?;
                    if byr < 1920 || byr > 2002 {
                        return Err(format!("invalid byr value: {}", byr).into());
                    }
                    passport.byr = Some(byr);
                }
                "iyr" => {
                    let iyr: i32 = value.parse()?;
                    if iyr < 2010 || iyr > 2020 {
                        return Err(format!("invalid iyr value: {}", iyr).into());
                    }
                    passport.iyr = Some(iyr);
                }
                "eyr" => {
                    let eyr: i32 = value.parse()?;
                    if eyr < 2020 || eyr > 2030 {
                        return Err(format!("invalid eyr value: {}", eyr).into());
                    }
                    passport.eyr = Some(eyr);
                }
                "hgt" => match HEIGHT_REGEX.captures(&value) {
                    Some(captures) => {
                        let height: i32 = captures.get(1).unwrap().as_str().parse()?;
                        let unit = captures.get(2).unwrap().as_str();

                        let value = match unit {
                            "cm" => {
                                if height < 150 || height > 193 {
                                    return Err(
                                        format!("invalid height, out of range: {}", value).into()
                                    );
                                }
                                Height::Centimeters(height)
                            }
                            "in" => {
                                if height < 59 || height > 76 {
                                    return Err(
                                        format!("invalid height, out of range: {}", value).into()
                                    );
                                }
                                Height::Inches(height)
                            }
                            _ => return Err(format!("invalid height unit: {}", unit).into()),
                        };
                        passport.hgt = Some(value);
                    }
                    None => return Err(format!("invalid height: {}", value).into()),
                },
                "hcl" => match COLOR_REGEX.captures(&value) {
                    Some(_) => {
                        passport.hcl = Some(value);
                    }
                    None => return Err(format!("invalid hcl: {}", value).into()),
                },
                "ecl" => {
                    if !VALID_EYE_COLORS.contains(&value.as_str()) {
                        return Err(format!("invalid ecl: {}", value).into());
                    }
                    passport.ecl = Some(value);
                }
                "pid" => match NINE_DIGIT_REGEX.captures(&value) {
                    Some(_) => {
                        passport.pid = Some(value);
                    }
                    None => return Err(format!("invalid pid: {}", value).into()),
                },
                "cid" => passport.cid = Some(value),
                _ => return Err(format!("invalid key: {}", key).into()),
            }
        }

        let all_fields_present = passport.byr.is_some()
            && passport.iyr.is_some()
            && passport.eyr.is_some()
            && passport.hgt.is_some()
            && passport.hcl.is_some()
            && passport.ecl.is_some()
            && passport.pid.is_some();

        if !all_fields_present {
            return Err("invalid passport, missing fields".into());
        }

        Ok(passport)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/4")?;
    let reader = BufReader::new(file);

    let mut num_valid = 0;
    let mut current = String::new();

    for result in reader.lines() {
        let line = result?;

        current.push_str(" ");
        current.push_str(&line);

        if line.is_empty() {
            match current.parse::<Passport>() {
                Ok(_) => {
                    num_valid += 1;
                }
                Err(_) => {}
            }
            current.clear();
        }
    }

    if !current.is_empty() {
        match current.parse::<Passport>() {
            Ok(_) => {
                num_valid += 1;
            }
            Err(_) => {}
        }
    }

    println!("{}", num_valid);

    Ok(())
}
