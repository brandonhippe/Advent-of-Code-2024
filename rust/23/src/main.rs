use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

fn bron_kerbosch(connections: &HashMap<String, HashSet<String>>, clique: HashSet<String>, candidates: &mut HashSet<String>, excluded: &mut HashSet<String>) -> Vec<HashSet<String>> {
    if candidates.len() == 0 && excluded.len() == 0 {
        return vec![clique];
    }

    let mut cliques: Vec<HashSet<String>> = Vec::new();
    let pivot: String = candidates.union(&excluded).max_by(|a, b| connections.get(a.clone()).unwrap().len().cmp(&connections.get(b.clone()).unwrap().len())).unwrap().to_string();
    for v in candidates.clone().difference(connections.get(&pivot).unwrap()) {
        cliques.extend_from_slice(
            &bron_kerbosch(
                connections, 
                HashSet::from_iter(clique.union(&HashSet::from([v.clone()])).map(|n| n.clone())), 
                &mut HashSet::from_iter(candidates.intersection(connections.get(v).unwrap()).map(|n| n.clone())),
                &mut HashSet::from_iter(excluded.intersection(connections.get(v).unwrap()).map(|n| n.clone())),
            )[..]
        );
        candidates.remove(v);
        excluded.insert(v.clone());
    }

    return cliques;
}


fn part1(contents: String) -> i64 {
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for line in contents.lines() {
        let (a, b) = line.split("-").map(|s| s.to_string()).collect_tuple().unwrap();
        connections.entry(a.clone()).or_insert(HashSet::new()).insert(b.clone());
        connections.entry(b.clone()).or_insert(HashSet::new()).insert(a.clone());
    }

    let valid: HashSet<(String, String, String)> = HashSet::from_iter(
        connections.clone().iter().filter(|(k, _)| k.starts_with('t')).flat_map(|(k, conns)| {
            conns.iter().flat_map(|k1| {
                connections.get(k1).unwrap().iter().filter_map(|k2| {
                    if conns.contains(k2) {
                        let mut v = vec![k.clone(), k1.clone(), k2.clone()];
                        v.sort();
                        v.iter().map(|s| s.clone()).collect_tuple()
                    } else {
                        None
                    }
                })
            })
        })
    );

    return valid.len() as i64;
}

fn part2(contents: String) -> String {
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for line in contents.lines() {
        let (a, b) = line.split("-").map(|s| s.to_string()).collect_tuple().unwrap();
        connections.entry(a.clone()).or_insert(HashSet::new()).insert(b.clone());
        connections.entry(b.clone()).or_insert(HashSet::new()).insert(a.clone());
    }

    let mut max_clique: Vec<String> = Vec::from_iter(bron_kerbosch(
        &connections, 
        HashSet::new(), 
        &mut HashSet::from_iter(connections.keys().map(|k| k.clone())),
        &mut HashSet::new(),
    ).iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap().iter().map(|s| s.clone()));
    max_clique.sort();

    return max_clique.iter().map(|s| s.clone()).reduce(|comb, s| format!("{},{}", comb, s)).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 7);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), "co,de,ka,ta".to_string());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "23".to_string();

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
        "\nPart 1:\nValid sets: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMax Clique: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}