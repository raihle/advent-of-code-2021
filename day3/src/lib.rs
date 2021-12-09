#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn most_common_bits() {
        let numbers = [3, 4, 5];
        assert_eq!(1, most_common_bit(&numbers, 0));
        assert_eq!(0, most_common_bit(&numbers, 1));
        assert_eq!(1, most_common_bit(&numbers, 2));
    }

    #[test]
    fn least_common_bits() {
        let numbers = [6, 7, 8];
        assert_eq!(1, least_common_bit(&numbers, 0));
        assert_eq!(0, least_common_bit(&numbers, 1));
        assert_eq!(0, least_common_bit(&numbers, 2));
    }

    #[test]
    fn epsilon_extracts_the_least_common_bits() {
        let bit_strings = [
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010"
        ];
        assert_eq!(9, epsilon(&bit_strings));
    }

    #[test]
    fn gamma_extracts_the_most_common_bits() {
        let bit_strings = [
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010"
        ];
        assert_eq!(22, gamma(&bit_strings));
    }

    #[test]
    fn parses_binary_numbers() {
        let bit_strings = [
            "101",
            "110",
            "111"   
        ];
        assert_eq!(vec![5_i32, 6, 7], parse_bit_strings(&bit_strings));
    }
}

pub fn epsilon(bit_strings: &[&str]) -> i32 {
    return extract_bits(bit_strings, least_common_bit);
}

pub fn gamma(bit_strings: &[&str]) -> i32 {
    return extract_bits(bit_strings, most_common_bit);
}

fn most_common_bit(numbers: &[i32], bit_index: u32) -> i32 {
    let bit_value = 2_i32.pow(bit_index);
    let higher_bit_value = 2 * bit_value;
    let number_of_ones = numbers.iter().filter(|x| (*x % higher_bit_value) >= bit_value).count();
    if number_of_ones > numbers.len() / 2 {
        return 1;
    }
    return 0;
}

fn least_common_bit(numbers: &[i32], bit_index: u32) -> i32 {
    return 1 - most_common_bit(numbers, bit_index);
}

fn parse_bit_strings(bit_strings: &[&str]) -> Vec<i32> {
    return bit_strings.iter().map(|x| i32::from_str_radix(x, 2).unwrap()).collect();
}

fn extract_bits(bit_strings: &[&str], bit_extractor: impl Fn(&[i32], u32) -> i32) -> i32 {
    let numbers = parse_bit_strings(&bit_strings);
    let mut result = 0;
    for n in 0..bit_strings[0].len() {
        let index = n.try_into().unwrap();
        result += 2_i32.pow(index) * bit_extractor(&numbers, index);
    }
    return result;
}
