use std::{cmp::min};

fn parse(input : &str) -> Vec<i32>
{
    let mut positions : Vec<i32> = input.split(',')
        .map(|s| s.parse::<i32>())
        .filter_map(|n| n.ok())
        .collect();

    positions.sort();

    positions
}

pub fn solution_a(input: &str) -> String
{
    let positions = parse(input);
    let median_pos = match positions.len() % 2 == 0
    {
        true => positions.len()/2,
        false => positions.len()/2 + 1
    };
    let median = positions[median_pos];
    let total : u32 = positions.iter()
        .map(|&n| median.abs_diff(n))
        .sum();

    total.to_string()
}

pub fn solution_b(input: &str) -> String
{
    let positions = parse(input);
    let count = positions.len() as f64;
    let sum  = positions.iter().sum::<i32>() as f64;
    let mean = (sum/count).round() as i32;
    
    let total_1 = total(&positions, &mean);
    let total_2 = total(&positions, &(mean-1));

    let min_total = min(total_1, total_2);

    min_total.to_string()
}

fn total(positions: &[i32], x: &i32) -> u32
{
    positions.iter()
        .map(|&n| cost(x.abs_diff(n)))
        .sum()
}

fn cost(n : u32) -> u32
{
    n * (n+1) / 2
}
