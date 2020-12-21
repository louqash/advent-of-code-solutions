pub fn solve_v1() -> i32 {

    let data = super::load_file("day10.txt");
    let mut jolts = data.lines().map(|c| c.parse::<u8>().unwrap()).collect::<Vec<u8>>();
    jolts.sort();
    let mut diff_1j = 0;
    let mut diff_3j = 1; // for the device's built-adapter difference

    // for the first outlet difference
    if jolts[0] == 1 {
        diff_1j += 1;
    } else if jolts[0] == 3 {
        diff_3j += 1;
    }
    for i in 0..jolts.len()-1 {
        let diff = jolts[i+1] - jolts[i];
        if diff == 1 {
            diff_1j += 1;
        } else if diff == 3 {
            diff_3j += 1;
        }
    }

    diff_1j * diff_3j
}

pub fn solve_v2() -> u64 {

    let data = super::load_file("day10.txt");
    let mut jolts = data.lines().map(|c| c.parse::<u8>().unwrap()).collect::<Vec<u8>>();
    jolts.push(0);
    jolts.sort();
    jolts.push(jolts.last().unwrap()+3);

    let mut tmp: Vec<u64> = vec![0; jolts.len()-1];
    tmp[0] = 1;
    for s in 1..jolts.len()-1 {
        for f in (0..s).rev() {
            if jolts[s] - jolts[f] <= 3 {
                tmp[s] += tmp[f]; // counting in how many ways we can arrange adapters
            } else {
                break;
            }
        }
    }

    *tmp.last().unwrap()
}