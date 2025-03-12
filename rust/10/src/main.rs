use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashMap, VecDeque};

fn trailhead_counts(start_pos: (i64, i64), heights: &HashMap<(i64, i64), i64>) -> Vec<i64> {
    let mut to_check: VecDeque<(i64, i64)> = VecDeque::from([start_pos]);
    let mut count: HashMap<(i64, i64), i64> = HashMap::new();

    while let Some(pos) = to_check.pop_front() {
        *count.entry(pos).or_insert(0) += 1;
        let curr_height: i64 = *heights.get(&pos).unwrap();

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos: (i64, i64) = (pos.0 + dx, pos.1 + dy);
            if heights.get(&new_pos).unwrap_or(&-1) - curr_height == 1 {
                to_check.push_back(new_pos);
            }
        }
    }

    return Vec::from_iter(count.iter().filter_map(|(k, v)| {
        if *heights.get(k).unwrap() == 9 {
            Some(*v)
        } else {
            None
        }
    }));
}

fn part1(contents: String) -> i64 {
    let mut heights: HashMap<(i64, i64), i64> = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            heights.insert((x as i64, y as i64), (c as u32 - '0' as u32) as i64);
        }
    }

    return heights.iter().filter_map(|(pos, h)| {
        if *h == 0 {
            Some(trailhead_counts(*pos, &heights).len() as i64)
        } else {
            None
        }
    }).sum::<i64>();
}

fn part2(contents: String) -> i64 {
    let mut heights: HashMap<(i64, i64), i64> = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            heights.insert((x as i64, y as i64), (c as u32 - '0' as u32) as i64);
        }
    }

    return heights.iter().filter_map(|(pos, h)| {
        if *h == 0 {
            Some(trailhead_counts(*pos, &heights).iter().sum::<i64>())
        } else {
            None
        }
    }).sum::<i64>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 36);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 81);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "10".to_string();

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
        "\nPart 1:\nTrailhead scores: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nTrailhead ratings: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}