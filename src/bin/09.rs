advent_of_code::solution!(9);

#[derive(Clone, Copy, Debug)]
struct Grid {
    x: i64,
    y: i64,
}

impl Grid {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn area(&self, rhs: &Grid) -> u64 {
        ((1 + (self.x - rhs.x).abs()) * (1 + (self.y - rhs.y).abs())) as u64
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut parts = value.split(',');
        let x = parts.next().unwrap().parse::<_>().unwrap();
        let y = parts.next().unwrap().parse::<_>().unwrap();
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
struct Line {
    start: Grid,
    end: Grid,
}

impl Line {
    pub fn new(start: Grid, end: Grid) -> Self {
        Self { start, end }
    }

    pub fn intersects(&self, other: &Line) -> bool {
        let d1 = (self.end.x - self.start.x) * (other.start.y - self.start.y)
            - (self.end.y - self.start.y) * (other.start.x - self.start.x);
        let d2 = (self.end.x - self.start.x) * (other.end.y - self.start.y)
            - (self.end.y - self.start.y) * (other.end.x - self.start.x);
        let d3 = (other.end.x - other.start.x) * (self.start.y - other.start.y)
            - (other.end.y - other.start.y) * (self.start.x - other.start.x);
        let d4 = (other.end.x - other.start.x) * (self.end.y - other.start.y)
            - (other.end.y - other.start.y) * (self.end.x - other.start.x);

        ((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0))
    }
}

struct Polygon {
    vertices: Vec<Grid>,
}

impl Polygon {
    pub fn new(vertices: Vec<Grid>) -> Self {
        Self { vertices }
    }

    pub fn contains(&self, point: &Grid) -> bool {
        let mut inside = false;
        let n = self.vertices.len();
        for i in 0..n {
            let j = (i + n - 1) % n;
            let pi = &self.vertices[i];
            let pj = &self.vertices[j];

            if (pi.y > point.y) != (pj.y > point.y) {
                let x_intersect = (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x;
                if point.x < x_intersect {
                    inside = !inside;
                }
            }
        }
        inside
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grids = input.lines().map(Grid::from).collect::<Vec<_>>();
    let n = grids.len();
    let mut ans = 0u64;
    for i in 0..n - 1 {
        for j in i + 1..n {
            ans = ans.max(grids[i].area(&grids[j]));
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grids = input.lines().map(Grid::from).collect::<Vec<_>>();
    let polygon = Polygon::new(grids.clone());
    let n = grids.len();
    let mut ans = 0u64;
    for i in 0..n - 1 {
        for j in i + 1..n {
            let vertices = [
                Grid::new(
                    grids[i].x.min(grids[j].x) + 1,
                    grids[i].y.min(grids[j].y) + 1,
                ),
                Grid::new(
                    grids[i].x.min(grids[j].x) + 1,
                    grids[i].y.max(grids[j].y) - 1,
                ),
                Grid::new(
                    grids[i].x.max(grids[j].x) - 1,
                    grids[i].y.min(grids[j].y) + 1,
                ),
                Grid::new(
                    grids[i].x.max(grids[j].x) - 1,
                    grids[i].y.max(grids[j].y) - 1,
                ),
            ];
            if !vertices.iter().all(|v| polygon.contains(v)) {
                continue;
            }

            let rectangle = vec![
                Line::new(vertices[0], vertices[1]),
                Line::new(vertices[1], vertices[3]),
                Line::new(vertices[3], vertices[2]),
                Line::new(vertices[2], vertices[0]),
            ];

            let mut intersects = false;
            for rect in &rectangle {
                if (0..grids.len())
                    .map(|i| Line::new(grids[i], grids[(i + 1) % grids.len()]))
                    .any(|edge| rect.intersects(&edge))
                {
                    intersects = true;
                    break;
                }
            }

            if intersects {
                continue;
            }

            ans = ans.max(grids[i].area(&grids[j]));
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
        assert_eq!(result, Some(50));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_one_inputs() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(4733727792));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_two_inputs() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1566346198));
    }
}
