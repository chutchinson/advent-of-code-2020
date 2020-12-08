#[macro_use] extern crate lazy_static;

mod prelude;
use prelude::*;
use std::collections::HashMap;
use regex::Regex;

lazy_static! {
    static ref HGT: Regex = Regex::new(r#"^(\d+)(cm|in)$"#).unwrap();
    static ref HCL: Regex = Regex::new(r#"^#[0-9a-f]{6}$"#).unwrap();
    static ref PID: Regex = Regex::new(r#"^[0-9]{9}$"#).unwrap();
}

fn validate_hgt(value: &str) -> Option<bool> {
    let captures = HGT.captures_iter(value).next()?;
    let value = captures[1].parse::<usize>().ok()?;
    let unit = &captures[2];
    let result =
        (unit == "cm" && value >= 150 && value <= 193) ||
        (unit == "in" && value >= 59 && value <= 76);
    Some(result)
}

fn main() {
    let mut data = String::new();
    let _ = stdin().lock().read_to_string(&mut data).expect("invalid input");
    let records = data
        .split("\n\n")
        .map(|batch| batch.replace("\n", " "))
        .map(|batch|  {
            batch.split(" ")
                .map(|x| x.split(":").map(|x| x.to_owned()).collect_tuple())
                .flatten()
                .collect::<HashMap<_, _>>()
        })
        .collect_vec();

    let count = records.iter()
        .filter(|record| ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().all(|x| record.contains_key(*x)))
        .count();

    println!("{}", count);

    let count = records.iter()
        .map(|record| {
            let mut valid = true;
            valid &= record.get("byr")?.parse::<isize>().ok().map(|v| v >= 1920 && v <= 2002)?;
            valid &= record.get("iyr")?.parse::<isize>().ok().map(|v| v >= 2010 && v <= 2020)?;
            valid &= record.get("eyr")?.parse::<isize>().ok().map(|v| v >= 2020 && v <= 2030)?;
            valid &= record.get("hgt").and_then(|v| validate_hgt(v))?;
            valid &= record.get("hcl").map(|v| HCL.is_match(v))?;
            valid &= record.get("pid").map(|v| PID.is_match(v))?;
            valid &= record.get("ecl").map(|v| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v.as_str()))?;
            Some(valid)
        })
        .filter(|x| *x == Some(true))
        .count();

    println!("{}", count);
}