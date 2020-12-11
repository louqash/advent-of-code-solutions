use std::collections::LinkedList;

pub fn solve_v1() -> u64 {
    let data = super::load_file("day9.txt");

    let mut buf: LinkedList<u64> = LinkedList::new();
    let mut numbers = data.lines().map(|n| n.parse::<u64>().unwrap());
    // read preamble
    for _ in 0..25 {
        buf.push_back(numbers.next().unwrap());
    }

    for number in numbers {
        let mut found = false;
        for &n1 in buf.iter() {
            for &n2 in buf.iter() {
                if n1 + n2 == number && n1 != n2 {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if !found {
            return number;
        }
        buf.pop_front();
        buf.push_back(number);
    }

    0
}

pub fn solve_v2() -> u64 {
    let data = super::load_file("day9.txt");

    let numbers = data
        .lines()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let target = 10884537;

    for start in 0..numbers.len() {
        let mut sum = 0;
        let mut min = std::u64::MAX;
        let mut max = 0;
        let mut n = 0;

        for next in start..numbers.len() {
            sum += numbers[next];
            n += 1;
            if sum > target {
                break;
            }
            if min > numbers[next] {
                min = numbers[next];
            } else if max < numbers[next] {
                max = numbers[next];
            }
            if sum == target {
                if n >= 2 {
                    return min + max;
                }
                break;
            }
        }
    }

    0
}
