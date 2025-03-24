use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use itertools::Itertools;
use std::iter::zip;

fn part1(contents: String) -> i64 {
    let mut keys: Vec<Vec<i64>> = Vec::new();
    let mut locks: Vec<Vec<i64>> = Vec::new();
    
    for grouping in contents.split("\n\n") {
        let mut new_item: Vec<i64> = vec![0; 5];
        let mut key_lockb: bool = true;
        for line in grouping.lines() {
            for (ix, c) in line.chars().enumerate() {
                if c == '#' {
                    new_item[ix] += 1;
                    key_lockb = true;
                } else {
                    key_lockb = false;
                }
            }
        }
        
        if key_lockb {
            keys.push(new_item);
        } else {
            locks.push(new_item);
        }
    }
    
    let height: i64 = 7;
    return keys.iter().cartesian_product(locks.iter()).filter(|(k, l)| {
        zip(k.into_iter(), l.into_iter()).all(|(kh, lh)| kh + lh <= height)
    }).count() as i64;
}

fn part2(contents: String) -> String {
    return "Christmas has been saved!".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 3);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "25".to_string();

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
        "\nPart 1:\nValid key/lock pairs: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\n{}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}