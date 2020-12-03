pub mod day1;

use std::fs;
pub fn load_file(name: &str) -> String {
    fs::read_to_string(String::from("test-vectors/") + name)
            .expect("Try `cargo run` in the respository's root dir.")
}

#[cfg(test)]
mod tests {
    #[test]
    fn day1() {
        println!("Day1 result [part 1]: {}", crate::day1::solve(2));
        println!("Day1 result [part 2]: {}", crate::day1::solve(3));
    }
}
