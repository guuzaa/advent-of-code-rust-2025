use std::collections::HashSet;

advent_of_code::solution!(4);

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();
    let n = lines.len();
    let mut graph = Vec::with_capacity(n);
    for line in lines {
        graph.push(line.as_bytes().to_vec());
    }
    let n = n as i32;
    let w = graph[0].len() as i32;
    let mut cnt = 0u64;

    for i in 0..w {
        for j in 0..n {
            if graph[i as usize][j as usize] != b'@' {
                continue;
            }

            let mut rolls = 0;
            for (x, y) in &NEIGHBORS {
                let ii = i + x;
                let jj = j + y;
                if ii >= 0 && ii < w && jj >= 0 && jj < n && graph[ii as usize][jj as usize] == b'@'
                {
                    rolls += 1;
                }
            }
            if rolls < 4 {
                cnt += 1;
            }
        }
    }

    Some(cnt)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();
    let n = lines.len();
    let mut graph = Vec::with_capacity(n);
    for line in lines {
        graph.push(line.as_bytes().to_vec());
    }
    let n = n as i32;
    let w = graph[0].len() as i32;
    let mut cnt = 0u64;
    let mut removals = HashSet::new();

    loop {
        let removals_cnt = removals.len();
        for i in 0..w {
            for j in 0..n {
                if graph[i as usize][j as usize] != b'@' || removals.contains(&(i, j)) {
                    continue;
                }

                let mut rolls = 0;
                for (x, y) in &NEIGHBORS {
                    let ii = i + x;
                    let jj = j + y;
                    if ii >= 0
                        && ii < w
                        && jj >= 0
                        && jj < n
                        && graph[ii as usize][jj as usize] == b'@'
                        && !removals.contains(&(ii, jj))
                    {
                        rolls += 1;
                    }
                }
                if rolls < 4 {
                    cnt += 1;
                    removals.insert((i, j));
                }
            }
        }

        if removals.len() == removals_cnt {
            break;
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
        assert_eq!(result, Some(13));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_one_inputs() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1491));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_two_inputs() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(8722));
    }
}
