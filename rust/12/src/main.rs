use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet, VecDeque};

fn contiguous_area(plots: &HashMap<(i64, i64), char>, start_pos: (i64, i64)) -> HashMap<(i64, i64), HashSet<(i64, i64)>> {
    let test_char: char = *plots.get(&start_pos).unwrap();
    let mut area: HashMap<(i64, i64), HashSet<(i64, i64)>> = HashMap::new();
    let mut checking: VecDeque<(i64, i64)> = VecDeque::from([start_pos]);

    while let Some(pos) = checking.pop_front() {
        if area.contains_key(&pos) {
            continue;
        }

        let mut neighbors: HashSet<(i64, i64)> = HashSet::from_iter([(-1, 0), (1, 0), (0, -1), (0, 1)].iter().map(|(dx, dy)| (pos.0 + dx, pos.1 + dy)));
        
        for new_pos in neighbors.clone().iter() {
            if *plots.get(&new_pos).unwrap_or(&' ') == test_char {
                checking.push_back(*new_pos);
                neighbors.remove(new_pos);
            }
        }

        area.insert(pos, neighbors);
    }

    return area;
}

fn part1(contents: String) -> i64 {
    let mut plots: HashMap<(i64, i64), char> = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            plots.insert((x as i64, y as i64), c);
        }
    }

    let mut total_price: i64 = 0;
    while plots.len() > 0 {
        let area = contiguous_area(&plots, *plots.keys().next().unwrap());
        total_price += area.len() as i64 * area.values().map(|v| v.len() as i64).sum::<i64>();
        plots = HashMap::from_iter(plots.iter().filter_map(|(k, v)| if !area.contains_key(k) {Some((*k, *v))} else {None}));
    }
    
    return total_price;
}

fn part2(contents: String) -> i64 {
    let mut plots: HashMap<(i64, i64), char> = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            plots.insert((x as i64, y as i64), c);
        }
    }

    let mut total_price: i64 = 0;
    while plots.len() > 0 {
        let area = contiguous_area(&plots, *plots.keys().next().unwrap());
        let mut area_plots: HashMap<(i64, i64), char> = HashMap::from_iter(area.values().flat_map(|v| v.iter().map(|p| (*p, '0'))));
        let mut sides: i64 = 0;
        
        while area_plots.len() > 0 {
            let side = contiguous_area(&area_plots, *area_plots.keys().next().unwrap());
            area_plots = HashMap::from_iter(area_plots.iter().filter_map(|(k, v)| if !side.contains_key(k) {Some((*k, *v))} else {None}));
            sides += 1;
        }

        total_price += sides * area.len() as i64;
        plots = HashMap::from_iter(plots.iter().filter_map(|(k, v)| if !area.contains_key(k) {Some((*k, *v))} else {None}));
    }
    
    return total_price;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let mut contents =
            fs::read_to_string("example1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents), 140);

        contents =
            fs::read_to_string("example2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents), 772);

        contents =
            fs::read_to_string("example3.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents), 1930);
    }

    #[test]
    fn p2_test() {
        let mut contents =
            fs::read_to_string("example1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents), 80);

        contents =
            fs::read_to_string("example2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents), 436);

        contents =
            fs::read_to_string("example3.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents), 1206);

        contents =
            fs::read_to_string("example4.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents), 236);
            
        contents =
            fs::read_to_string("example5.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents), 368);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "12".to_string();

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
        "\nPart 1:\nPrice: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\n {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}