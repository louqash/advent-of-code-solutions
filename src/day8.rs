use std::collections::HashSet;

pub fn solve_v1() -> i32 {
    let data = super::load_file("day8.txt");
    let instructions: Vec<&str> = data.lines().collect();

    let mut visited: HashSet<usize> = HashSet::new();
    let mut acc = 0;
    let mut pc = 0;
    while pc < instructions.len() && !visited.contains(&pc) {
        // possible instructions are `acc`, `nop` and `jmp` so the line can be broken at 4th character
        let inst = instructions[pc].split_at(3).0;
        let num = instructions[pc].split_at(4).1.parse::<i32>().unwrap();
        visited.insert(pc);
        pc += 1;
        match inst {
            "acc" => acc += num,
            "jmp" => pc = (pc as i32 + num - 1) as usize,
            _ => (),
        }
    }
    acc
}

pub fn solve_v2() -> i32 {
    let data = super::load_file("day8.txt");
    let instructions: Vec<&str> = data.lines().collect();

    let mut visited: HashSet<usize> = HashSet::new();
    let mut changes: HashSet<usize> = HashSet::new();
    let mut acc = 0;
    let mut pc = 0;
    let mut changed = false;
    while pc < instructions.len() {
        // possible instructions are `acc`, `nop` and `jmp` so the line can be broken at 4th character
        let mut inst = instructions[pc].split_at(3).0;
        let num = instructions[pc].split_at(4).1.parse::<i32>().unwrap();
        if !changed && inst != "acc" && !changes.contains(&pc) {
            changed = true;
            changes.insert(pc);
            if inst == "nop" {
                inst = "jmp";
            } else { // if inst == "jmp" {
                inst = "nop";
            }
        }
        visited.insert(pc);
        pc += 1;
        match (inst, num) {
            ("acc", _) => acc += num,
            ("jmp", _) => pc = (pc as i32 + num - 1) as usize,
            _ => (),
        }
        if visited.contains(&pc) {
            // reset
            visited.clear();
            acc = 0;
            pc = 0;
            changed = false;
        }
    }
    acc
}
