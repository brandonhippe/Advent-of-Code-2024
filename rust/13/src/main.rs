use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use regex::Regex;
use std::iter::zip;

#[derive(Debug)]
struct Claw {
    a: (f64, f64),
    b: (f64, f64),
    prize: (f64, f64),
}

impl Claw {
    fn new(group: &str) -> Claw {
        let re = Regex::new(r"-?\d+").unwrap();
        let mut nums = re.find_iter(group);

        Claw {
            a: (nums.next().unwrap().as_str().parse::<f64>().unwrap(), nums.next().unwrap().as_str().parse::<f64>().unwrap()),
            b: (nums.next().unwrap().as_str().parse::<f64>().unwrap(), nums.next().unwrap().as_str().parse::<f64>().unwrap()),
            prize: (nums.next().unwrap().as_str().parse::<f64>().unwrap(), nums.next().unwrap().as_str().parse::<f64>().unwrap()),
        }
    }

    fn tokens(&self) -> Option<(i64, i64)> {
        let mut matrix: Vec<Vec<f64>> = vec![
            vec![self.a.0, self.b.0, self.prize.0],
            vec![self.a.1, self.b.1, self.prize.1],
        ];

        for i in 0..matrix.len() {
            // Scale current row
            let scale: f64 = matrix[i][i];
            matrix[i] = Vec::from_iter(matrix[i].iter().map(|n| *n / scale));

            // Reduce other rows
            for j in (0..matrix.len()).filter(|j| *j != i) {
                let sub_mult = matrix[j][i];
                matrix[j] = Vec::from_iter(zip(matrix[j].clone(), matrix[i].clone()).map(|(v_j, v_i)| v_j - sub_mult * v_i));
            }
        }

        if matrix.iter().all(|r| {
            let n = r[r.len() - 1].round();
            (r[r.len() - 1] - n).abs() < 0.001
        }) {
            Some((matrix[0][2].round() as i64, matrix[1][2].round() as i64))
        } else {
            None
        }
    }
}

fn part1(contents: String) -> i64 {
    return contents.split("\n\n").filter_map(|g| {
        let c = Claw::new(g);
        if let Some((a, b)) = c.tokens() {
            if a.max(b) <= 100 {
                Some(a * 3 + b)
            } else {
                None
            }
        } else {
            None
        }
    }).sum::<i64>();
}

fn part2(contents: String) -> i64 {
    return contents.split("\n\n").filter_map(|g| {
        let mut c = Claw::new(g);
        c.prize = (c.prize.0 + 10000000000000.0, c.prize.1 + 10000000000000.0);

        if let Some((a, b)) = c.tokens() {
            Some(a * 3 + b)
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

        assert_eq!(part1(contents), 480);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "13".to_string();

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
        "\nPart 1:\nTokens spent: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nTokens spent: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}