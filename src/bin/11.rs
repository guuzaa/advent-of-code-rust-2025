use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

advent_of_code::solution!(11);

fn parse_graph(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let device = parts[0].trim().to_string();
        let outputs: Vec<String> = parts[1].split_whitespace().map(|s| s.to_string()).collect();
        graph.insert(device, outputs);
    }

    graph
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse_graph(input);
    let mut memory = HashMap::new();
    dfs_count_paths(&mut memory, &graph, "you", "out")
}

fn dfs_count_paths(
    memory: &mut HashMap<String, u64>,
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
) -> Option<u64> {
    if current == target {
        return Some(1);
    }

    if let Some(ans) = memory.get(current) {
        return Some(*ans);
    }

    let ans = graph
        .get(current)?
        .iter()
        .filter_map(|output| dfs_count_paths(memory, graph, output, target))
        .sum();
    memory.insert(current.to_string(), ans);
    Some(ans)
}

#[allow(unused)]
fn bfs_count_paths(graph: &HashMap<String, Vec<String>>, start: &str, end: &str) -> Option<u64> {
    let mut count = 0;
    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back(start);
    while let Some(cur) = queue.pop_front() {
        if cur == end {
            count += 1;
            continue;
        }
        graph
            .get(cur)?
            .iter()
            .for_each(|output| queue.push_back(output));
    }

    Some(count)
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    name: String,
    has_fft: bool,
    has_dac: bool,
}

impl Node {
    fn new(name: String, has_fft: bool, has_dac: bool) -> Self {
        Self {
            name,
            has_fft,
            has_dac,
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = parse_graph(input);
    let mut memory = HashMap::new();
    memoized_dfs_count_paths(&mut memory, &graph, "svr", "out", false, false)
}

fn memoized_dfs_count_paths(
    memory: &mut HashMap<Node, u64>,
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    has_fft: bool,
    has_dac: bool,
) -> Option<u64> {
    if current == target && has_fft && has_dac {
        return Some(1);
    }
    let node = Node::new(current.to_string(), has_fft, has_dac);
    if let Some(cnt) = memory.get(&node) {
        return Some(*cnt);
    }

    let ans = graph
        .get(current)
        .unwrap_or(&Vec::new())
        .iter()
        .filter_map(|output| {
            memoized_dfs_count_paths(
                memory,
                graph,
                output,
                target,
                has_fft || output == "fft",
                has_dac || output == "dac",
            )
        })
        .sum();
    memory.insert(node, ans);
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_one_inputs() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(708));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    #[ignore = "private"]
    fn test_part_two_inputs() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(545394698933400));
    }
}
