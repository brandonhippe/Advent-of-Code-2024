use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Crate {
    x: i64,
    y: i64,
    width: i64,
}

impl Hash for Crate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x..=self.x+self.width).hash(state);
        self.y.hash(state);
    }
}

impl Crate {
    fn can_push(&self, dx: i64, dy: i64, walls: &HashSet<(i64, i64)>, inital_crates: &HashSet<Crate>) -> bool {
        let n_y = self.y + dy;
        let n_x = self.x + dx;
        for test_x in n_x..=n_x+self.width {
            if walls.contains(&(test_x, n_y)) {
                return false;
            }
        }

        for test in inital_crates.intersection(&check_crates(self.x, dx, self.y, dy, self.width)) {
            if !test.can_push(dx, dy, walls, inital_crates) {
                return false;
            }
        }

        return true;
    }

    fn push_crates(&self, dx: i64, dy: i64, inital_crates: &HashSet<Crate>) -> HashMap<Crate, Crate> {
        let mut new_crates: HashMap<Crate, Crate> = HashMap::from([
            (*self, Crate {x: self.x + dx, y: self.y + dy, width: self.width})
        ]);

        for pushing in inital_crates.intersection(&check_crates(self.x, dx, self.y, dy, self.width)) {
            for (k, v) in pushing.push_crates(dx, dy, inital_crates).drain() {
                new_crates.insert(k, v);
            }
        }

        return new_crates;
    } 
}

fn check_crates(x: i64, dx: i64, y: i64, dy: i64, width: i64) -> HashSet<Crate> {
    if dx != 0 {
        HashSet::from_iter((1..=width+1).map(|off| {
            Crate {
                x: x + (off * dx),
                y: y,
                width: width,
            }
        }))
    } else {
        HashSet::from_iter(((x-width..=x+width)).map(|n_x| {
            Crate {
                x: n_x,
                y: y + dy,
                width: width,
            }
        }))
    }
}

fn run_input(contents: String, width: i64) -> i64 {
    let mut input_groups = contents.split("\n\n");
    let mut walls: HashSet<(i64, i64)> = HashSet::new();
    let mut crates: HashSet<Crate> = HashSet::new();
    let mut start_pos: Option<(i64, i64)> = None;
    
    for (y, line) in input_groups.next().unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'O' => {crates.insert(Crate {x: (width + 1) * x as i64, y: y as i64, width: width});},
                '@' => start_pos = Some(((width + 1) * x as i64, y as i64)),
                '#' => {
                    for x_p in (width + 1) * x as i64..=(width + 1) * x as i64 + width {
                        walls.insert((x_p, y as i64));
                    }
                },
                '.' => (),
                _ => panic!("{}", format!("Unknown character in map: {}", c))
            }
        }
    }
    assert!(start_pos.is_some());
    let mut robot_pos: (i64, i64) = start_pos.unwrap();
    
    for (dx, dy) in input_groups.next().unwrap().chars().filter(|c| *c != '\n').map(|c| {
        match c {
            '^' => (0, -1),
            'v' => (0, 1),
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => panic!("{}", format!("Unknown character in directions: {}", c))
        }
    }) {
        let next_pos = (robot_pos.0 + dx, robot_pos.1 + dy);
        if walls.contains(&next_pos) {
            continue;
        }

        let mut intersect_crates: HashSet<Crate> = HashSet::from_iter((-width..width).map(|w| {
            Crate {
                x: next_pos.0 + w,
                y: next_pos.1,
                width: width,
            }
        }));
        intersect_crates.insert(Crate {x: next_pos.0, y: next_pos.1, width});
        let to_move: Option<&Crate> = crates.intersection(&intersect_crates).next();

        if to_move.is_none() {
            robot_pos = next_pos;
            continue;
        }

        let moving = *to_move.unwrap();
        if moving.can_push(dx, dy, &walls, &crates) {
            let moved_crates: HashMap<Crate, Crate> = moving.push_crates(dx, dy, &crates);
            crates = HashSet::from_iter(crates.iter().map(|c| *moved_crates.get(c).unwrap_or(c)));
            robot_pos = next_pos;
        }
    }
    
    return crates.iter().map(|c| c.y * 100 + c.x).sum::<i64>();
}

fn part1(contents: String) -> i64 {
    return run_input(contents, 0);
}

fn part2(contents: String) -> i64 {
    return run_input(contents, 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents), 2028);

        let contents =
            fs::read_to_string("example2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents), 10092);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example3.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents), 618);

        let contents =
            fs::read_to_string("example2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents), 9021);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "15".to_string();

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
        "\nPart 1:\nCoordinate Sum: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nCoordinate Sum: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}