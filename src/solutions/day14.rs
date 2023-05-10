use std::{collections::HashMap};
use itertools::Itertools;

#[derive(Debug)]
struct Polymer
{
    state: HashMap<String, u64>,
    rules: HashMap<String, (String, String)>
}

impl Polymer
{
    fn new(input: &str) -> Self
    {
        let (template_str, rules_str) = input.split_once("\r\n\r\n").unwrap();

        let mut state = HashMap::new();
        let template_chars = template_str.chars().collect::<Vec<_>>();
        for i in 0..template_str.len()-1
        {
            let pair = format!("{}{}", template_chars[i], template_chars[i+1]);
            let &count = state.get(&pair).unwrap_or(&0);
            state.insert(pair, count + 1);
        }

        let rules = rules_str.lines().fold(HashMap::new(), |mut map, s|
        {
            let (rule_from, rule_to) = s.split_once(" -> ").unwrap();
            let first_char = &rule_from[0..1];
            let second_char = &rule_from[1..];
            let rule_1 = format!("{}{}", first_char, rule_to);
            let rule_2 = format!("{}{}", rule_to, second_char);
            map.insert(rule_from.to_owned(), (rule_1, rule_2));
            map
        });

        Polymer { state, rules }
    }

    fn step(mut self) -> Self
    {
        self.state = self.state.iter().fold(HashMap::new(), |mut map, (pair, &count)|
        {
            let (rule_1, rule_2) = self.rules.get(pair).cloned().unwrap();
            if rule_1 == rule_2
            {
                let new_count = 2 * (map.get(&rule_1).unwrap_or(&0) + count);
                map.insert(rule_1, new_count);
            }
            else
            {
                let count_1 = map.get(&rule_1).unwrap_or(&0) + count;
                let count_2 = map.get(&rule_2).unwrap_or(&0) + count;
                map.insert(rule_1, count_1);
                map.insert(rule_2, count_2);
            }
            map
        });
        self
    }
}

pub fn solution_a(input: &str) -> String
{
    solve(input, 10)
}

pub fn solution_b(input: &str) -> String
{
    solve(input, 40)
}

fn solve(input: &str, steps: u32) -> String
{
    let mut polymer = Polymer::new(input);
    for i in 0..steps { polymer = polymer.step(); }

    let char_counts = polymer.state.into_iter().fold(HashMap::new(), |mut map, (pair, count)|
    {
        let char = pair.chars().next().unwrap();
        let char_count = map.get(&char).unwrap_or(&0) + count;
        map.insert(char, char_count);
        map
    }).into_iter()
        .sorted_by(|&(char_a, char_a_count), (char_b, char_b_count)| char_a_count.cmp(char_b_count))
        .collect::<Vec<_>>();

    let (least_char, mut least_count) = char_counts[0];
    let (most_char, mut most_count) = char_counts[char_counts.len()-1];
    let last_char_of_input = input.lines().next().unwrap().chars().last().unwrap();
    if least_char == last_char_of_input { least_count += 1; }
    if most_char == last_char_of_input { most_count += 1; }
    let result = most_count - least_count;

    result.to_string()
}