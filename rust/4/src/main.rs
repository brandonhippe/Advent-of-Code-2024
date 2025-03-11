use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use itertools::Itertools;

fn part1(contents: String) -> i64 {
    let char_map: HashMap<(i64, i64), char> = HashMap::from_iter(contents.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, c)| ((x as i64, y as i64), c))
    }));

    let mut test: Vec<((i64, i64), (i64, i64))> = Vec::from_iter(char_map.iter().filter_map(|(k, v)| {
        if *v == 'X' {
            Some((-1..=1).cartesian_product(-1..=1).filter_map(|(x, y)| {
                if x != 0 || y != 0 {
                    Some(((k.0 + x, k.1 + y), (x, y)))
                } else {
                    None
                }
            }))
        } else {
            None
        }
    }).flatten());

    for next_char in "MAS".chars() {
        test = Vec::from_iter(test.iter().filter_map(|(pos, dir)| {
            if *char_map.get(&pos).unwrap_or(&' ') == next_char {
                Some(((pos.0 + dir.0, pos.1 + dir.1), *dir))
            } else {
                None
            }
        }))
    }

    return test.len() as i64;
}

fn part2(contents: String) -> i64 {
    let char_map: HashMap<(i64, i64), char> = HashMap::from_iter(contents.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, c)| ((x as i64, y as i64), c))
    }));

    let mut test: Vec<((i64, i64), (i64, i64))> = Vec::from_iter(char_map.iter().filter_map(|(k, v)| {
        if *v == 'M' {
            Some((-1..=1).cartesian_product(-1..=1).filter_map(|(x, y)| {
                if (x + y) % 2 == 0 && x != 0 && y != 0 {
                    Some(((k.0 + x, k.1 + y), (x, y)))
                } else {
                    None
                }
            }))
        } else {
            None
        }
    }).flatten());

    for next_char in "AS".chars() {
        test = Vec::from_iter(test.iter().filter_map(|(pos, dir)| {
            if *char_map.get(&pos).unwrap_or(&' ') == next_char {
                Some(((pos.0 + dir.0, pos.1 + dir.1), *dir))
            } else {
                None
            }
        }))
    }

    let mut centers: HashMap<(i64, i64), i64> = HashMap::new();
    for ((x, y), (dx, dy)) in test.iter() {
        *centers.entry((x - (2*dx), y - (2*dy))).or_insert(0) += 1;
    }
    
    return centers.iter().filter(|(_, v)| {
        **v >= 2
    }).count() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 18);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 9);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "4".to_string();

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
        "\nPart 1:\nXMAS instances: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nX-MAS instances: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}