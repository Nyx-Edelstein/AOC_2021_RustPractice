fn parse(input : &str, size : usize) -> Vec<u64>
{
    input.split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .fold(vec![0u64; size], |mut pop, n|
        {
            *pop.get_mut(n).unwrap() += 1;
            pop
        })
}

pub fn solution_a(input: &str) -> String
{
    solve(input, 80)
}

pub fn solution_b(input: &str) -> String
{
    solve(input, 256)
}

fn solve(input: &str, num_days : u32) -> String
{
    const CYCLE_MAX : usize = 6;
    const NEW_DELAY : usize = 2;
    const MAX : usize = CYCLE_MAX + NEW_DELAY;

    let mut population = parse(input, MAX+1);
    let mut new_population = vec![0; MAX+1];

    for i in 0..num_days
    {
        for n in 0..population.len()
        {
            if n == 0
            {
                new_population[MAX] = population[0];
                new_population[CYCLE_MAX] = population[0];
            }
            else
            {
                new_population[n-1] += population[n]
            }
        }
        (population, new_population) = (new_population, population);
        for val in &mut new_population { *val = 0; }
    }

    let pop_count : u64 = population.iter().sum();
    pop_count.to_string()
}