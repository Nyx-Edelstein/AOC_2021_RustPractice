use itertools::Itertools;
use std::{collections::{HashSet}};

fn parse(input : &str) -> Vec<Vec<u32>>
{
    input.lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line : &str) -> Vec<u32>
{
    line.chars()
        .filter_map(|s| s.to_digit(10))
        .collect()
}

pub fn solution_a(input: &str) -> String
{
    let heightmap = parse(input);
    let risk_level_sum = (0..heightmap.len()).cartesian_product(0..heightmap[0].len())
        .map(|(row, col)| get_risk_level(&heightmap, row, col))
        .sum::<u32>();
    risk_level_sum.to_string()
}

fn get_risk_level(heightmap: &[Vec<u32>], row: usize, col: usize) -> u32
{
    let val = heightmap[row][col];
    let all_nearby_points_higher =
    [
        row.checked_sub(1).map(|row_above| heightmap[row_above][col]),
        col.checked_sub(1).map(|col_left| heightmap[row][col_left]),
        heightmap[row].get(col+1).copied(),
        heightmap.get(row+1).map(|row_below| row_below[col])
    ].into_iter().flatten().all(|nearby_val| val < nearby_val);
    match all_nearby_points_higher
    {
        true => val + 1,
        false => 0
    }
}

pub fn solution_b(input: &str) -> String
{
    let heightmap = parse(input);
    let basin_score = (0..heightmap.len()).cartesian_product(0..heightmap[0].len())
        .filter(|(row, col)| get_risk_level(&heightmap, *row, *col) > 0)
        .map(|(row, col)| get_basin_size(&heightmap, row, col))
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .product::<u32>();

    basin_score.to_string()
}

fn get_basin_size(heightmap: &[Vec<u32>], row: usize, col: usize) -> u32
{
    let mut basin_points : HashSet<(usize, usize)> = HashSet::new();
    let mut to_consider : Vec<(usize, usize)> = vec![(row,col)];
    while let Some(current) = to_consider.pop()
    {
        basin_points.insert(current);
        let next = expand_node(current, heightmap);
        for node in next.into_iter().filter(|node| !basin_points.contains(node))
        {
            to_consider.push(node);
        }
    }

    basin_points.len().try_into().unwrap()
}

fn expand_node((row, col): (usize, usize), heightmap: &[Vec<u32>]) -> Vec<(usize, usize)>
{
    let mut nodes : Vec<(usize, usize)> = Vec::new();
    if row > 0 { nodes.push((row-1, col)) }
    if col > 0 { nodes.push((row, col-1)) }
    if col+1 < heightmap[0].len() { nodes.push((row, col+1)) }
    if row+1 < heightmap.len() { nodes.push((row+1, col)) }

    nodes.into_iter().filter(|(r, c)| heightmap[*r][*c] != 9).collect::<Vec<_>>()
}
