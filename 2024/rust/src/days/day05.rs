use std::collections::HashMap;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let (rules, updates): (HashMap<u64, Vec<u64>>, Vec<Vec<u64>>) = parse_input(input);

    let mut sol1: u64 = 0;
    for update in updates {
        let ordered = is_ordered(&update, &rules);
        if ordered {
            sol1 += *update.get(update.len()/2).unwrap_or(&0);
        } else {
            
        }
    }

    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn is_ordered(update: &Vec<u64>, rules: &HashMap<u64, Vec<u64>>) -> bool {
    let mut ordered = true;
    'next_update: for (page_idx, page) in update.iter().enumerate() {
        for (check_idx,check) in update.clone().iter().enumerate() {
            if let Some(after) = rules.get(page) {
                if after.contains(check) && check_idx <= page_idx {
                    ordered = false;
                    break 'next_update;
                }
            }
        }
    }
    ordered
}

fn fix_unordered(update: &Vec<u64>, rules: &HashMap<u64, Vec<u64>>) -> bool {
    let mut ordered = true;
    'next_update: for (page_idx, page) in update.iter().enumerate() {
        for (check_idx,check) in update.clone().iter().enumerate() {
            if let Some(after) = rules.get(page) {
                if after.contains(check) && check_idx <= page_idx {
                    ordered = false;
                    break 'next_update;
                }
            }
        }
    }
    ordered
}

fn parse_input(input: &str) -> (HashMap<u64, Vec<u64>>, Vec<Vec<u64>>) {
    let rules_updates: Vec<&str> = input.split("\n\n").collect();
    let mut rules: HashMap<u64, Vec<u64>> = HashMap::new();

    rules_updates
        .first()
        .unwrap_or(&"")
        .trim()
        .lines()
        .for_each(|l| {
            let left_rule = l
                .trim()
                .split("|")
                .next()
                .unwrap_or_default()
                .parse()
                .unwrap_or_default();
            let right_rule = l
                .trim()
                .split("|")
                .last()
                .unwrap_or_default()
                .parse()
                .unwrap_or_default();
            let value = rules.entry(left_rule).or_default();
            value.push(right_rule);
        });

    let updates: Vec<Vec<u64>> = rules_updates
        .last()
        .unwrap_or(&"")
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .split(",")
                .map(|n| n.parse().unwrap_or_default())
                .collect()
        })
        .collect();

    (rules, updates)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing_test() {
        let input = "47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47";
        let (mut rules, updates): (HashMap<u64, Vec<u64>>, Vec<Vec<u64>>) = parse_input(input);

        let mut rules: Vec<(&u64, &mut Vec<u64>)> = rules
            .iter_mut()
            .map(|(key, value)| {
                value.sort();
                (key, value)
            })
            .collect();

        rules.sort_by(|a, b| a.0.cmp(b.0));

        assert_eq!(
            rules,
            vec![
                (&29_u64, &mut vec![13]),
                (&47, &mut vec![13, 29, 53, 61]),
                (&53, &mut vec![13, 29]),
                (&61, &mut vec![13, 29, 53]),
                (&75, &mut vec![13, 29, 47, 53, 61]),
                (&97, &mut vec![13, 29, 47, 53, 61, 75])
            ]
        );

        assert_eq!(
            updates,
            vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ]
        );
    }

    #[test]
    fn aot_test() {
        let input = "47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47";
        let (p1,p2) = solve(input);
        assert_eq!(p1, Solution::from(143_u64));
        assert_eq!(p2, Solution::from(0_u64));
    }
}
