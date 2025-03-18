use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashSet, HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, PartialEq)]
struct BestPath {
    pos: (i64, i64),
    direction: (i64, i64),
    cost: i64,
    heuristic: i64,
    on_path: HashSet<(i64, i64)>,
}

impl BestPath {
    fn new(pos: (i64, i64), end: (i64, i64), direction: (i64, i64), cost: i64) -> BestPath {
        BestPath {
            pos: pos,
            direction: direction,
            cost: cost,
            heuristic: cost + (pos.0 - end.0).abs() + (pos.1 - end.1).abs(),
            on_path: HashSet::from([pos]),
        }
    }
    
    fn from(from_path: &BestPath, end: (i64, i64), new_direction: (i64, i64)) -> BestPath {
        let new_pos = (from_path.pos.0 + from_path.direction.0, from_path.pos.1 + from_path.direction.1);
        let new_cost = from_path.cost + if from_path.direction == new_direction {1} else {1001};
        let mut new_path = from_path.on_path.clone();
        new_path.insert(new_pos.clone());

        BestPath {
            pos: new_pos,
            direction: new_direction,
            cost: new_cost,
            heuristic: new_cost + (new_pos.0 - end.0).abs() + (new_pos.1 - end.1).abs(),
            on_path: new_path,
        }
    }
}

impl Hash for BestPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.direction.hash(state);
    }
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

fn a_star(start: (i64, i64), end: (i64, i64), available_spaces: &HashSet<(i64, i64)>) -> Option<BestPath> {
    let mut open_list: BinaryHeap<BestPath> = BinaryHeap::new();
    let mut open_dict: HashMap<((i64, i64), (i64, i64)), BestPath> = HashMap::new();
    let mut visited: HashMap<((i64, i64), (i64, i64)), BestPath> = HashMap::new();

    for (direction, cost) in [((1, 0), 0), ((0, -1), 1000), ((0, 1), 1000)] {
        if !available_spaces.contains(&(start.0 + direction.0, start.1 + direction.1)) {
            continue;
        }
        let start_pos: BestPath = BestPath::new(start, end, direction, cost);
        open_dict.insert((start_pos.pos, start_pos.direction), start_pos.clone());
        open_list.push(start_pos);
    }

    let mut min_path = BestPath::new(end, end, (0, 0), i64::MAX);

    while let Some(mut path) = open_list.pop() {
        if let Some(updated_path) = open_dict.remove(&(path.pos, path.direction)) {
            path = updated_path;
        } else {
            continue;
        }
        
        if path.pos == end {
            min_path = match min_path.cmp(&path) {
                Ordering::Less => path,
                Ordering::Equal => {
                    BestPath {
                        pos: min_path.pos,
                        direction: min_path.direction,
                        cost: min_path.cost,
                        heuristic: min_path.heuristic,
                        on_path: HashSet::from_iter(min_path.on_path.union(&path.on_path).map(|v| *v)),
                    }
                }
                _ => min_path
            };
            continue;
        }
        
        let mut visited_entry = visited.entry((path.pos, path.direction)).or_insert(path.clone());
        match visited_entry.heuristic.cmp(&path.heuristic) {
            Ordering::Less => {continue;},
            Ordering::Equal => {
                let comb_on_path: HashSet<(i64, i64)> = HashSet::from_iter(visited_entry.on_path.union(&path.on_path).map(|v| *v));
                visited_entry.on_path = comb_on_path.clone();
                path.on_path = comb_on_path;
            },
            Ordering::Greater => {
                *visited_entry = path.clone();
            }
        }

        for new_direction in [path.direction, (path.direction.1, path.direction.0), (-path.direction.1, -path.direction.0)] {
            let mut new_path = BestPath::from(&path, end, new_direction);
            if new_path.pos != end && !available_spaces.contains(&(new_path.pos.0 + new_path.direction.0, new_path.pos.1 + new_path.direction.1)) {
                continue;
            }

            if let Some(visited_path) = visited.get(&(new_path.pos, new_path.direction)) {
                if visited_path.heuristic < new_path.heuristic {
                    continue;
                }
            }

            if let Some(open_entry) = open_dict.get(&(new_path.pos, new_path.direction)) {
                if open_entry.heuristic < new_path.heuristic {
                    continue;
                }
                new_path.on_path = HashSet::from_iter(new_path.on_path.union(&open_entry.on_path).map(|v| *v));
            }
            
            open_dict.insert((new_path.pos, new_path.direction), new_path.clone());
            open_list.push(new_path);
        }
    }

    return if min_path.cost < i64::MAX {
        Some(min_path)
    } else {
        None
    };
}

fn part1(contents: String) -> i64 {
    let mut start_pos: Option<(i64, i64)> = None;
    let mut end_pos: Option<(i64, i64)> = None;
    let mut available_spaces: HashSet<(i64, i64)> = HashSet::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate().filter(|(_ix, c)| *c != '#') {
            let pos = (x as i64, y as i64);
            match c {
                'S' => {start_pos = Some(pos);},
                'E' => {end_pos = Some(pos);},
                '.' => (),
                _ => panic!("Unknown character at position ({}, {}): {}", x, y, c)
            }
            available_spaces.insert(pos);
        }
    }

    return a_star(start_pos.unwrap(), end_pos.unwrap(), &available_spaces).unwrap().cost;
}

fn part2(contents: String) -> i64 {
    let mut start_pos: Option<(i64, i64)> = None;
    let mut end_pos: Option<(i64, i64)> = None;
    let mut available_spaces: HashSet<(i64, i64)> = HashSet::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate().filter(|(_ix, c)| *c != '#') {
            let pos = (x as i64, y as i64);
            match c {
                'S' => {start_pos = Some(pos);},
                'E' => {end_pos = Some(pos);},
                '.' => (),
                _ => panic!("Unknown character at position ({}, {}): {}", x, y, c)
            }
            available_spaces.insert(pos);
        }
    }

    return a_star(start_pos.unwrap(), end_pos.unwrap(), &available_spaces).unwrap().on_path.len() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents), 7036);

        let contents =
            fs::read_to_string("example2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents), 11048);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents), 45);

        let contents =
            fs::read_to_string("example2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents), 64);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "16".to_string();

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
        "\nPart 1:\nLowest Score: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nUnique tiles on any best path: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}