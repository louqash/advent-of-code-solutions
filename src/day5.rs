pub fn solve_v1() -> i32 {

    let data = super::load_file("day5.txt");

    let mut biggest_id = 0;
    for line in data.trim().split("\n") {
        let mut row_lim = (0, 127);
        let mut col_lim = (0, 7);

        for c in line.chars() {
            if c == 'F' {
                row_lim.1 -= (row_lim.1 - row_lim.0 + 1) / 2;
            } else if c == 'B' {
                row_lim.0 += (row_lim.1 - row_lim.0 + 1) / 2;
            } else if c == 'L' {
                col_lim.1 -= (col_lim.1 - col_lim.0 + 1) / 2;
            } else { // if c == 'R' {
                col_lim.0 += (col_lim.1 - col_lim.0 + 1) / 2;
            }
        }

        let id = row_lim.0 * 8 + col_lim.0;
        if biggest_id < id {
            biggest_id = id;
        }
    }

    biggest_id
}

pub fn solve_v2() -> i32 {

    let data = super::load_file("day5.txt");

    let mut seats: Vec<u8> = vec![0; 128];
    for line in data.trim().split("\n") {
        let mut row_lim = (0, 127);
        let mut col_lim = (0, 7);

        for c in line.chars() {
            if c == 'F' {
                row_lim.1 -= (row_lim.1 - row_lim.0 + 1) / 2;
            } else if c == 'B' {
                row_lim.0 += (row_lim.1 - row_lim.0 + 1) / 2;
            } else if c == 'L' {
                col_lim.1 -= (col_lim.1 - col_lim.0 + 1) / 2;
            } else { // if c == 'R' {
                col_lim.0 += (col_lim.1 - col_lim.0 + 1) / 2;
            }
        }
        let col: &mut u8 = seats.get_mut(row_lim.0).unwrap();
        *col = *col | 1 << col_lim.0;
    }

    let my_seat = seats.into_iter().enumerate()
        .filter(|(_, row)| (!(row >> 1) & 0x3f).count_ones() == 1)
        .nth(0).unwrap();

    let col = 7 - (my_seat.1).leading_ones() as i32;
    let row = my_seat.0 as i32;

    row * 8 + col
}