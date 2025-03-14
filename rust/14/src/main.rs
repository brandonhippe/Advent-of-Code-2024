use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use regex::Regex;
use std::collections::HashMap;
use mod_exp::mod_exp;
use std::iter::zip;

#[derive(Debug)]
struct Robot {
    p_x: i64,
    p_y: i64,
    v_x: i64,
    v_y: i64,
}

impl Robot {
    fn new(line: &str) -> Robot {
        let re = Regex::new(r"-?\d+").unwrap();
        let mut nums = re.find_iter(line);

        Robot {
            p_x: nums.next().unwrap().as_str().parse::<i64>().unwrap(),
            p_y: nums.next().unwrap().as_str().parse::<i64>().unwrap(),
            v_x: nums.next().unwrap().as_str().parse::<i64>().unwrap(),
            v_y: nums.next().unwrap().as_str().parse::<i64>().unwrap(),
        }
    }

    fn move_robot(&mut self, width: i64, height: i64) {
        self.p_x = (self.p_x + self.v_x + width) % width;
        self.p_y = (self.p_y + self.v_y + height) % height;
    }

    fn quadrant(&self, width: i64, height: i64) -> Option<i64> {
        if self.p_x == width / 2 || self.p_y == height / 2 {
            None
        } else {
            Some((((self.p_y > height / 2) as i64) << 1) + ((self.p_x > width / 2) as i64))
        }
    }
}

fn part1(contents: String, width: i64, height: i64) -> i64 {
    let mut robots: Vec<Robot> = Vec::from_iter(contents.lines().map(|l| Robot::new(l)));

    for _ in 0..100 {
        for mut robot in &mut robots {
            robot.move_robot(width, height);
        }
    }

    let mut robot_quadrants: HashMap<i64, Vec<Robot>> = HashMap::new();
    for robot in robots {
        if let Some(q) = robot.quadrant(width, height) {
            robot_quadrants.entry(q).or_insert(Vec::new()).push(robot);
        }
    }
    
    return robot_quadrants.values().map(|v| v.len() as i64).product::<i64>();
}

fn part2(contents: String, width: i64, height: i64) -> i64 {
    let mut robots: Vec<Robot> = Vec::from_iter(contents.lines().map(|l| Robot::new(l)));

    let mut min_x_var: f64 = f64::INFINITY;
    let mut min_y_var: f64 = f64::INFINITY;
    let mut min_x_t: i64 = -1;
    let mut min_y_t: i64 = -1;

    for t in 0..width.max(height) {
        let mean_x = robots.iter().map(|r| r.p_x as f64).sum::<f64>() / (robots.len() as f64);
        let mean_y = robots.iter().map(|r| r.p_y as f64).sum::<f64>() / (robots.len() as f64);
        let var_x = robots.iter().map(|r| (r.p_x as f64 - mean_x).powf(2.0)).sum::<f64>() / (robots.len() as f64);
        let var_y = robots.iter().map(|r| (r.p_y as f64 - mean_y).powf(2.0)).sum::<f64>() / (robots.len() as f64);
        
        if var_x < min_x_var {
            min_x_var = var_x;
            min_x_t = t as i64;
        }
        
        if var_y < min_y_var {
            min_y_var = var_y;
            min_y_t = t as i64;
        }

        for mut robot in &mut robots {
            robot.move_robot(width, height);
        }
    }

    // Chinese Remainder Theorem for the answer
    let n_s: Vec<i64> = vec![width, height];
    let big_n = n_s.iter().product::<i64>();
    let a_s: Vec<i64> = vec![min_x_t, min_y_t];
    let y_s: Vec<i64> = Vec::from_iter(n_s.iter().map(|n| big_n / n));
    let z_s: Vec<i64> = Vec::from_iter(zip(y_s.clone(), n_s.clone()).map(|(y, m)| mod_exp(y, m - 2, m)));

    return zip(a_s, zip(y_s, z_s)).map(|(a, (y, z))| a * y * z).sum::<i64>() % big_n;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents, 11, 7), 12);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "14".to_string();

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
        "\nPart 1:\nSafety Factor: {}\nRan in {:.5?}",
        part1(contents.clone(), 101, 103),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nEaster Egg appears after: {}\nRan in {:.5?}",
        part2(contents.clone(), 101, 103),
        part2_timer.elapsed()
    );
}