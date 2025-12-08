advent_of_code::solution!(8);

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn distance(&self, rhs: &Point) -> f64 {
        let sum = rhs.x.wrapping_sub(self.x).pow(2)
            + rhs.y.wrapping_sub(self.y).pow(2)
            + rhs.z.wrapping_sub(self.z).pow(2);
        (sum as f64).sqrt()
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let nums = value.split(',').collect::<Vec<&str>>();
        let x = nums[0].parse::<_>().unwrap_or(0);
        let y = nums[1].parse::<_>().unwrap_or(0);
        let z = nums[2].parse::<_>().unwrap_or(0);
        Self { x, y, z }
    }
}

#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    last: Option<(usize, usize)>,
    size: Vec<u64>, // size of each set, valid only for roots
}

impl UnionFind {
    /// Initialize n separate sets: {0}, {1}, ..., {n-1}
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            last: None,
            size: vec![1; n],
        }
    }

    /// Find the root of x, with path compression
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// Union two sets; return the new root
    pub fn union(&mut self, lhs: usize, rhs: usize) -> usize {
        let mut a = self.find(lhs);
        let mut b = self.find(rhs);

        if a == b {
            return a;
        }

        // union by size: ensure a is the larger root
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }

        // attach b under a
        self.parent[b] = a;
        self.size[a] += self.size[b];

        if self.all_included(a) {
            self.last = Some((lhs, rhs));
        }

        a
    }

    pub fn all_included(&self, a: usize) -> bool {
        self.size[a] == self.size.len() as u64
    }

    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    pub fn top_sizes(&self, top: usize) -> Vec<u64> {
        let mut sizes = self.size.clone();
        let n = sizes.len();
        sizes.sort();
        sizes[n - top..].into()
    }

    /// Return the size of the set containing x
    pub fn set_size(&mut self, x: usize) -> u64 {
        let root = self.find(x);
        self.size[root]
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();
    let points = lines.into_iter().map(Point::from).collect::<Vec<Point>>();
    let n = points.len();
    let mut distances = vec![];
    for i in 0..n - 1 {
        for j in i + 1..n {
            distances.push((points[i].distance(&points[j]), i, j));
        }
    }
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Greater));
    let mut uf = UnionFind::new(n);
    for dis in distances.iter().take(n) {
        uf.union(dis.1, dis.2);
    }
    Some(uf.top_sizes(3).into_iter().product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();
    let points = lines.into_iter().map(Point::from).collect::<Vec<Point>>();
    let n = points.len();
    let mut distances = vec![];
    for i in 0..n - 1 {
        for j in i + 1..n {
            distances.push((points[i].distance(&points[j]), i, j));
        }
    }
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Greater));
    let mut uf = UnionFind::new(n);
    for dis in &distances {
        let p = uf.union(dis.1, dis.2);
        if uf.all_included(p) {
            let last = uf.last?;
            return Some((points[last.0].x * points[last.1].x) as u64);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(375));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_one_inputs() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_ne!(result, Some(792));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_two_inputs() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6844224));
    }
}
