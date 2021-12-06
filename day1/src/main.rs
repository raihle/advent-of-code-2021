use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");
    let measurements: Vec<i32> = input.lines().map(|num| num.parse::<i32>().unwrap()).collect();
    println!("The array is {} numbers long", measurements.len());
    let increasing_measurements = count_increasing_numbers(&measurements, 1);
    println!("{} measurements were larger than the last one (part 1)", increasing_measurements);
    let increasing_measurements_sliding = count_increasing_numbers(&measurements, 3);
    println!("{} sliding measurements were larger than the last one (part 2)", increasing_measurements_sliding);
}

fn count_increasing_numbers(numbers: &[i32], distance: usize) -> i32 {
    let mut increases = 0;
    for n in distance..numbers.len() {
        if numbers[n] > numbers[n-distance] {
            increases += 1;
        }
    }
    return increases;
}

/*struct SlidingWindow<'a> {
    curr: usize,
    size: usize,
    data: &'a [i32],
}

impl Iterator for SlidingWindow<'_> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr + self.size >= self.data.len() {
            return None;
        }
        let mut sum = 0;
        for n in self.curr..(self.curr + self.size) {
            sum += self.data[n];
        }
        self.curr = self.curr + 1;
        return Some(sum);
    }

    fn size_hint() {
        return (this.data.len() - this.size + 1, Some(this.data.len() - this.size + 1));
    }
}

fn sliding_window(data: &[i32], size: usize) -> SlidingWindow {
    SlidingWindow { curr: 0, size: size, data: data }
}
*/
