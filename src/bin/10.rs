advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let records = input.lines().collect::<Vec<&str>>();
    let mut ans = 0u64;
    for record in records {
        let parts = record.split_ascii_whitespace().collect::<Vec<&str>>();
        let result = parts[0];
        let mut targets = Vec::with_capacity(result.len() - 2);
        for ch in &result.as_bytes()[1..result.len() - 1] {
            match ch {
                b'.' => targets.push(false),
                b'#' => targets.push(true),
                _ => unimplemented!("unknown ch {ch}"),
            }
        }

        let mut actions = Vec::with_capacity(parts.len() - 2);
        for part in &parts[1..parts.len() - 1] {
            let action = part[1..part.len() - 1]
                .split(',')
                .map(|num| num.parse::<usize>().expect("not a number"))
                .collect::<Vec<_>>();
            actions.push(action);
        }

        ans += part_one_helper(targets, actions);
    }
    Some(ans)
}

fn part_one_helper(targets: Vec<bool>, actions: Vec<Vec<usize>>) -> u64 {
    fn dfs(
        cur: usize,
        step: &mut u64,
        ans: &mut u64,
        candidates: &mut Vec<bool>,
        targets: &Vec<bool>,
        actions: &Vec<Vec<usize>>,
    ) {
        if *step >= *ans {
            return;
        }

        if candidates == targets {
            *ans = *step;
            return;
        }

        for i in cur..actions.len() {
            let action = &actions[i];
            *step += 1;
            for &pos in action {
                candidates[pos] = !candidates[pos];
            }

            dfs(i + 1, step, ans, candidates, targets, actions);

            *step -= 1;
            for &pos in action {
                candidates[pos] = !candidates[pos];
            }
        }
    }

    let n = targets.len();
    let mut ans = u64::MAX;
    let mut step = 0;
    let mut candidates = vec![false; n];
    dfs(0, &mut step, &mut ans, &mut candidates, &targets, &actions);
    ans
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_one_inputs() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(419));
    }

    #[test]
    #[ignore = "unimplemented"]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
