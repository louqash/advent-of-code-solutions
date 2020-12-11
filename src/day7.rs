use std::collections::{HashMap, HashSet, LinkedList};

// probably not the most optimal way of solving those tasks
// but I don't have better idea at the moment
// TODO(pixsll): optimize it!

pub fn solve_v1() -> i32 {
    let data = super::load_file("day7.txt");

    let mut bags_capable: HashSet<&str> = HashSet::new();
    let mut bags: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    for line in data.split("\n") {
        // line format
        // <style> <color> bags contain <number> (<style> <color> bag[s]*,)* <style> <color> bag[s]*.
        let spaces: Vec<usize> = line
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == ' ')
            .map(|(i, _)| i)
            .collect();
        let inner_bags = line.chars().filter(|&c| c == ',').count() + 1;
        let bag = &line[0..spaces[1]];
        let mut contents: HashMap<&str, i32> = HashMap::new();

        for n in 0..inner_bags {
            let offset = 4 + n * 4;
            let cap_str = &line[spaces[offset - 1] + 1..spaces[offset - 1] + 2];
            let capacity;
            if let Ok(n) = cap_str.parse::<i32>() {
                capacity = n;
            } else {
                continue;
            }
            let inner_bag = &line[spaces[offset] + 1..spaces[offset + 2]];
            if inner_bag == "shiny gold" {
                bags_capable.insert(bag);
            }
            contents.insert(inner_bag, capacity);
        }
        bags.insert(bag, contents);
    }

    let mut added_new = true;
    while added_new {
        added_new = false;
        for (bag, map) in bags.iter() {
            for (inner_bag, _) in map.iter() {
                if bags_capable.contains(inner_bag) {
                    if bags_capable.insert(bag) {
                        added_new = true;
                    }
                }
            }
        }
    }

    bags_capable.len() as i32
}

pub fn solve_v2() -> i32 {
    let data = super::load_file("day7.txt");

    let mut bags: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    for line in data.split("\n") {
        // line format
        // <style> <color> bags contain <number> (<style> <color> bag[s]*,)* <style> <color> bag[s]*.
        let spaces: Vec<usize> = line
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == ' ')
            .map(|(i, _)| i)
            .collect();
        let inner_bags = line.chars().filter(|&c| c == ',').count() + 1;
        let bag = &line[0..spaces[1]];
        let mut contents: HashMap<&str, i32> = HashMap::new();

        for n in 0..inner_bags {
            let offset = 4 + n * 4;
            let cap_str = &line[spaces[offset - 1] + 1..spaces[offset - 1] + 2];
            let capacity;
            if let Ok(n) = cap_str.parse::<i32>() {
                capacity = n;
            } else {
                continue;
            }
            let inner_bag = &line[spaces[offset] + 1..spaces[offset + 2]];
            contents.insert(inner_bag, capacity);
        }
        bags.insert(bag, contents);
    }

    let mut sum = 0;
    let mut list: LinkedList<(&str, i32)> = LinkedList::new();
    list.push_back(("shiny gold", 1));

    while list.len() > 0 {
        for _ in 0..list.len() {
            let (bag, mul) = list.pop_front().unwrap();
            for (inner_bag, count) in bags[bag].iter() {
                sum += mul * count;
                list.push_back((inner_bag, mul * count));
            }
        }
    }

    sum
}
