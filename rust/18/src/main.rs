use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::cmp::Ordering;
use std::collections::{HashSet, HashMap, BinaryHeap};

#[derive(Clone, Debug, Eq, PartialEq)]
struct BestPath {
    pos: (i64, i64),
    cost: i64,
    heuristic: i64,
}

impl Ord for BestPath {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heuristic.cmp(&self.heuristic).then(other.cost.cmp(&self.cost))
    }
}

impl PartialOrd for BestPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl BestPath {
    fn new(pos: (i64, i64), end: (i64, i64), cost: i64) -> BestPath {
        BestPath {
            pos: pos,
            cost: cost,
            heuristic: cost + (pos.0 - end.0).abs() + (pos.1 - end.1).abs()
        }
    }
}

fn a_star(start: (i64, i64), end: (i64, i64), corrupted: &HashSet<(i64, i64)>, max_coord: i64) -> Option<i64> {
    let start_path = BestPath::new(start, end, 0);
    let mut open_list: BinaryHeap<BestPath> = BinaryHeap::from([start_path.clone()]);
    let mut open_dict: HashMap<(i64, i64), i64> = HashMap::from([(start_path.pos, start_path.heuristic)]);
    let mut visited: HashMap<(i64, i64), i64> = HashMap::new();

    while let Some(path) = open_list.pop() {
        if let Some(h) = open_dict.remove(&path.pos) {
            if h != path.heuristic {
                continue;
            }
        } else {
            continue;
        }

        if *visited.entry(path.pos).or_insert(path.heuristic) < path.heuristic {
            continue;
        }

        if path.pos == end {
            return Some(path.cost);
        }

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_path = BestPath::new((path.pos.0 + dx, path.pos.1 + dy), end, path.cost + 1);
            if corrupted.contains(&new_path.pos) || new_path.pos.0.min(new_path.pos.1) < 0 || new_path.pos.0.max(new_path.pos.1) > max_coord {
                continue;
            }

            if let Some(exisitng_visited) = visited.get(&new_path.pos) {
                if *exisitng_visited <= new_path.heuristic {
                    continue;
                }
            }

            if *open_dict.entry(new_path.pos).or_insert(new_path.heuristic) < new_path.heuristic {
                continue;
            }
            open_list.push(new_path);
        }
    }
    return None;
}

fn part1(contents: String, max_coord: i64, sim_bytes: usize) -> i64 {
    let corrupted: HashSet<(i64, i64)> = HashSet::from_iter(
        contents.lines().take(sim_bytes).map(|l| {
            let mut nums = l.split(",").map(|n| n.parse::<i64>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
    );

    return a_star((0, 0), (max_coord, max_coord), &corrupted, max_coord).unwrap();
}

fn part2(contents: String, max_coord: i64) -> String {
    let corrupted: Vec<(i64, i64)> = Vec::from_iter(
        contents.lines().map(|l| {
            let mut nums = l.split(",").map(|n| n.parse::<i64>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
    );
    let mut bound = corrupted.len() >> 1;
    let mut adjust = bound >> 1;

    while adjust > 0 {
        if a_star((0, 0), (max_coord, max_coord), &HashSet::from_iter(corrupted[..bound].iter().map(|p| *p)), max_coord).is_some() {
            bound += adjust;
        } else {
            bound -= adjust;
        }
        adjust >>= 1;
    }
    return format!("{},{}", corrupted[bound].0, corrupted[bound].1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents, 6, 12), 22);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents, 6), "6,1".to_string());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "18".to_string();

    let root = env::current_dir().unwrap();
    let path_str = if args.len() > 1 {
        args[1].clone()
    } else if root.ends_with(format!("{}", day)) {
        format!("../../../Inputs/{}_{}.txt", year, day)
    } else {
        format!("/Inputs/{}_{}.txt", year, day)
    };

    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nMinimum steps to exit: {}\nRan in {:.5?}",
        part1(contents.clone(), 70, 1024),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLocation of first byte that prevents exit: {}\nRan in {:.5?}",
        part2(contents.clone(), 70),
        part2_timer.elapsed()
    );
}