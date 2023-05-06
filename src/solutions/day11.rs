use std::{collections::{HashMap}, cmp::min};
use itertools::Itertools;

struct OctopusGrid
{
    rows: usize,
    cols: usize,
    octopi: HashMap<(usize,usize),Octopus>
}

impl OctopusGrid
{
    fn new(input: &str) -> Self
    {     
        let mut grid = OctopusGrid {rows: 0, cols: 0, octopi: HashMap::new()};

        let lines = input.lines().collect::<Vec<_>>();
        grid.rows = lines.len();
        grid.cols = lines[0].chars().count();

        lines.iter().enumerate().for_each(|(row, line)|
        {
            line.chars().enumerate().for_each(|(col, level)|
            {
                let energy_level = level.to_digit(10).unwrap();
                let adj_rows = row.checked_sub(1).unwrap_or_default()..=min(row+1, grid.rows-1);
                let adj_cols = col.checked_sub(1).unwrap_or_default()..=min(col+1, grid.cols-1);
                let adjacent_octopi = adj_rows.cartesian_product(adj_cols)
                    .filter(|&x| x != (row,col))
                    .collect();

                let octopus = Octopus { energy_level, is_flashing: false, adjacent_octopi};
                grid.octopi.insert((row, col), octopus);
            });
        });

        grid
    }
    
    fn step(mut self) -> (Self, u32)
    {
        let mut to_update : Vec<(usize, usize)> = (0..self.rows).cartesian_product(0..self.cols).collect();
        while !to_update.is_empty()
        {
            let mut to_propagate : Vec<(usize, usize)> = Vec::new();
            for cell in to_update.iter()
            {
                let mut octopus = self.octopi.get_mut(cell).unwrap();
                if octopus.is_flashing { continue; }

                octopus.energy_level += 1;
                if octopus.energy_level > 9
                {
                    octopus.is_flashing = true;
                    octopus.adjacent_octopi.iter().for_each(|adj|
                    {
                        to_propagate.push(*adj);
                    });
                }
            }
            to_update = to_propagate;
        }
        
        let mut num_flashing = 0;
        for octopus in self.octopi.values_mut().filter(|o| o.is_flashing)
        {
            num_flashing += 1;
            octopus.is_flashing = false;
            octopus.energy_level = 0;
        }
        (self, num_flashing)
    }    
}

struct Octopus
{
    energy_level : u32,
    is_flashing : bool,
    adjacent_octopi : Vec<(usize, usize)>
}

pub fn solution_a(input: &str) -> String
{
    let mut grid = OctopusGrid::new(input);
    let mut num_flashes = 0;
    let mut flashing : u32;
    for i in 1..=100
    {
        (grid, flashing) = grid.step();
        num_flashes += flashing;
    }
    num_flashes.to_string()
}

pub fn solution_b(input: &str) -> String
{
    let mut grid = OctopusGrid::new(input);
    let mut i = 0;
    while grid.octopi.values().any(|o| o.energy_level != 0)
    {
        (grid, _) = grid.step();
        i += 1;
    }
    i.to_string()
}