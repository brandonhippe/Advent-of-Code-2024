use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn count_blinks(stones: HashMap<i64, i64>, blinks: i64) -> i64 {
    if blinks == 0 {
        return stones.values().sum::<i64>();
    }

    let mut next_stones: HashMap<i64, i64> = HashMap::new();
    for (stone_val, amt) in stones.iter() {
        let val_str = format!("{}", stone_val);
        for next_val in if *stone_val == 0 {
            vec![1]
        } else if val_str.len() % 2 == 0 {
            vec![val_str[..val_str.len() / 2].parse::<i64>().unwrap(), val_str[val_str.len() / 2..].parse::<i64>().unwrap()]
        } else {
            vec![stone_val * 2024]
        } {
            *next_stones.entry(next_val).or_insert(0) += amt;
        }
    }

    return count_blinks(next_stones, blinks - 1);
}

fn part1(contents: String, blinks: i64) -> i64 {
    let mut stone_map: HashMap<i64, i64> = HashMap::new();
    for n in contents.lines().next().unwrap().split_whitespace() {
        *stone_map.entry(n.parse::<i64>().unwrap()).or_insert(0) += 1;
    }
    return count_blinks(stone_map, blinks);
}

fn part2(contents: String) -> i64 {
    let mut stone_map: HashMap<i64, i64> = HashMap::new();
    for n in contents.lines().next().unwrap().split_whitespace() {
        *stone_map.entry(n.parse::<i64>().unwrap()).or_insert(0) += 1;
    }
    return count_blinks(stone_map, 75);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents.clone(), 6), 22);
        assert_eq!(part1(contents, 25), 55312);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "11".to_string();

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
        "\nPart 1:\nBlinks: {}\nRan in {:.5?}",
        part1(contents.clone(), 25),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nBlinks: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}