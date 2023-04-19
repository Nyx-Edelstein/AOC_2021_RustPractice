pub fn parse(input: &str) -> Vec<Vec<u32>>
{
    let bin_numbers = input.lines()
        .map(parse_line)
        .collect();

    bin_numbers
}

pub fn parse_line(line: &str) -> Vec<u32>
{
    let bits = line.chars()
        .map(|bit| bit.to_digit(2).unwrap_or_else(|| panic!("could not parse {}", bit)))
        .collect();

    bits
}

pub fn solution_a(input: &str) -> String
{
    let bin_numbers = parse(input);
    let bin_width = bin_numbers[0].len();
    let cutoff : u32 = (bin_numbers.len()/2 + 1).try_into().unwrap();
    let bitcount = get_bitcount(&bin_numbers, bin_width);
    let gamma_epsilon = get_gamma_epsilon(bitcount, cutoff);    
    let gamma = to_number(&gamma_epsilon.0);
    let epsilon = to_number(&gamma_epsilon.1); 
    
    (gamma * epsilon).to_string()
}

fn get_bitcount(bin_numbers: &[Vec<u32>], bin_width: usize) -> Vec<u32>
{
    let mut bitcount: Vec<u32> = vec![0; bin_width];
    for bin_number in bin_numbers.iter()
    {
        for (position, bit) in (*bin_number).iter().enumerate()
        {
            bitcount[position] += *bit;
        }
    }

    bitcount
}

fn get_gamma_epsilon(bitcount: Vec<u32>, cutoff: u32) -> (Vec<u32>, Vec<u32>)
{
    let bin_width = bitcount.len();
    let mut gamma_epsilon = (vec![0; bin_width], vec![0; bin_width]);
    for (pos, bit) in bitcount.iter().enumerate()
    {
        let x = bitcount[pos];
        if x < cutoff //'0' is most common bit
        {
            gamma_epsilon.0[pos] = 0;
            gamma_epsilon.1[pos] = 1;
        }
        else //'1' is the most common bit
        {
            gamma_epsilon.0[pos] = 1;
            gamma_epsilon.1[pos] = 0;
        }
    }
    
    gamma_epsilon
}

fn to_number(num_bin: &[u32]) -> u32
{
    num_bin.iter().fold(0, |acc, digit| (acc << 1) + digit)
}


#[derive(PartialEq)]
enum SortType
{
    MostCommon,
    LeastCommon
}

pub fn solution_b(input: &str) -> String
{
    let oxygen_numbers = parse(input);
    let scrubber_numbers = oxygen_numbers.clone();

    let oxygen_rating = get_rating(oxygen_numbers, SortType::MostCommon);
    let scrubber_rating = get_rating(scrubber_numbers, SortType::LeastCommon);

    (oxygen_rating * scrubber_rating).to_string()
}

fn get_rating(mut numbers: Vec<Vec<u32>>, sort: SortType) -> u32
{
    let mut bitplace : usize = 0;
    while numbers.len() > 1
    {
        let most_common_bit = get_most_common_bit(&numbers, &bitplace);

        numbers.retain(|n| match sort
        {
            SortType::MostCommon => (*n)[bitplace] == most_common_bit,
            SortType::LeastCommon => (*n)[bitplace] != most_common_bit
        });

        bitplace += 1;
    }

    to_number(&numbers[0])
}

fn get_most_common_bit(numbers: &Vec<Vec<u32>>, bitplace: &usize) -> u32
{
    let mut bitcount: u32 = 0;
    for bin_number in numbers.iter()
    {
        let bit = (*bin_number)[*bitplace];
        bitcount += bit;
    }

    let number_count : u32 = numbers.len().try_into().unwrap();
    let even = number_count % 2 == 0;
    let half = match even
    {
        true => number_count/2,
        false => number_count/2 + 1
    };
    
    match bitcount >= half
    {
        true => 1,
        false => 0
    }
}