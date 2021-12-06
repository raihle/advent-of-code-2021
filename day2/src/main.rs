use std::fs;
use day2::move_absolute;
use day2::move_relative;

fn main() {
    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let abs_results = move_absolute(&input);
    println!("Absolute movement (part 1):  {} X, {} Y (product {})", abs_results.x, abs_results.y, abs_results.x * abs_results.y);

    let rel_results = move_relative(&input);
    println!("Relative movement (part 2):  {} X, {} Y (product {})", rel_results.x, rel_results.y, rel_results.x * rel_results.y);
}
