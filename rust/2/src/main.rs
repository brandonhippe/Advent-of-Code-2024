use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashSet;

fn part1(contents: String) -> i64 {
    return contents.lines().filter_map(|line| {
        let nums: Vec<i64> = Vec::from_iter(line.split_whitespace().map(|n| n.parse::<i64>().unwrap()));
        let diff_set: HashSet<i64> = HashSet::from_iter(nums.windows(2).map(|ns| ns[1] - ns[0]));

        if diff_set.intersection(&HashSet::from([1, 2, 3])).count() == diff_set.len() || diff_set.intersection(&HashSet::from([-1, -2, -3])).count() == diff_set.len() {
            Some(nums)
        } else {
            None
        }
    }).count() as i64;
}

fn part2(contents: String) -> i64 {
    return contents.lines().filter_map(|line| {
        let nums: Vec<i64> = Vec::from_iter(line.split_whitespace().map(|n| n.parse::<i64>().unwrap()));
        for rem_ix in 0..nums.len() {
            let rem_nums: Vec<i64> = Vec::from_iter(nums.iter().enumerate().filter_map(|(ix, n)| {
                if ix == rem_ix {
                    None
                } else {
                    Some(*n)
                }
            }));
            let diff_set: HashSet<i64> = HashSet::from_iter(rem_nums.windows(2).map(|ns| ns[1] - ns[0]));
    
            if diff_set.intersection(&HashSet::from([1, 2, 3])).count() == diff_set.len() || diff_set.intersection(&HashSet::from([-1, -2, -3])).count() == diff_set.len() {
                return Some(nums);
            }
        }

        None
    }).count() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 2);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 4);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "2".to_string();

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
        "\nPart 1:\nSafe reports: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSafe reports: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}