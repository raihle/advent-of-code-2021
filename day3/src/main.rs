use std::fs;
use day3::gamma;
use day3::epsilon;

fn main() {
    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");
    let lines = input.lines().collect::<Vec<&str>>();

    let gamma_value = gamma(&lines);
    let epsilon_value = epsilon(&lines);
    println!("Gamma {}, Epsilon {}, Product (step 1) {}", gamma_value, epsilon_value, gamma_value * epsilon_value);
}
