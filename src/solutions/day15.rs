use std::{collections::{BinaryHeap}, cmp::Ordering};

pub fn solution_a(input: &str) -> String
{
    let grid = Grid::new(input);
    solve(grid)
}

pub fn solution_b(input: &str) -> String
{
    let mut grid = Grid::new(input);
    grid = grid.expand(5, 5);
    solve(grid)
}

fn solve(grid: Grid) -> String
{
    let mut to_consider = BinaryHeap::from_iter([Path::new()]);
    let mut risk : Vec<Vec<u32>> = vec![vec![u32::MAX; grid.len_y]; grid.len_x];
    while let Some(path) = to_consider.pop()
    {
        for next in path.next(&grid)
        {
            if next.risk < risk[next.loc_x][next.loc_y]
            {
                risk[next.loc_x][next.loc_y] = next.risk;
                to_consider.push(next);
            }
        }
    }
    let least_risk = risk[grid.len_x-1][grid.len_y-1];
    least_risk.to_string()
}

struct Grid
{
    points : Vec<Vec<u32>>,
    len_x : usize,
    len_y : usize,
}

impl Grid
{
    fn new(input: &str) -> Self
    {
        let points = input.lines().map(|line|
        {
            line.chars().map(|c|
            {
                c.to_digit(10).unwrap()
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        let len_x = points.len();
        let len_y = points[0].len();
        Grid { points, len_x, len_y }
    }

    fn expand(mut self, x_mult: usize, y_mult: usize) -> Self
    {
        let mut new_points = vec![vec![0; 5*self.len_y]; 5*self.len_x];
        for x in 0..self.points.len()
        {
            for y in 0..self.points[0].len()
            {
                let val = self.points[x][y];
                for mx in 0..x_mult
                {
                    for my in 0..y_mult
                    {
                        let mut new_val = val + ((mx + my) as u32);
                        while new_val > 9 { new_val -= 9; }

                        let new_x = x + mx * self.len_x;
                        let new_y = y + my * self.len_y;
                        new_points[new_x][new_y] = new_val;
                    }
                }
            }
        }
        self.points = new_points;
        self.len_x *= 5;
        self.len_y *= 5;
        self
    }
}

#[derive(Eq, PartialEq)]
struct Path
{
    loc_x : usize,
    loc_y : usize,
    risk : u32,
    heuristic: u32,
}

impl Path
{
    fn new() -> Self
    {
        Path { loc_x: 0, loc_y: 0, risk: 0, heuristic: 0 }
    }

    fn next(&self, grid: &Grid) -> Vec<Path>
    {
        [
            self.step(grid, 0, -1), //Up
            self.step(grid, 1, 0), //Right
            self.step(grid, 0, 1), //Down
            self.step(grid, -1, 0) //Left
        ].into_iter().flatten().collect::<Vec<_>>()
    }

    fn step(&self, grid: &Grid, x: i32, y: i32) -> Option<Path>
    {
        let loc_x = ((self.loc_x as i32) + x) as usize;
        let loc_y = ((self.loc_y as i32) + y) as usize;
        let Some(&val) = grid.points.get(loc_x).and_then(|x| x.get(loc_y)) else { return None; };

        let risk = self.risk + val;
        let dist_to_end = grid.len_x - loc_x + grid.len_y - loc_y;
        let heuristic = risk + dist_to_end as u32;

        Some(Path { loc_x, loc_y, risk, heuristic })
    }
}

impl Ord for Path
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        other.heuristic.cmp(&self.heuristic)
    }
}

impl PartialOrd for Path
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

