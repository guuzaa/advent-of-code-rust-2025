advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input.trim().split(",").collect::<Vec<&str>>();
    let mut ans = 0u64;
    for range in ranges {
        let numbers = range.split("-").collect::<Vec<&str>>();
        let start = numbers[0].parse::<u64>().ok()?;
        let end = numbers[1].parse::<u64>().ok()?;
        for number in start..=end {
            let number_str = number.to_string();
            let n = number_str.len();
            if n % 2 == 0 && number_str[0..n / 2] == number_str[n / 2..] {
                ans += number;
            }
        }
    }
    Some(ans)
}

fn is_repetition(s: &str) -> bool {
    let doubled = format!("{s}{s}");
    doubled[1..doubled.len() - 1].contains(s)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = input.trim().split(",").collect::<Vec<&str>>();
    let mut ans = 0u64;

    for range in ranges {
        let numbers = range.split("-").collect::<Vec<&str>>();
        let start = numbers[0].parse::<u64>().ok()?;
        let end = numbers[1].parse::<u64>().ok()?;
        for number in start..=end {
            let number_str = number.to_string();
            if is_repetition(&number_str) {
                ans += number;
            }
        }
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_one_inputs() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(20223751480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_two_inputs() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(30260171216));
    }
}
