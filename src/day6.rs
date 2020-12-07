use std::collections::HashMap;

pub fn solve_v1() -> i32 {
    let data = super::load_file("day6.txt");
    let groups: Vec<&str> = data.split("\n\n").collect() ;

    let mut count = 0;
    for group in groups {
        let mut chars: HashMap<u8, i32> = HashMap::new();
        let answers = group.as_bytes();
        for &answer in answers {
            if answer != b'\n' {
                let count = chars.entry(answer).or_insert(0);
                *count += 1;
            }
        }
        count += chars.len() as i32;
    }

    count
}

pub fn solve_v2() -> i32 {
    let data = super::load_file("day6.txt");
    let groups: Vec<&str> = data.split("\n\n").collect() ;

    let mut count = 0;
    for group in groups {
        let mut chars: HashMap<u8, i32> = HashMap::new();
        let mut groups = 1;
        let answers = group.as_bytes();
        for &answer in answers {
            if answer != b'\n' {
                let count = chars.entry(answer).or_insert(0);
                *count += 1;
            } else {
                groups += 1;
            }
        }

        for (_, &v) in chars.iter() {
            if v == groups {
                count += 1;
            }
        }
    }

    count
}