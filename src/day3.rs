fn count_trees_encountered(map: &Vec<&str>, mov: (usize, usize)) -> i32 {
    let width = map[0].len();
    let mut pos: (usize, usize) = (0, 0);

    let mut count = 0;
    while pos.1 < map.len() {
        if map[pos.1].chars().nth(pos.0).unwrap() == '#' {
            count += 1;
        }
        pos = ((pos.0 + mov.0) % width, pos.1 + mov.1);
    }

    count
}

pub fn solve_v1() -> i32 {
    let data: String = super::load_file("day3.txt");
    let map: Vec<&str> = data.split("\n").collect();
    count_trees_encountered(&map, (3, 1))
}

pub fn solve_v2() -> i64 {
    let data: String = super::load_file("day3.txt");
    let map: Vec<&str> = data.split("\n").collect();

    let movs: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut counts: Vec<i64> = vec![0; movs.len()];
    for i in 0..movs.len() {
        counts[i] = count_trees_encountered(&map, movs[i]) as i64;
    }

    counts.iter().product()
}
