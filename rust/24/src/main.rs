use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashSet, HashMap, VecDeque};
use std::iter::zip;
use itertools::Itertools;

fn parse_data(contents: &str, swaps: &HashMap<String, String>) -> (
    HashMap<(String, String), HashSet<String>>,
    HashMap<(String, String), HashSet<String>>,
    HashMap<(String, String), HashSet<String>>,
    HashMap<String, bool>,
    HashMap<String, (String, String)>,
) {
    let mut content_groups = contents.split("\n\n");
    let gate_vals: HashMap<String, bool> = HashMap::from_iter(
        content_groups.next().unwrap().lines().map(|line| {
            let mut l_split = line.split(": ");
            (l_split.next().unwrap().to_string(), l_split.next().unwrap() == "1")
        })
    );
    
    let mut ands: HashMap<(String, String), HashSet<String>> = HashMap::new();
    let mut ors: HashMap<(String, String), HashSet<String>> = HashMap::new();
    let mut xors: HashMap<(String, String), HashSet<String>> = HashMap::new();
    let mut adj_list: HashMap<String, (String, String)> = HashMap::new();
    
    for line in content_groups.next().unwrap().lines() {
        let mut line_map = line.split(" -> ");
        let gate_ins: Vec<String> = Vec::from_iter(line_map.next().unwrap().split_whitespace().map(|s| s.to_string()));
        let mut gate_out = line_map.next().unwrap().to_string();
        gate_out = swaps.get(&gate_out).unwrap_or(&gate_out).to_string();
    
        adj_list.insert(gate_out.clone(), (gate_ins[0].clone(), gate_ins[2].clone()));
        match gate_ins[1].as_str() {
            "AND" => ands.entry((gate_ins[0].clone(), gate_ins[2].clone())).or_insert(HashSet::new()).insert(gate_out),
            "OR" => ors.entry((gate_ins[0].clone(), gate_ins[2].clone())).or_insert(HashSet::new()).insert(gate_out),
            "XOR" => xors.entry((gate_ins[0].clone(), gate_ins[2].clone())).or_insert(HashSet::new()).insert(gate_out),
            _ => panic!("Unknown gate type: {}", gate_ins[1])
        };
    }

    return (ands, ors, xors, gate_vals, adj_list);
}

fn sim_circuit(initial_gates: HashMap<String, bool>, ands: &HashMap<(String, String), HashSet<String>>, ors: &HashMap<(String, String), HashSet<String>>, xors: &HashMap<(String, String), HashSet<String>>) -> HashMap<String, bool> {
    let mut gate_vals = initial_gates.clone();
    let mut p_changed: i64 = 0;
    let mut changed: i64 = initial_gates.len() as i64;

    while changed != 0 || p_changed != 0 {
        let mut updates: Vec<(String, bool)> = Vec::new();
        for ((k1, k2), outs) in ands.iter() {
            let k1_v = gate_vals.get(k1);
            let k2_v = gate_vals.get(k2);
    
            for o in outs.iter().filter(|o| !gate_vals.contains_key(*o)) {
                if k1_v.is_some() && k2_v.is_some() {
                    updates.push((o.to_string(), *k1_v.unwrap() & *k2_v.unwrap()));
                } else if (k1_v.is_some() && !*k1_v.unwrap()) || (k2_v.is_some() && !*k2_v.unwrap()) {
                    updates.push((o.to_string(), false));
                }
            }
        }
    
        for ((k1, k2), outs) in ors.iter() {
            let k1_v = gate_vals.get(k1);
            let k2_v = gate_vals.get(k2);
    
            for o in outs.iter().filter(|o| !gate_vals.contains_key(*o)) {
                if k1_v.is_some() && k2_v.is_some() {
                    updates.push((o.to_string(), *k1_v.unwrap() | *k2_v.unwrap()));
                } else if (k1_v.is_some() && *k1_v.unwrap()) || (k2_v.is_some() && *k2_v.unwrap()) {
                    updates.push((o.to_string(), true));
                }
            }
        }
    
        for ((k1, k2), outs) in xors.iter() {
            let k1_v = gate_vals.get(k1);
            let k2_v = gate_vals.get(k2);
    
            for o in outs.iter().filter(|o| !gate_vals.contains_key(*o)) {
                if k1_v.is_some() && k2_v.is_some() {
                    updates.push((o.to_string(), *k1_v.unwrap() ^ *k2_v.unwrap()));
                }
            }
        }

        p_changed = changed;
        changed = updates.len() as i64;
        for (k, v) in updates {
            gate_vals.insert(k, v);
        }
    }

    return gate_vals;
}

fn incorrect_on_paths(shift_amt: i64, range_lim: i64, input_size: i64, ands: &HashMap<(String, String), HashSet<String>>, ors: &HashMap<(String, String), HashSet<String>>, xors: &HashMap<(String, String), HashSet<String>>, adj_list: &HashMap<String, (String, String)>, get_all: bool) -> HashMap<Option<bool>, HashMap<String, HashSet<String>>> {
    let mut on_path: HashMap<Option<bool>, HashMap<String, HashSet<String>>> = HashMap::new();

    for (t1, t2) in (0..range_lim).map(|n| n << shift_amt).cartesian_product((0..range_lim).map(|n| n << shift_amt)) {
        let expected_val: i64 = t1 + t2;

        let in_out_slice = 0..=(shift_amt + (range_lim as f64).log2().ceil() as i64);
        let expected_output: HashMap<String, bool> = HashMap::from_iter(in_out_slice.clone().map(|n| {
            (format!("z{:02}", n), (expected_val >> n) % 2 == 1)
        }));

        let gate_vals = sim_circuit(
            HashMap::from_iter(
                zip("xy".chars(), [t1, t2]).flat_map(|(c, v)| {
                    (0..input_size).map(move |n| {
                        (format!("{}{:02}", c, n), (v >> n) % 2 == 1)
                    })
                })
            ),
            ands,
            ors,
            xors,
        );
        
        let incorrect_wires: HashMap<String, bool> = HashMap::from_iter(
            expected_output.iter().filter_map(|(k, v)| {
                if !gate_vals.contains_key(k) || gate_vals.get(k).unwrap() != v {
                    Some((k.clone(), *v))
                } else {
                    None
                }
            })
        );

        if incorrect_wires.len() == 0 {
            continue;
        }
        
        if !get_all {
            return HashMap::from([(Some(true), HashMap::new())]);
        }

        let mut open_wires: VecDeque<(String, String, bool)> = VecDeque::from_iter(
            incorrect_wires.keys().map(|k| {
                (k.clone(), k.clone(), *expected_output.get(k).unwrap())
            })
        );
        let mut checked_wires: HashSet<String> = HashSet::new();

        while let Some((wire, dest_wire, expected)) = open_wires.pop_front() {
            if checked_wires.contains(&wire) || !adj_list.contains_key(&wire) {
                continue;
            }
            checked_wires.insert(wire.clone());
            on_path.entry(gate_vals.get(&wire).copied()).or_insert(HashMap::new()).entry(dest_wire.clone()).or_insert(HashSet::new()).insert(wire.clone());

            let correct: bool = expected == *gate_vals.get(&wire).unwrap();
            let from_pair = adj_list.get(&wire).unwrap();

            for (new_f0, new_f1) in [true, false].iter().cartesian_product([true, false].iter()) {
                let result_val = if ands.contains_key(&from_pair) && ands.get(&from_pair).unwrap().contains(&wire) {
                    new_f0 & new_f1
                } else if ors.contains_key(&from_pair) && ors.get(&from_pair).unwrap().contains(&wire) {
                    new_f0 | new_f1
                } else if xors.contains_key(&from_pair) && xors.get(&from_pair).unwrap().contains(&wire) {
                    new_f0 ^ new_f1
                } else {
                    panic!("Unknown from_pair: {:?}", from_pair);
                };

                if correct ^ (expected == result_val) {
                    open_wires.push_back((from_pair.0.clone(), dest_wire.clone(), *new_f0));
                    open_wires.push_back((from_pair.1.clone(), dest_wire.clone(), *new_f1));
                }
            }
        }
    }

    return on_path;
}

fn test_valid(shift_amt: i64, range_lim: i64, input_size: i64, ands: &HashMap<(String, String), HashSet<String>>, ors: &HashMap<(String, String), HashSet<String>>, xors: &HashMap<(String, String), HashSet<String>>, adj_list: &HashMap<String, (String, String)>) -> bool {
    return incorrect_on_paths(shift_amt, range_lim, input_size, ands, ors, xors, adj_list, false).len() == 0;
}

fn part1(contents: String) -> i64 {
    let binding = HashMap::new();
    let (ands, ors, xors, gate_vals, _) = parse_data(&contents, &binding);
    return sim_circuit(gate_vals, &ands, &ors, &xors).iter().filter_map(|(k, v)| {
        if k.starts_with('z') {
            Some((*v as i64) << k[1..].parse::<i64>().unwrap())
        } else {
            None
        }
    }).sum::<i64>();
}

fn part2(contents: String) -> String {
    let binding = HashMap::new();
    let input_size: i64 = (parse_data(&contents, &binding).3.len() as i64) / 2;
    
    let mut swapping: Option<HashSet<String>> = None;
    let mut open_swaps: VecDeque<(HashMap<String, String>, i64)> = VecDeque::from([(HashMap::new(), 0)]);

    while let Some((to_swap, start_at)) = open_swaps.pop_front() {
        let (ands, ors, xors, _, adj_list) = parse_data(&contents, &to_swap);

        let mut found_error: bool = false;
        let mut shift_amt: i64 = start_at;
        while shift_amt < input_size {
            let range_lim: i64 = if shift_amt == input_size - 1 {2} else {4};
            let incorrect_outputs = incorrect_on_paths(
                shift_amt,
                range_lim,
                input_size,
                &ands,
                &ors,
                &xors,
                &adj_list,
                true,
            );

            if incorrect_outputs.len() == 0 || (incorrect_outputs.len() < 2 && shift_amt < input_size - 1) {
                shift_amt += 1;
                continue;
            }
            found_error = true;

            let dests: HashSet<String> = incorrect_outputs.values().map(|v| HashSet::from_iter(v.keys().map(|s| s.clone()))).reduce(|a, b| HashSet::from_iter(a.intersection(&b).map(|s| s.clone()))).unwrap();
            let dest_sets: Vec<HashSet<String>> = Vec::from_iter(
                dests.iter().map(|d| (
                    incorrect_outputs.values().filter_map(|v| {
                        v.get(d).cloned()
                    }).reduce(|a, b| HashSet::from_iter(a.intersection(&b).map(|s| s.clone()))).unwrap()
                ))
            );
            if dest_sets.len() == 1 {
                shift_amt += 1;
                continue;
            }

            let mut valid_found: Vec<HashMap<String, String>> = Vec::new();
            for sets in dest_sets.iter().combinations(2) {
                for (i_wire, j_wire) in sets[0].iter().cartesian_product(sets[1].iter()) {
                    if i_wire == j_wire || to_swap.contains_key(i_wire) || to_swap.contains_key(j_wire) {
                        continue;
                    }

                    let mut new_swap = to_swap.clone();
                    new_swap.insert(i_wire.clone(), j_wire.clone());
                    new_swap.insert(j_wire.clone(), i_wire.clone());
                    
                    let (test_ands, test_ors, test_xors, _, test_adj_list) = parse_data(&contents, &new_swap);
                    if test_valid(shift_amt, range_lim, input_size, &test_ands, &test_ors, &test_xors, &test_adj_list) {
                        valid_found.push(new_swap);
                    }
                }
            }

            for new_swap in valid_found {
                open_swaps.push_back((new_swap, shift_amt));
            }
            
            break;
        }

        if !found_error {
            swapping = Some(HashSet::from_iter(to_swap.keys().map(|s| s.clone())));
            break;
        }
    }

    let mut swapping_vec: Vec<String> = Vec::from_iter(swapping.unwrap().iter().map(|s| s.to_string()));
    swapping_vec.sort();

    return swapping_vec.iter().map(|s| s.to_string()).reduce(|tot, s| format!("{},{}", tot, s)).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents), 4);

        let contents =
            fs::read_to_string("example2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents), 2024);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "24".to_string();

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
        "\nPart 1:\nCircuit output: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nIncorrect wires: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}