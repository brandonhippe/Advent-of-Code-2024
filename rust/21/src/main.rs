use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use itertools::Itertools;
use cached::proc_macro::cached;

const DIRECTIONAL: &str = " ^A\n<v>";
const NUMERIC: &str = "789\n456\n123\n 0A";

fn reduce_path<'a>(a: &'a str, b: &'a str) -> String {
    let mut directions: Vec<(i64, i64)> = Vec::from_iter(
        format!("{}{}", a, b).chars().map(|c| 
            match c {
                'v' => (0, 1),
                '>' => (1, 0),
                '^' => (0, -1),
                '<' => (-1, 0),
                _ => panic!("Unknown direction")
            }
    ));
    while let Some(ix) = directions.windows(2).enumerate().filter_map(|(ix, dirs)| {
        if (dirs[0].0 + dirs[1].0) == 0 && dirs[0].1 + dirs[1].1 == 0 {
            Some(ix)
        } else {
            None
        }
    }).next() {
        directions.remove(ix);
        directions.remove(ix);
    }

    return directions.iter().map(|d| {
        match d {
            (0, 1) => "v".to_string(),
            (1, 0) => ">".to_string(),
            (0, -1) => "^".to_string(),
            (-1, 0) => "<".to_string(),
            _ => panic!("Unknown direction")
        }
    }).reduce(|s, d| {
        format!("{}{}", s, d)
    }).unwrap();
}

#[cached]
fn keypad_mapping(keypad_str: String) -> HashMap<char, HashMap<char, HashSet<String>>> {
    let mut mapping: HashMap<char, HashMap<char, HashSet<String>>> = HashMap::new();
    let mut positions: HashMap<(i64, i64), char> = HashMap::new();
    for (y, line) in keypad_str.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != ' ' {
                let pos = (x as i64, y as i64);
                positions.insert(pos, c);
                mapping.entry(c).or_insert(HashMap::from([(c, HashSet::from(["".to_string()]))]));

                for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let test_pos = (pos.0 + dx, pos.1 + dy);
                    if positions.contains_key(&test_pos) {
                        let test_c = *positions.get(&test_pos).unwrap();
                        mapping.entry(c).or_insert(HashMap::new()).entry(test_c).or_insert(HashSet::new()).insert(
                            match (dx, dy) {
                                (0, 1) => "v".to_string(),
                                (1, 0) => ">".to_string(),
                                (0, -1) => "^".to_string(),
                                (-1, 0) => "<".to_string(),
                                _ => panic!("Unknown direction")
                            }
                        );

                        mapping.entry(test_c).or_insert(HashMap::new()).entry(c).or_insert(HashSet::new()).insert(
                            match (-dx, -dy) {
                                (0, 1) => "v".to_string(),
                                (1, 0) => ">".to_string(),
                                (0, -1) => "^".to_string(),
                                (-1, 0) => "<".to_string(),
                                _ => panic!("Unknown direction")
                            }
                        );
                    }
                }
            }
        }
    }

    for k in mapping.clone().keys().collect::<Vec<_>>() {
        for i in mapping.clone().keys().filter(|i| *i != k && mapping.get(i).unwrap().contains_key(&k)).collect::<Vec<_>>() {
            for j in mapping.clone().keys().filter(|j| *j != i && *j != k && mapping.get(&k).unwrap().contains_key(j)).collect::<Vec<_>>() {
                let mut combos: HashSet<String> = HashSet::from_iter(
                    mapping.get(i).unwrap().get(k).unwrap().iter().cartesian_product(mapping.get(k).unwrap().get(j).unwrap().iter())
                        .map(|(a, b)| reduce_path(a, b))
                );
                let min_len = combos.iter().min_by(|a, b| a.len().cmp(&b.len())).unwrap().len();
                combos = HashSet::from_iter(
                    combos.iter().filter(|s| s.len() == min_len).cloned()
                );

                let mut i_j: HashSet<String> = mapping.get(i).unwrap().get(j).unwrap_or(&HashSet::from([keypad_str.to_string()])).clone();
                i_j = match combos.iter().next().unwrap().len().cmp(&i_j.iter().min_by(|a, b| a.len().cmp(&b.len())).unwrap().len()) {
                    Ordering::Less => combos,
                    Ordering::Equal => HashSet::from_iter(i_j.union(&combos).cloned()),
                    Ordering::Greater => i_j,
                };
                mapping.entry(*i).or_insert(HashMap::new()).insert(*j, i_j);
            }
        }
    }

    return mapping;
}

#[cached]
fn shortest_sequence(start_key: char, end_key: char, rem_directional: i64) -> i64 {
    if rem_directional == 0 {
        1
    } else {
        let directional_mapping = keypad_mapping(DIRECTIONAL.to_string());
        let pos_keycodes: HashSet<String> = HashSet::from_iter(
            directional_mapping.get(&start_key).unwrap().get(&end_key).unwrap().iter().map(|s| format!("{}A", s).to_string())
        );

        pos_keycodes.iter().map(|seq| {
            let mut seq_len: i64 = 0;
            let mut p_char: char = 'A';
            for c in seq.chars() {
                seq_len += shortest_sequence(p_char, c, rem_directional - 1);
                p_char = c;
            }

            seq_len
        }).min().unwrap()
    }
}

fn part1(contents: String) -> i64 {
    let numeric_mapping = keypad_mapping(NUMERIC.to_string());

    return contents.lines().map(|line| {
        let mut possible: HashSet<String> = HashSet::from([String::new()]);
        let mut last_char: char = 'A';
        for c in line.chars() {
            possible = HashSet::from_iter(
                possible.iter().cartesian_product(numeric_mapping.get(&last_char).unwrap().get(&c).unwrap().iter())
                    .map(|(pre, post)| {
                        format!("{}{}A", pre, post).to_string()
                    })
            );
            last_char = c;
        }

        let shortest_len = possible.iter().map(|seq| {
            let mut seq_len: i64 = 0;
            let mut p_char: char = 'A';
            for c in seq.chars() {
                seq_len += shortest_sequence(p_char, c, 2);
                p_char = c;
            }

            seq_len
        }).min().unwrap();

        line[..line.len()-1].parse::<i64>().unwrap() * shortest_len
    }).sum::<i64>();
}

fn part2(contents: String) -> i64 {
    let numeric_mapping = keypad_mapping(NUMERIC.to_string());

    return contents.lines().map(|line| {
        let mut possible: HashSet<String> = HashSet::from([String::new()]);
        let mut last_char: char = 'A';
        for c in line.chars() {
            possible = HashSet::from_iter(
                possible.iter().cartesian_product(numeric_mapping.get(&last_char).unwrap().get(&c).unwrap().iter())
                    .map(|(pre, post)| {
                        format!("{}{}A", pre, post).to_string()
                    })
            );
            last_char = c;
        }

        let shortest_len = possible.iter().map(|seq| {
            let mut seq_len: i64 = 0;
            let mut p_char: char = 'A';
            for c in seq.chars() {
                seq_len += shortest_sequence(p_char, c, 25);
                p_char = c;
            }

            seq_len
        }).min().unwrap();

        line[..line.len()-1].parse::<i64>().unwrap() * shortest_len
    }).sum::<i64>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 126384);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "21".to_string();

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
        "\nPart 1:\nMinimum keypresses: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMinimum keypresses: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}