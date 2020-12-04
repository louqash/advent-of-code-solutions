use regex::Regex;
use std::collections::HashMap;

pub fn solve_v1() -> i32 {
    let data = super::load_file("day4.txt");
    let passports: Vec<String> = data.split("\n\n").map(|p| p.replace("\n", " ")).collect() ;

    let mut count = 0;
    for passport in passports {
        let num_of_fields = passport.split(" ")
            .map(|s| {
                let kv: Vec<&str> = s.split(":").collect();
                (kv[0], kv[1])
            })
            .filter(|kv| kv.0 != "cid")
            .count();
        if num_of_fields == 7 {
            count += 1;
        }
    }
    count
}

pub fn solve_v2() -> i32 {
    let data = super::load_file("day4.txt");
    let passports: Vec<String> = data.split("\n\n")
        .map(|p| p.replace("\n", " "))
        .collect() ;
    let regexes: HashMap<&str, Regex> = vec![
        ("byr", Regex::new(r"^19[2-9][0-9]|200[0-2]$").unwrap()),
        ("iyr", Regex::new(r"^201[0-9]|2020$").unwrap()),
        ("eyr", Regex::new(r"^202[0-9]|2030$").unwrap()),
        ("hgt", Regex::new(r"^(1([5-8][0-9]|9[0-3])cm)|((59|6[0-9]|7[0-6])in)$").unwrap()),
        ("hcl", Regex::new(r"^#[0-9a-f]{6}$").unwrap()),
        ("ecl", Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap()),
        ("pid", Regex::new(r"^[0-9]{9}$").unwrap()),
    ].into_iter().collect();

    let mut count = 0;
    for passport in passports {
        let fields: Vec<(&str, &str)> = passport.split(" ")
            .map(|s| {
                let kv: Vec<&str> = s.split(":").collect();
                (kv[0], kv[1])
            }).filter(|kv| kv.0 != "cid")
            .collect();
        if fields.len() < 7 {
            continue;
        }

        let mut valid = true;
        for kv in fields {
            if let Some(r) = regexes.get(kv.0) {
                if !r.is_match(kv.1) {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            count += 1;
        }
    }
    count
}