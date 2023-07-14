use itertools::Itertools;

pub fn solution_a(input: &str) -> String
{
    let mut grid = input.lines()
        .map(|line| line.chars().map(Cell::from_char).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = grid.len();
    let cols = grid[0].len();
    let steps = [(Cell::East, (0, 1)), (Cell::South, (1, 0))];
    let mut iterations = 0;
    let mut stopped = false;
    while !stopped
    {
        iterations += 1;
        stopped = true;
        for (cell_type, (x, y)) in steps
        {
            let mut next = grid.clone();
            (0..rows).cartesian_product(0..cols).for_each(|(i, j)|
            {
                if grid[i][j] == cell_type && grid[(i + x) % rows][(j + y) % cols] == Cell::Empty
                {
                    next[(i + x) % rows][(j + y) % cols] = cell_type;
                    next[i][j] = Cell::Empty;
                    stopped = false;
                }
            });
            grid = next
        }
    }
    iterations.to_string()
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell
{
    Empty,
    East,
    South
}

impl Cell
{
    fn from_char(c: char) -> Self
    {
        match c
        {
            '.' => Cell::Empty,
            '>' => Cell::East,
            'v' => Cell::South,
            _ => panic!("invalid input")
        }
    }
}
