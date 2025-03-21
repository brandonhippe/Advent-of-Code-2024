use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use cached::proc_macro::cached;

#[cached]
fn num_combos(pattern: String, possible_patterns: Vec<String>, min_len: usize, max_len: usize) -> Option<i64> {
    if pattern.len() == 0 {
        return Some(1);
    }

    let mut num_possible: Option<i64> = None;
    for slice_len in (min_len..=max_len).rev().filter(|s| *s <= pattern.len() && possible_patterns.contains(&pattern[..*s].to_string())) {
        if let Some(num) = num_combos(pattern[slice_len..].to_string(), possible_patterns.clone(), min_len, max_len) {
            num_possible = Some(num_possible.unwrap_or(0) + num);
        }
    }
    return num_possible;
}

fn part1(contents: String) -> i64 {
    let mut input_groups = contents.split("\n\n");
    let possible_patterns = Vec::from_iter(input_groups.next().unwrap().split(", ").map(|p| p.to_string()));
    let min_len = possible_patterns.iter().min_by(|a, b| a.len().cmp(&b.len())).unwrap().len();
    let max_len = possible_patterns.iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap().len();
    return input_groups.next().unwrap().lines().filter_map(|line| num_combos(line.to_string(), possible_patterns.clone(), min_len, max_len)).count() as i64;

}

fn part2(contents: String) -> i64 {
    let mut input_groups = contents.split("\n\n");
    let possible_patterns = Vec::from_iter(input_groups.next().unwrap().split(", ").map(|p| p.to_string()));
    let min_len = possible_patterns.iter().min_by(|a, b| a.len().cmp(&b.len())).unwrap().len();
    let max_len = possible_patterns.iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap().len();
    return input_groups.next().unwrap().lines().filter_map(|line| num_combos(line.to_string(), possible_patterns.clone(), min_len, max_len)).sum::<i64>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 6);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 16);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "19".to_string();

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
        "\nPart 1:\nNumber of possible designs: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nWays to make all possible designs: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}