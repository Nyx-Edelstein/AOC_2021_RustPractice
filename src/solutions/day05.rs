use std::{collections::HashMap, cmp::max, cmp::min};

#[derive(Debug)]
struct Line
{
    x1 : u32,
    y1 : u32,
    x2 : u32,
    y2 : u32,
}

impl Line
{
    fn new(input : &str) -> Self
    {
        let values : Vec<_> = input.split(&[',', ' ', '-', '>'][..])
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        Self{x1: values[0], y1: values[1], x2: values[2], y2: values[3]}
    }

    fn is_horizontal(&self) -> bool
    {
        self.x1 == self.x2
    }

    fn is_vertical(&self) -> bool
    {
        self.y1 == self.y2
    }

    fn intersect(&self, plane : &mut HashMap<(u32, u32), u32>)
    {
        //Get points from start to end
        //Cases: horizontal, vertical, diagonal(45 degrees exactly)
        let d_x = max(self.x1, self.x2) - min(self.x1, self.x2);
        let d_y = max(self.y1, self.y2) - min(self.y1, self.y2);
        let num_steps = max(d_x, d_y);
        let points : Vec<_> = (0..=num_steps).map(|n|
        {
            let x = Self::n_closer(self.x1, self.x2, n);
            let y = Self::n_closer(self.y1, self.y2, n);
            (x, y)
        }).collect();

        //For each point, either insert 1 or increment
        for &point in points.iter()
        {
            *plane.entry(point).or_insert(0) += 1;
        }
    }

    fn n_closer(v1 : u32, v2 : u32, n : u32) -> u32
    {
        match v1 <= v2
        {
            true => min(v1 + n, v2),
            false => max(v1 - n, v2)
        }
    }
}

pub fn solution_a(input: &str) -> String
{
    let lines : Vec<_> = input.lines()
        .map(Line::new)
        .filter(|l| l.is_horizontal() || l.is_vertical())
        .collect();

    count_overlaps(lines)
}

pub fn solution_b(input: &str) -> String
{
    let lines : Vec<_> = input.lines()
        .map(Line::new)
        .collect();
    
    count_overlaps(lines)
}

fn count_overlaps(lines : Vec<Line>) -> String
{    
    let mut plane : HashMap<(u32, u32), u32> = HashMap::new();
    for line in lines.iter()
    {
        line.intersect(&mut plane);
    }

    let overlaps = plane.iter()
        .filter(|(&point, &count)| count > 1)
        .count();

    overlaps.to_string()
}

