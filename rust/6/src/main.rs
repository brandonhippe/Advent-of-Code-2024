use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet};

fn detect_loop(start_pos: (i64, i64, i64, i64), spaces: &HashMap<(i64, i64), bool>, already_visited: HashSet<(i64, i64, i64, i64)>) -> bool {
    let mut visited: HashSet<(i64, i64, i64, i64)> = already_visited.clone();
    let mut pos = start_pos;

    while spaces.contains_key(&(pos.0, pos.1)) {
        if visited.contains(&pos) {
            return true;
        }
        visited.insert(pos);

        pos = if *spaces.get(&(pos.0 + pos.2, pos.1 + pos.3)).unwrap_or(&true) {
            (pos.0 + pos.2, pos.1 + pos.3, pos.2, pos.3)
        } else {
            (pos.0, pos.1, -pos.3, pos.2)
        };
    }

    return false;
}

fn part1(contents: String) -> i64 {
    let mut start_pos: Option<(i64, i64)> = None;
    let mut spaces: HashMap<(i64, i64), bool> = HashMap::new();
    
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            spaces.insert((x as i64, y as i64), c != '#');
            if c == '^' {
                start_pos = Some((x as i64, y as i64));
            }
        }
    }
    assert!(start_pos.is_some());
    
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut pos = start_pos.unwrap();
    let mut facing: (i64, i64) = (0, -1);

    while spaces.contains_key(&pos) {
        visited.insert(pos);
        if *spaces.get(&(pos.0 + facing.0, pos.1 + facing.1)).unwrap_or(&true) {
            pos = (pos.0 + facing.0, pos.1 + facing.1);
        } else {
            facing = (-facing.1, facing.0);
        }
    }
    return visited.len() as i64;
}

fn part2(contents: String) -> i64 {
    let mut start_pos: Option<(i64, i64)> = None;
    let mut spaces: HashMap<(i64, i64), bool> = HashMap::new();
    
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            spaces.insert((x as i64, y as i64), c != '#');
            if c == '^' {
                start_pos = Some((x as i64, y as i64));
            }
        }
    }
    assert!(start_pos.is_some());
    
    let mut visited: HashSet<(i64, i64, i64, i64)> = HashSet::new();
    let start = start_pos.unwrap();
    let mut pos: (i64, i64, i64, i64) = (start.0, start.1, 0, -1);

    let mut count: HashSet<(i64, i64)> = HashSet::new();
    while spaces.contains_key(&(pos.0, pos.1)) {
        visited.insert(pos);

        pos = if *spaces.get(&(pos.0 + pos.2, pos.1 + pos.3)).unwrap_or(&true) {
            let mut test_spaces = spaces.clone();
            test_spaces.insert((pos.0 + pos.2, pos.1 + pos.3), false);
            if detect_loop((pos.0, pos.1, -pos.3, pos.2), &test_spaces, visited.clone()) {
                count.insert((pos.0 + pos.2, pos.1 + pos.3));
            }
            (pos.0 + pos.2, pos.1 + pos.3, pos.2, pos.3)
        } else {
            (pos.0, pos.1, -pos.3, pos.2)
        };
    }

    return count.len() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 41);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 6);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "6".to_string();

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
        "\nPart 1:\nPositions visited: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\n {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}