use std::collections::HashSet;
use std::env;
use std::fs;

fn load_file(path: &String) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
    let contents = fs::read_to_string(path).expect("Error reading file");
    let mut split_input = contents.trim().split("\n\n");

    let rules = split_input
        .next()
        .unwrap()
        .split("\n")
        .map(|x| {
            let mut rule = x.split("|");
            (
                rule.next().unwrap().parse::<i32>().unwrap(),
                rule.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<HashSet<_>>();

    let updates = split_input
        .next()
        .unwrap()
        .split("\n")
        .map(|row| {
            row.split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (rules, updates)
}

fn part1(input: &(HashSet<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let (rules, updates) = input;

    updates
        .iter()
        .filter(|update| {
            for (index, number1) in update.iter().enumerate() {
                for number2 in update.iter().skip(index + 1) {
                    if rules.contains(&(*number2, *number1)) {
                        return false;
                    }
                }
            }
            true
        })
        .map(|update| update[(update.len() - 1) / 2])
        .sum()
}

fn part2(input: &(HashSet<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let (rules, updates) = input;

    updates
        .iter()
        .filter(|update| {
            for (index, number1) in update.iter().enumerate() {
                for number2 in update.iter().skip(index + 1) {
                    if rules.contains(&(*number2, *number1)) {
                        return true;
                    }
                }
            }
            false
        })
        .map(|update| {
            let mut update = (*update).clone();

            loop {
                let mut fixed = false;

                'rules_loop: for &(first, second) in rules {
                    for (i, number1) in update.iter().enumerate() {
                        if *number1 == first {
                            continue 'rules_loop;
                        }

                        if *number1 == second {
                            for (j, number2) in update.iter().enumerate().skip(i) {
                                if *number2 == first {
                                    update.swap(i, j);
                                    fixed = true;
                                    continue 'rules_loop;
                                }
                            }
                        }
                    }
                }

                if !fixed {
                    break;
                }
            }

            update
        })
        .map(|update| update[(update.len() - 1) / 2])
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No file provided");
    }

    let input = load_file(&args[1]);

    println!("Part 1 solution is {}", part1(&input));
    println!("Part 2 solution is {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let rules: HashSet<(i32, i32)> = HashSet::from([
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ]);

        let updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        assert_eq!(part1(&(rules, updates)), 143);
    }

    #[test]
    fn part2_test() {
        let rules: HashSet<(i32, i32)> = HashSet::from([
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ]);

        let updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        assert_eq!(part2(&(rules, updates)), 123);
    }
}
