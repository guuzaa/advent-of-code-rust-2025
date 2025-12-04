advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let banks = input.lines().collect::<Vec<&str>>();
    let mut ans = 0u64;
    for bank in banks {
        let mut max_num = 0u64;
        let mut max_sum = 0u64;

        for &ch in bank.as_bytes() {
            let number = (ch - b'0') as u64;
            max_sum = max_sum.max((max_num * 10) + number);
            max_num = max_num.max(number);
        }
        ans += max_sum;
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks = input.lines().collect::<Vec<&str>>();
    let mut ans = 0u64;
    let length = 12usize;

    for bank in banks {
        let numbers = bank.as_bytes();
        let n = numbers.len();
        let mut max_sum = 0;
        let mut start = 0usize;
        let mut cnt = 0;
        let mut end = n - length + cnt;
        while cnt < length && end >= start {
            let (idx, max_val) =
                numbers[start..=end]
                    .iter()
                    .enumerate()
                    .max_by(|(i1, v1), (i2, v2)| {
                        v1.cmp(v2).then_with(|| i2.cmp(i1)) // reverse index order in tie
                    })?;
            max_sum = max_sum * 10 + (max_val - b'0') as u64;

            start += idx + 1;
            cnt += 1;
            end = n - length + cnt;
        }

        ans += max_sum;
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_one_inputs() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(17311));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_two_inputs() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(171419245422055));
    }
}
