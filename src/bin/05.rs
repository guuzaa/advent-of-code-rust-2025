advent_of_code::solution!(5);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn overlap(&self, rhs: &Range) -> bool {
        let self_range = self.start..=self.end;
        let other_range = rhs.start..=rhs.end;

        self_range.contains(&rhs.start)
            || self_range.contains(&rhs.end)
            || other_range.contains(&self.start)
            || other_range.contains(&self.end)
    }

    #[inline]
    fn contains(&self, rhs: u64) -> bool {
        rhs >= self.start && rhs <= self.end
    }

    fn merge(&mut self, rhs: Range) {
        self.start = self.start.min(rhs.start);
        self.end = self.end.max(rhs.end);
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.split("\n");
    let mut ranges: Vec<Range> = vec![];
    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        let mut numbers = line.split('-');
        let start = numbers.next()?.parse::<u64>().ok()?;
        let end = numbers.next()?.parse::<u64>().ok()?;
        let range = Range::new(start, end);
        let pos = ranges.partition_point(|&x| x < range);

        if pos < ranges.len() && range.overlap(&ranges[pos]) {
            ranges[pos].merge(range);
        } else {
            ranges.insert(pos, range);
        }

        if pos > 0 && ranges[pos - 1].overlap(&ranges[pos]) {
            let next_range = ranges[pos];
            ranges[pos - 1].merge(next_range);
            ranges.remove(pos);
        }
    }

    let mut fresh = 0u64;
    for id in lines.filter_map(|line| line.parse::<u64>().ok()) {
        let pos = ranges.partition_point(|x| id > x.end);
        if pos < ranges.len() && ranges[pos].contains(id) {
            fresh += 1;
        }
    }

    Some(fresh)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.split("\n");
    let mut ranges: Vec<(u64, u64)> = vec![];
    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        let mut numbers = line.split('-');
        let start = numbers.next()?.parse::<u64>().ok()?;
        let end = numbers.next()?.parse::<u64>().ok()?;
        let range = (start, end);
        let pos = ranges.partition_point(|&x| x < range);

        if pos < ranges.len()
            && ((start..=end).contains(&ranges[pos].0)
                || (start..=end).contains(&ranges[pos].1)
                || (ranges[pos].0..=ranges[pos].1).contains(&start)
                || (ranges[pos].0..=ranges[pos].1).contains(&end))
        {
            ranges[pos].0 = ranges[pos].0.min(start);
            ranges[pos].1 = ranges[pos].1.max(end);
        } else {
            ranges.insert(pos, range);
        }

        if pos > 0 && (ranges[pos - 1].0..=ranges[pos - 1].1).contains(&ranges[pos].0) {
            ranges[pos - 1].0 = ranges[pos - 1].0.min(ranges[pos].0);
            ranges[pos - 1].1 = ranges[pos - 1].1.max(ranges[pos].1);
            ranges.remove(pos);
        }
    }

    Some(ranges.into_iter().map(|range| range.1 - range.0 + 1).sum())
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
    #[ignore = "private"]
    fn test_part_one_inputs() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(674));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_two_inputs() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(352509891817881));
    }
}
