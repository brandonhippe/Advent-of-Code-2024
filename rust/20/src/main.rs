use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashSet, HashMap, VecDeque};

fn min_path(start: (i64, i64), end: (i64, i64), available: &HashSet<(i64, i64)>) -> Option<Vec<(i64, i64)>> {
    let mut checking: VecDeque<((i64, i64), Vec<(i64, i64)>)> = VecDeque::from([(
        start,
        Vec::from([start]),
    )]);

    while let Some((pos, path)) = checking.pop_front() {
        if pos == end {
            return Some(path);
        }


        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let next_pos: (i64, i64) = (pos.0 + dx, pos.1 + dy);
            if path.contains(&next_pos) || !available.contains(&next_pos) {
                continue;
            }
            let mut next_path = path.clone();
            next_path.push(next_pos);
            checking.push_back((next_pos, next_path));
        }
    }

    return None;
}

fn count_cheats(min_path: Vec<(i64, i64)>, available: &HashSet<(i64, i64)>, cheat_len: i64) -> HashMap<i64, i64> {
    let mut cheats: HashMap<i64, i64> = HashMap::new();
    let path_ixs: HashMap<(i64, i64), usize> = HashMap::from_iter(
        min_path.iter().enumerate().map(|(ix, pos)| (*pos, ix))
    );

    for (start_ix, start_pos) in min_path.iter().enumerate() {
        for dx in -cheat_len..=cheat_len {
            let y_lim = cheat_len - dx.abs();
            for dy in -y_lim..=y_lim {
                let end_pos = (start_pos.0 + dx, start_pos.1 + dy);
                if !available.contains(&end_pos) || *path_ixs.get(&end_pos).unwrap_or(&0) <= start_ix {
                    continue;
                }
                *cheats.entry((*path_ixs.get(&end_pos).unwrap() as i64) - (start_ix as i64) - dx.abs() - dy.abs()).or_insert(0) += 1;
            }
        }
    }

    return cheats;
}

fn part1(contents: String, min_saved: i64) -> i64 {
    let mut start_pos: Option<(i64, i64)> = None;
    let mut end_pos: Option<(i64, i64)> = None;
    let mut available: HashSet<(i64, i64)> = HashSet::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as i64, y as i64);
            match c {
                'S' => {start_pos = Some(pos); available.insert(pos);},
                'E' => {end_pos = Some(pos); available.insert(pos);},
                '.' => {available.insert(pos);},
                '#' => (),
                _ => panic!("Unknown map character: {}", c)
            }
        }
    }

    return count_cheats(min_path(start_pos.unwrap(), end_pos.unwrap(), &available).unwrap(), &available, 2).iter().filter_map(|(saved, amt)| {
        if *saved >= min_saved {
            Some(amt)
        } else {
            None
        }
    }).sum::<i64>();
}

fn part2(contents: String, min_saved: i64) -> i64 {
    let mut start_pos: Option<(i64, i64)> = None;
    let mut end_pos: Option<(i64, i64)> = None;
    let mut available: HashSet<(i64, i64)> = HashSet::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as i64, y as i64);
            match c {
                'S' => {start_pos = Some(pos); available.insert(pos);},
                'E' => {end_pos = Some(pos); available.insert(pos);},
                '.' => {available.insert(pos);},
                '#' => (),
                _ => panic!("Unknown map character: {}", c)
            }
        }
    }

    return count_cheats(min_path(start_pos.unwrap(), end_pos.unwrap(), &available).unwrap(), &available, 20).iter().filter_map(|(saved, amt)| {
        if *saved >= min_saved {
            Some(amt)
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

        assert_eq!(part1(contents, 1), 44);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents, 50), 285);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "20".to_string();

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
        "\nPart 1:\nNumber of cheats: {}\nRan in {:.5?}",
        part1(contents.clone(), 100),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nNumber of cheats: {}\nRan in {:.5?}",
        part2(contents.clone(), 100),
        part2_timer.elapsed()
    );
}