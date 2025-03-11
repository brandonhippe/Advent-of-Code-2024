use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use regex::Regex;

fn part1(contents: String) -> i64 {
    let mul_re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();
    return mul_re.captures_iter(&contents).map(|caps| {
        caps.get(1).unwrap().as_str().parse::<i64>().unwrap() * caps.get(2).unwrap().as_str().parse::<i64>().unwrap()
    }).sum::<i64>();
}

fn part2(contents: String) -> i64 {
    let mul_re = Regex::new(r"do\(\)|don't\(\)|mul\((-?\d+),(-?\d+)\)").unwrap();
    return mul_re.captures_iter(&contents).fold((0, true), |(sum, enabled), caps| {
        match caps.get(0).unwrap().as_str() {
            "do()" => (sum, true),
            "don't()" => (sum, false),
            _ => (sum + ((enabled as i64) * caps.get(1).unwrap().as_str().parse::<i64>().unwrap() * caps.get(2).unwrap().as_str().parse::<i64>().unwrap()), enabled)
        }
    }).0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        assert_eq!(part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()), 161);
    }

    #[test]
    fn p2_test() {
        assert_eq!(part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string()), 48);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "3".to_string();

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
        "\nPart 1:\nSum of multiplications: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSum of enabled multiplications: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}