use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::iter::zip;
use std::collections::HashMap;

fn part1(contents: String) -> i64 {
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();
    for line in contents.lines() {
        let mut split = line.split_whitespace();
        left.push(split.next().unwrap().parse::<i64>().unwrap());
        right.push(split.next().unwrap().parse::<i64>().unwrap());
    }
    
    left.sort();
    right.sort();
    return zip(left, right).map(|(l, r)| (l-r).abs()).sum::<i64>();
}

fn part2(contents: String) -> i64 {
    let mut right_counts: HashMap<i64, i64> = HashMap::new();
    for line in contents.lines() {
        let r_num = line.split_whitespace().last().unwrap().parse::<i64>().unwrap();
        *right_counts.entry(r_num).or_insert(0) += 1;
    }
    return contents.lines().map(|line| {
        let l_num = line.split_whitespace().next().unwrap().parse::<i64>().unwrap();
        l_num * right_counts.get(&l_num).unwrap_or(&0)
    }).sum::<i64>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 11);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 31);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "1".to_string();

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
        "\nPart 1:\nDistance: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSimiarity Score: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}