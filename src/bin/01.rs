advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut dial = 50i32;
    let mut cnt = 0u64;
    let limit = 100i32;

    for line in lines {
        let direction = &line[0..1];
        let distance = line[1..].parse::<i32>().ok()?;
        let dial_long = match direction {
            "L" => dial - distance,
            "R" => dial + distance,
            _ => unreachable!(),
        };
        dial = dial_long.rem_euclid(limit);
        if dial == 0 {
            cnt += 1;
        }
    }

    Some(cnt)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut dial = 50i32;
    let mut cnt = 0u64;
    let limit = 100i32;

    for line in lines {
        let direction = &line[0..1];
        let distance = line[1..].parse::<i32>().ok()?;

        match direction {
            "L" => {
                let dial_long = dial - distance;
                let mut corssings = (dial_long / limit).abs();
                if dial != 0 && dial_long <= 0 {
                    corssings += 1;
                }
                dial = dial_long.rem_euclid(limit);
                cnt += corssings as u64;
            }
            "R" => {
                let dial_long = dial + distance;
                let mut corssings = (dial_long / limit).abs();
                if dial != 0 && dial_long == 0 {
                    corssings += 1;
                }
                dial = dial_long.rem_euclid(limit);
                cnt += corssings as u64;
            }
            _ => {}
        }
    }
    Some(cnt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_two_inputs() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(5847));
    }
}
