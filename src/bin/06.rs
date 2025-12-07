advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();
    let n = lines.len();
    let numbers = lines[..n - 1]
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|num| num.parse::<u64>().ok())
                .collect()
        })
        .collect::<Vec<Vec<u64>>>();

    let operators = lines
        .last()?
        .split_ascii_whitespace()
        .filter(|op| !op.is_empty())
        .collect::<Vec<&str>>();
    let mut ans = 0u64;
    for (id, op) in operators.into_iter().enumerate() {
        ans += match op {
            "+" => (0..n - 1).map(|i| numbers[i][id]).sum(),
            "*" => (0..n - 1).map(|i| numbers[i][id]).product::<u64>(),
            _ => panic!("operator {op} not supported"),
        };
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();
    let n = lines.len();
    let operators = lines
        .last()?
        .split_ascii_whitespace()
        .collect::<Vec<&str>>();

    let numbers = lines[..n - 1]
        .iter()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let mut ans = 0u64;
    let mut cur = 0usize;
    for op in operators.into_iter() {
        ans += match op {
            "+" => {
                let mut sum = 0;
                while cur < numbers[0].len() {
                    let mut num = 0;
                    for row in &numbers {
                        let ch = row[cur];
                        if !ch.is_ascii_digit() {
                            continue;
                        }
                        num = num * 10 + (ch - b'0') as u64;
                    }
                    cur += 1;
                    if num == 0 {
                        break;
                    }
                    sum += num;
                }
                sum
            }
            "*" => {
                let mut product = 1;
                while cur < numbers[0].len() {
                    let mut num = 0;
                    for row in &numbers {
                        let ch = row[cur];
                        if !ch.is_ascii_digit() {
                            continue;
                        }
                        num = num * 10 + (ch - b'0') as u64;
                    }
                    cur += 1;
                    if num == 0 {
                        break;
                    }
                    product *= num;
                }
                product
            }
            _ => panic!("operator {op} not supported"),
        };
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_one_inputs() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(3261038365331));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_two_inputs() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(8342588849093));
    }

    #[test]
    fn dummy_test() {
        let line = " 45 64  387 23 ";
        let numbers = line.split(' ').collect::<Vec<&str>>();
        dbg!(numbers);
    }
}
