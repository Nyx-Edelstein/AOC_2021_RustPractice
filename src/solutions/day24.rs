use std::collections::HashMap;

pub fn solution_a(input: &str) -> String
{
    solve(input, false)
}

pub fn solution_b(input: &str) -> String
{
    solve(input, true)
}

fn solve(input: &str, reverse: bool) -> String
{
    let instructions = Instruction::parse(input);
    let result = solve_recursive(&instructions, 0, [0,0,0,0], &mut HashMap::new(), reverse);
    result.unwrap().to_string().chars().rev().collect::<String>()
}

fn solve_recursive
(
    instructions: &[Instruction],
    counter: usize,
    registers: [i64; 4],
    cache: &mut HashMap<(usize, [i64;4]), Option<i64>>,
    reverse: bool
) -> Option<i64>
{
    if let Some(result) = cache.get(&(counter, registers)) { return *result; }

    let digits = match reverse
    {
        true => [1, 2, 3, 4, 5, 6, 7, 8, 9],
        false => [9, 8, 7, 6, 5, 4, 3, 2, 1]
    };
    'next_digit: for digit in digits
    {
        //First instruction is always "inp"
        let mut registers = registers;
        let mut counter = counter;
        instructions[counter].eval(&mut registers, Some(digit));
        counter += 1;

        while let Some(instruction) = instructions.get(counter)
        {
            if !matches!(instruction, Instruction::inp(_))
            {
                instruction.eval(&mut registers, None);
                counter += 1;
            }
            else if let Some(result) = solve_recursive(instructions, counter, registers, cache, reverse)
            {
                let r = result * 10 + digit;
                cache.insert((counter, registers), Some(r));
                return Some(r);
            }
            else
            {
                continue 'next_digit;
            }
        }

        //"valid" if register z is 0
        if registers[3] == 0
        {
            cache.insert((counter, registers), Some(digit));
            return Some(digit);
        }
    }

    cache.insert((counter, registers), None);
    None
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Argument
{
    //w=0, x=1, y=2, z=3
    //Stored as usize so we can easily index into an array to get register values
    Register(usize),
    Literal(i64)
}

impl Argument
{
    fn get_value(&self, registers: &[i64; 4]) -> i64
    {
        match self
        {
            Argument::Register(r) => registers[*r],
            Argument::Literal(v) => *v,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Instruction
{
    inp(usize),
    add(usize, Argument),
    mul(usize, Argument),
    div(usize, Argument),
    mdl(usize, Argument),
    eql(usize, Argument)
}

impl Instruction
{
    fn parse(input: &str) -> Vec<Instruction>
    {
        let str_to_reg = |s: &str|
        {
            match s
            {
                "w" => 0usize,
                "x" => 1,
                "y" => 2,
                "z" => 3,
                _ => panic!("invalid input")
            }
        };

        input.lines().map(|line|
        {
            let s = line.split(' ').collect::<Vec<_>>();
            let op = s[0];
            let a = str_to_reg(s[1]);
            let b = s.get(2).map(|&s|
            {
                if s.chars().all(|c| c.is_alphabetic()) { Argument::Register(str_to_reg(s)) }
                else { Argument::Literal(s.parse::<i64>().unwrap()) }
            });
            match (op, a, b)
            {
                ("inp", a, None) => Instruction::inp(a),
                ("add", a, Some(b)) => Instruction::add(a, b),
                ("mul", a, Some(b)) => Instruction::mul(a, b),
                ("div", a, Some(b)) => Instruction::div(a, b),
                ("mod", a, Some(b)) => Instruction::mdl(a, b),
                ("eql", a, Some(b)) => Instruction::eql(a, b),
                _ => panic!("invalid opcode")
            }
        }).collect::<Vec<_>>()
    }

    fn eval(&self, registers: &mut [i64; 4], input: Option<i64>)
    {
        match self
        {
            Instruction::inp(a) => registers[*a] = input.unwrap(),
            Instruction::add(a, b) => registers[*a] += b.get_value(registers),
            Instruction::mul(a, b) => registers[*a] *= b.get_value(registers),
            Instruction::div(a, b) => registers[*a] /= b.get_value(registers),
            Instruction::mdl(a, b) => registers[*a] %= b.get_value(registers),
            Instruction::eql(a, b) => registers[*a] = if registers[*a] == b.get_value(registers) {1} else {0}
        }
    }
}
