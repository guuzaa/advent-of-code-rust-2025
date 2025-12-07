advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let mut split = 0u64;
    let start_pos = lines.first()?.iter().position(|ch| *ch == b'S')?;
    let n = lines[0].len();
    let mut pre_tachyon = vec![0; n];
    pre_tachyon[start_pos] += 1;
    for line in lines[1..].iter() {
        if !line.contains(&b'^') {
            continue;
        }

        let mut cur_tachyon = vec![0; n];
        for (pos, cnt) in pre_tachyon.into_iter().enumerate() {
            if cnt == 0 {
                continue;
            }

            match line[pos] {
                b'.' => {
                    cur_tachyon[pos] = 1;
                }
                b'^' => {
                    if pos > 0 {
                        cur_tachyon[pos - 1] = 1;
                    }
                    if pos + 1 < line.len() {
                        cur_tachyon[pos + 1] = 1;
                    }
                    split += 1;
                }
                _ => unreachable!("unsupported {}", line[pos]),
            }
        }
        pre_tachyon = cur_tachyon;
    }
    Some(split)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let start_pos = lines.first()?.iter().position(|ch| *ch == b'S')?;
    let mut timelines = vec![0u64; lines[0].len()];
    timelines[start_pos] += 1;
    for line in lines[1..].iter() {
        if !line.contains(&b'^') {
            continue;
        }

        let mut cur_timelines = vec![0u64; lines[0].len()];
        for (pos, cnt) in timelines.into_iter().enumerate() {
            if cnt == 0 {
                continue;
            }

            match line[pos] {
                b'.' => {
                    cur_timelines[pos] += cnt;
                }
                b'^' => {
                    if pos > 0 {
                        cur_timelines[pos - 1] += cnt;
                    }
                    if pos + 1 < line.len() {
                        cur_timelines[pos + 1] += cnt;
                    }
                }
                _ => unreachable!("unsupported {}", line[pos]),
            }
        }

        timelines = cur_timelines;
    }

    Some(timelines.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_one_inputs() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1638));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_two_inputs() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(7759107121385));
    }
}
