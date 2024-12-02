use std::env;
use std::fs;

fn load_file(path: &String) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(path).expect("Error reading file");
    contents
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| row.split(' ').collect::<Vec<_>>())
        .map(|row| {
            row.iter()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn is_safe_range(levels: &[i32]) -> bool {
    let difference = (levels[0] - levels[1]).abs();

    (1..=3).contains(&difference)
}

fn is_increasing(report: &[i32]) -> bool {
    report
        .windows(2)
        .all(|levels| levels[0] < levels[1] && is_safe_range(levels))
}

fn is_decreasing(report: &[i32]) -> bool {
    report
        .windows(2)
        .all(|levels| levels[0] > levels[1] && is_safe_range(levels))
}

fn part1(input: &[Vec<i32>]) -> usize {
    input
        .iter()
        .filter(|report| is_increasing(report) || is_decreasing(report))
        .count()
}

fn part2(input: &[Vec<i32>]) -> usize {
    input
        .iter()
        .filter(|report| {
            if is_increasing(report) || is_decreasing(report) {
                return true;
            }

            for i in 0..report.len() {
                let problem_dampened_report = report[..i]
                    .iter()
                    .chain(report[i + 1..].iter())
                    .cloned()
                    .collect::<Vec<i32>>();

                if is_increasing(&problem_dampened_report)
                    || is_decreasing(&problem_dampened_report)
                {
                    return true;
                }
            }

            false
        })
        .count()
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
        let input = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn part2_test() {
        let input = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(part2(&input), 4);
    }
}
