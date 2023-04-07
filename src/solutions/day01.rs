pub fn solution_a(input: &String) -> String
{
    let mut num_depth_increases : u32 = 0;
    let mut prev_depth : u32 = 0;
    for line in input.lines()
    {
        let depth = line.parse::<u32>().unwrap();
        if depth > prev_depth
        {
            num_depth_increases = num_depth_increases + 1;
        }
        prev_depth = depth;
    };

    return (num_depth_increases-1).to_string();
}

pub fn solution_b(input: &String) -> String
{
    //let mut windows = Vec::new();
    let lines = input.lines();
    for line in lines
    {
        println!("{}", line)
    };

    return String::from("");
}