fn parse(input: &String) -> Vec<u32>
{
    input.lines().map(|s| s.to_string().parse::<u32>().unwrap()).collect()
}

pub fn solution_a(input: &String) -> String
{
    let depths = parse(input);
    get_depth_increases(depths)
}

pub fn solution_b(input: &String) -> String
{
    let depths = parse(input);
    let windows = depths.iter()
        .enumerate()
        .map(|(n,depth)| depth + depths.get(n+1).unwrap_or(&0) + depths.get(n+2).unwrap_or(&0))
        .collect();
    get_depth_increases(windows)
}

fn get_depth_increases(depths: Vec<u32>) -> String
{
    let mut num_depth_increases : u32 = 0;
    let mut prev_depth : u32 = 0;
    for depth in depths
    {
        if depth > prev_depth
        {
            num_depth_increases = num_depth_increases + 1;
        }
        prev_depth = depth;
    };
    return (num_depth_increases-1).to_string();
}


