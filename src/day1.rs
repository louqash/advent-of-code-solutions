pub fn solve(n: usize) -> i32 {
    let data = super::load_file("day1.txt");
    let numbers: Vec<i32> = data
        .trim()
        .split("\n")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut idxs = (0..n).collect::<Vec<usize>>();
    loop {
        for i in (0..n).rev() {
            if idxs[i] >= numbers.len() {
                if i == 0 {
                    return 0;
                }
                idxs[i] = 0;
                idxs[i - 1] += 1;
            } else {
                break;
            }
        }
        let mut sum = 0;
        for &idx in idxs.iter() {
            sum += numbers[idx];
        }
        if sum == 2020 {
            let mut prod = 1;
            for &idx in idxs.iter() {
                prod *= numbers[idx];
            }
            return prod;
        }
        idxs[n - 1] += 1;
    }
}
