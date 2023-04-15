enum Direction
{
    Forward,
    Down,
    Up
}

struct Command
{
    direction: Direction,
    distance: u32,
}

fn parse(input: &str) -> Vec<Command>
{
    let commands: Vec<Command> = input.lines()
        .map(parse_line)
        .collect();

    commands
}

fn parse_line(input: &str) -> Command
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

    Command
    {
        direction: dir,
        distance: dist 
    }
}

pub fn solution_a(input: &str) -> String
{
    let commands = parse(input);
    let mut depth = 0;
    let mut distance = 0;
    for command in commands.iter()
    {
        match command.direction
        {
            Direction::Forward => distance += command.distance,
            Direction::Down => depth += command.distance,
            Direction::Up => depth -= command.distance,
        }
    }

    let result = depth * distance;
    result.to_string()
}

pub fn solution_b(input: &str) -> String
{
    let commands = parse(input);
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;
    for command in commands.iter()
    {
        match command.direction
        {
            Direction::Down => aim += command.distance,
            Direction::Up => aim -= command.distance,
            Direction::Forward =>
            {
                distance += command.distance;
                depth += command.distance * aim;
            },
        }
    }

    let result = depth * distance;
    result.to_string()
}