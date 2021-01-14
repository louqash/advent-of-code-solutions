pub fn solve_v1() -> usize {
    let data: String = super::load_file("day11.txt");
    let width = data.find('\n').unwrap() as i32;
    let mut data = data.replace("\n", "").into_bytes();
    let height = (data.len() as i32)/width;
    let mut tmp = data.clone();

    let neighbours: [(i32, i32); 8]= [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), (1, 0),
        (-1, 1), (0, 1), (1, 1)
    ];
    loop {
        for y in 0..height {
            for x in 0..width {
                let position = (y*width + x) as usize;
                let mut taken: usize = 0;
                for neighbour in &neighbours {
                    let nx = x + neighbour.0;
                    let ny = y + neighbour.1;
                    if nx < 0 || nx >= width || ny < 0 || ny >= height {
                        continue;
                    }
                    let npos = (ny * width + nx) as usize;
                    if data[npos] == b'#' {
                        taken += 1;
                    }
                }
                if data[position] == b'L' && taken == 0 {
                    tmp[position] = b'#';
                } else if data[position] == b'#' && taken >= 4 {
                    tmp[position] = b'L';
                }
            }
        }
        if tmp == data {
            break;
        }
        data = tmp.clone();
    }
    data.into_iter().filter(|&c| c == b'L').count()
}

pub fn solve_v2() -> usize {
    let data: String = super::load_file("day11.txt");
    let width = data.find('\n').unwrap() as i32;
    let mut data = data.replace("\n", "").into_bytes();
    let height = (data.len() as i32)/width;
    let mut tmp = data.clone();

    let neighbours: [(i32, i32); 8]= [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), (1, 0),
        (-1, 1), (0, 1), (1, 1)
    ];
    loop {
        for y in 0..height {
            for x in 0..width {
                let position = (y*width + x) as usize;
                let mut taken: usize = 0;
                for neighbour in &neighbours {
                    let mut nx = x;
                    let mut ny = y;
                    loop {
                        nx += neighbour.0;
                        ny += neighbour.1;
                        if nx < 0 || nx >= width || ny < 0 || ny >= height {
                            break;
                        }
                        let npos = (ny * width + nx) as usize;
                        if data[npos] == b'#' {
                            taken += 1;
                            break
                        } else if data[npos] == b'L' {
                            break;
                        }
                    }
                }
                if data[position] == b'L' && taken == 0 {
                    tmp[position] = b'#';
                } else if data[position] == b'#' && taken >= 5 {
                    tmp[position] = b'L';
                }
            }
        }
        if tmp == data {
            break;
        }
        data = tmp.clone();
    }
    data.into_iter().filter(|&c| c == b'#').count()
}