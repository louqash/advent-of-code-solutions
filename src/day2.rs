pub fn solve_v1() -> i32 {
    let data = super::load_file("day2.txt");

    let mut count = 0;
    for line in data.split("\n") {
        let parts: Vec<&str> = line.split(" ").map(|s| s.trim()).collect();
        assert_eq!(parts.len(), 3, "Invalid input file");

        let limits: Vec<i32> = parts[0]
            .split("-")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let letter = parts[1].chars().nth(0).unwrap();
        let password = parts[2];

        let encounters = password.chars().filter(|&c| c == letter).count() as i32;
        if encounters >= limits[0] && encounters <= limits[1] {
            count += 1;
        }
    }
    count
}
pub fn solve_v2() -> i32 {
    let data = super::load_file("day2.txt");

    let mut count = 0;
    for line in data.split("\n") {
        let parts: Vec<&str> = line.split(" ").map(|s| s.trim()).collect();
        assert_eq!(parts.len(), 3, "Invalid input file");

        let limits: Vec<usize> = parts[0]
            .split("-")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let letter = parts[1].chars().nth(0).unwrap();
        let password = parts[2];

        let l1 = password.chars().nth(limits[0] - 1).unwrap();
        let l2 = password.chars().nth(limits[1] - 1).unwrap();

        if (l1 == letter || l2 == letter) && l1 != l2 {
            count += 1;
        }
    }
    count
}
