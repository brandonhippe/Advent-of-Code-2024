use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashSet, HashMap, VecDeque};
use itertools::Itertools;

fn mix_prune(secret: i64, val: i64) -> i64 {
    (secret ^ val) & 0xffffff
}

fn part1(contents: String) -> i64 {
    return contents.lines().map(|line| {
        let mut n = line.parse::<i64>().unwrap();
        for _ in 0..2000 {
            n = mix_prune(n, n << 6);
            n = mix_prune(n, n >> 5);
            n = mix_prune(n, n << 11);
        }

        n
    }).sum::<i64>();
}

fn part2(contents: String) -> i64 {
    let mut bananas_gained: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
    for line in contents.lines() {
        let mut n = line.parse::<i64>().unwrap();
        let mut p_b = n % 10;
        let mut last_4: VecDeque<i64> = VecDeque::new();
        let mut used: HashSet<(i64, i64, i64, i64)> = HashSet::new();

        for _ in 0..2000 {
            n = mix_prune(n, n << 6);
            n = mix_prune(n, n >> 5);
            n = mix_prune(n, n << 11);

            let b = n % 10;
            let diff = b - p_b;
            p_b = b;

            last_4.push_front(diff);
            last_4.truncate(4);
            if last_4.len() == 4 {
                let key = last_4.iter().map(|v| *v).collect_tuple().unwrap();
                if used.contains(&key) {
                    continue;
                }

                *bananas_gained.entry(key).or_insert(0) += b;
                used.insert(key);
            }
        }
    }

    return *bananas_gained.values().max().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example1.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 37327623);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example2.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 23);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "22".to_string();

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
        "\nPart 1:\nSum of secret numbers: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMost bananas: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}