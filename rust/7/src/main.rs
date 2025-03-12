use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn can_make(nums: Vec<i64>, goal: i64, p2: bool) -> bool {
    if nums.len() == 1 {
        return goal == nums[0];
    }

    let last_ix = nums.len() - 1;

    if p2 {
        let goal_str = format!("{}", goal);
        let last_str = format!("{}", nums[last_ix]);
        if goal_str.len() >= last_str.len() && goal_str.ends_with(&last_str) && can_make(nums[..last_ix].to_vec(), goal_str[..goal_str.len()-last_str.len()].parse::<i64>().unwrap_or(0), p2) {
            return true;
        }
    }

    if goal % nums[last_ix] == 0 && can_make(nums[..last_ix].to_vec(), goal / nums[last_ix], p2) {
        return true;
    }

    return can_make(nums[..last_ix].to_vec(), goal - nums[last_ix], p2);
}

fn part1(contents: String) -> i64 {
    return contents.lines().filter_map(|line| {
        let mut sides = line.split(": ");
        let goal = sides.next().unwrap().parse::<i64>().unwrap();
        let nums = Vec::from_iter(sides.next().unwrap().split_whitespace().map(|v| v.parse::<i64>().unwrap()));
        if can_make(nums, goal, false) {
            Some(goal)
        } else {
            None
        }
    }).sum::<i64>();
}

fn part2(contents: String) -> i64 {
    return contents.lines().filter_map(|line| {
        let mut sides = line.split(": ");
        let goal = sides.next().unwrap().parse::<i64>().unwrap();
        let nums = Vec::from_iter(sides.next().unwrap().split_whitespace().map(|v| v.parse::<i64>().unwrap()));
        if can_make(nums, goal, true) {
            Some(goal)
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

        assert_eq!(part1(contents), 3749);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 11387);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "7".to_string();

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
        "\nPart 1:\nCalibration result: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nCalibration result: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}