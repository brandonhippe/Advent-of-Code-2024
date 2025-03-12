use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn part1(contents: String) -> i64 {
    let mut antennas: HashMap<char, HashSet<(i64, i64)>> = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_insert(HashSet::new()).insert((x as i64, y as i64));
            }
        }
    }
    
    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();
    for antenna_group in antennas.values() {
        for comb in antenna_group.iter().combinations(2) {
            let a1 = comb[0];
            let a2 = comb[1];
            let dx = a2.0 - a1.0;
            let dy = a2.1 - a1.1;
            antinodes.insert((a1.0 - dx, a1.1 - dy));
            antinodes.insert((a2.0 + dx, a2.1 + dy));
        }
    }

    return antinodes.iter().filter(|n| {
        n.0 >= 0 && n.0 < contents.lines().count() as i64 &&
        n.1 >= 0 && n.1 < contents.lines().count() as i64
    }).count() as i64;
}

fn part2(contents: String) -> i64 {
    let mut antennas: HashMap<char, HashSet<(i64, i64)>> = HashMap::new();
    let max_size = contents.lines().count() as i64;
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_insert(HashSet::new()).insert((x as i64, y as i64));
            }
        }
    }
    
    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();
    for antenna_group in antennas.values() {
        for comb in antenna_group.iter().combinations(2) {
            let mut a1: (i64, i64) = *comb[0];
            let mut a2: (i64, i64) = *comb[1];
            let dx = a2.0 - a1.0;
            let dy = a2.1 - a1.1;

            while a1.0 >= 0 && a1.0 < max_size && a1.1 >= 0 && a1.1 < max_size {
                antinodes.insert(a1.clone());
                a1 = (a1.0 - dx, a1.1 - dy);
            }

            while a2.0 >= 0 && a2.0 < max_size && a2.1 >= 0 && a2.1 < max_size {
                antinodes.insert(a2.clone());
                a2 = (a2.0 + dx, a2.1 + dy);
            }
        }
    }

    return antinodes.len() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 14);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 34);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "8".to_string();

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
        "\nPart 1:\nUnique antinodes: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nUnique antinodes: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}