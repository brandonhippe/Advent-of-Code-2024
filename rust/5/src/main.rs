use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use std::cmp::Ordering;

fn part1(contents: String) -> i64 {
    let mut break_split = contents.split("\n\n");
    let mut rules: HashMap<i64, Vec<i64>> = HashMap::new();
    for line in break_split.next().unwrap().lines() {
        let mut nums = line.split("|");
        let left = nums.next().unwrap().parse::<i64>().unwrap();
        let right = nums.next().unwrap().parse::<i64>().unwrap();
        rules.entry(left).or_insert(Vec::new()).push(right);
    }

    return break_split.next().unwrap().lines().filter_map(|line| {
        let list: Vec<i64> = Vec::from_iter(line.split(',').map(|n| n.parse::<i64>().unwrap()));
        let mut sorted_list = list.clone();
        sorted_list.sort_by(|a, b| {
            if rules.entry(*a).or_insert(Vec::new()).contains(&b) {
                Ordering::Less
            } else if rules.entry(*b).or_insert(Vec::new()).contains(&a) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        if sorted_list == list {
            Some(sorted_list[list.len() / 2])
        } else {
            None
        }
    }).sum::<i64>();
}

fn part2(contents: String) -> i64 {
    let mut break_split = contents.split("\n\n");
    let mut rules: HashMap<i64, Vec<i64>> = HashMap::new();
    for line in break_split.next().unwrap().lines() {
        let mut nums = line.split("|");
        let left = nums.next().unwrap().parse::<i64>().unwrap();
        let right = nums.next().unwrap().parse::<i64>().unwrap();
        rules.entry(left).or_insert(Vec::new()).push(right);
    }

    return break_split.next().unwrap().lines().filter_map(|line| {
        let list: Vec<i64> = Vec::from_iter(line.split(',').map(|n| n.parse::<i64>().unwrap()));
        let mut sorted_list = list.clone();
        sorted_list.sort_by(|a, b| {
            if rules.entry(*a).or_insert(Vec::new()).contains(&b) {
                Ordering::Less
            } else if rules.entry(*b).or_insert(Vec::new()).contains(&a) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        if sorted_list == list {
            None
        } else {
            Some(sorted_list[list.len() / 2])
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

        assert_eq!(part1(contents), 143);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 123);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "5".to_string();

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
        "\nPart 1:\nSum of correctly ordered middle pages: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSum of corrected order middle pages: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}