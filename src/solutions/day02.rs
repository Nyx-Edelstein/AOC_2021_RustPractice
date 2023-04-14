enum Direction
{
    Forward,
    Down,
    Up
}

struct Command
{
    Direction: Direction,
    Distance: u32,
}

fn Parse(input: &String) -> Vec<Command>
{
    let commands: Vec<Command> = input.lines()
        .map(|s| ParseLine(s))
        .collect();

    return commands;
}

fn ParseLine(input: &str) -> Command
{
    let tokens: Vec<_> = input.split_whitespace().collect();

    let (a, b) = match &tokens[..]
    {
        &[first, second, ..] => (first, second),
        _ => unreachable!(),
    };

    let dir = match a
    {
        "forward" => Direction::Forward,
        "down" => Direction::Down,
        "up" => Direction::Up,
        &_ => unreachable!()
    };

    let dist = b.parse::<u32>().unwrap();

    return Command
    {
        Direction: dir,
        Distance: dist 
    }
}

pub fn solution_a(input: &String) -> String
{
    let commands = Parse(input);
    let mut depth = 0;
    let mut distance = 0;
    for command in commands.iter()
    {
        match command.Direction
        {
            Direction::Forward => distance += command.Distance,
            Direction::Down => depth += command.Distance,
            Direction::Up => depth -= command.Distance,
        }
    }

    let result = depth * distance;
    return  result.to_string();
}

pub fn solution_b(input: &String) -> String
{
    let commands = Parse(input);
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;
    for command in commands.iter()
    {
        match command.Direction
        {
            Direction::Down => aim += command.Distance,
            Direction::Up => aim -= command.Distance,
            Direction::Forward =>
            {
                distance += command.Distance;
                depth += command.Distance * aim;
            },
        }
    }

    let result = depth * distance;
    return  result.to_string();
}