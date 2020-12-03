pub mod day1;
pub mod day2;

use std::fs;
pub fn load_file(name: &str) -> String {
    fs::read_to_string(String::from("test-vectors/") + name)
            .expect("Try `cargo run` in the respository's root dir.")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_all() {
        println!("Day 1 Task:");
        println!("[part 1]: {}", crate::day1::solve(2));
        println!("[part 2]: {}", crate::day1::solve(3));
        println!("");

        println!("Day 2 Task:");
        println!("[part 1]: {}", crate::day2::solve_v1());
        println!("[part 2]: {}", crate::day2::solve_v2());
        println!("");
    }
}
