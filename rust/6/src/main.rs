use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use std::thread;
use std::thread::available_parallelism;

fn guard_movement(pos: (i64, i64), facing: (i64, i64), spaces: &HashMap<(i64, i64), bool>, visited: &mut HashSet<((i64, i64), (i64, i64))>, add_obstacles: &mut HashSet<(i64, i64)>) -> ((i64, i64), (i64, i64), Option<bool>) {
    if !spaces.contains_key(&pos) {
        return (pos, facing, Some(false));
    }

    if visited.contains(&(pos, facing)) {
        return (pos, facing, Some(true));
    }
    visited.insert((pos, facing));

    let forward: (i64, i64) = (pos.0 + facing.0, pos.1 + facing.1);
    let right: (i64, i64) = (-facing.1, facing.0);
    
    return if *spaces.get(&forward).unwrap_or(&true) {
        add_obstacles.insert(forward);
        (forward, facing, None)
    } else {
        (pos, right, None)
    };
}

fn part1(contents: String) -> i64 {
    let mut start_pos: Option<((i64, i64), (i64, i64))> = None;
    let mut spaces: HashMap<(i64, i64), bool> = HashMap::new();
    
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            spaces.insert((x as i64, y as i64), c != '#');
            match c {
                '^' => {start_pos = Some(((x as i64, y as i64), (0, -1)));},
                'v' => {start_pos = Some(((x as i64, y as i64), (0, 1)));},
                '>' => {start_pos = Some(((x as i64, y as i64), (1, 0)));},
                '<' => {start_pos = Some(((x as i64, y as i64), (-1, 0)));},
                _ => ()
            }
        }
    }
    
    let mut visited: HashSet<((i64, i64), (i64, i64))> = HashSet::new();
    let (mut pos, mut facing) = start_pos.unwrap();
    let mut finished: Option<bool> = None;

    while finished.is_none() {
        (pos, facing, finished) = guard_movement(pos, facing, &spaces, &mut visited, &mut HashSet::new());
    }

    let actual_visited: HashSet<(i64, i64)> = HashSet::from_iter(
        visited.iter().map(|(p, _f)| *p)
    );
    return actual_visited.len() as i64;
}

fn part2(contents: String) -> i64 {
    let mut start_pos: Option<((i64, i64), (i64, i64))> = None;
    let mut spaces: HashMap<(i64, i64), bool> = HashMap::new();
    
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            spaces.insert((x as i64, y as i64), c != '#');
            match c {
                '^' => {start_pos = Some(((x as i64, y as i64), (0, -1)));},
                'v' => {start_pos = Some(((x as i64, y as i64), (0, 1)));},
                '>' => {start_pos = Some(((x as i64, y as i64), (1, 0)));},
                '<' => {start_pos = Some(((x as i64, y as i64), (-1, 0)));},
                _ => ()
            }
        }
    }
    
    let mut visited: HashSet<((i64, i64), (i64, i64))> = HashSet::new();
    let (mut pos, mut facing) = start_pos.unwrap();
    let mut finished: Option<bool> = None;
    let mut check_obstacles: HashSet<(i64, i64)> = HashSet::new();

    while finished.is_none() {
        (pos, facing, finished) = guard_movement(pos, facing, &spaces, &mut visited, &mut check_obstacles);
    }

    let num_cpus = available_parallelism().unwrap().get();
    let num_per_thread = (check_obstacles.len() / num_cpus) + 1;
    let threads: Vec<_> = (0..num_cpus).map(|n| {
        let spaces = spaces.clone();
        let check_obstacles = check_obstacles.clone();
        let start_pos = start_pos.unwrap();
        
        thread::spawn(move || {
            let mut count: i64 = 0;
            for obstacle_pos in check_obstacles.iter().skip(n * num_per_thread).take(num_per_thread) {
                let mut test_spaces = spaces.clone();
                test_spaces.insert(*obstacle_pos, false);
                let (mut test_pos, mut test_facing) = start_pos;
                let mut test_finished = None;
                let mut test_visited: HashSet<((i64, i64), (i64, i64))> = HashSet::new();
                while test_finished.is_none() {
                    (test_pos, test_facing, test_finished) = guard_movement(test_pos, test_facing, &test_spaces, &mut test_visited, &mut HashSet::new());
                }
        
                count += test_finished.unwrap() as i64;
            }

            count
        })
    }).collect();

    return threads.into_iter().map(|t| t.join().unwrap()).sum::<i64>();
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
        "\nPart 2:\nPositions to cause guard to loop: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}