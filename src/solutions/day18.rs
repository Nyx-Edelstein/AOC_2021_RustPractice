pub fn solution_a(input: &str) -> String
{
    let numbers = parse(input);
    let mut number = numbers[0].clone();
    for n in numbers.iter().skip(1)
    {
        number = number.add(n);
    }    
    number.magnitude().to_string()
}

pub fn solution_b(input: &str) -> String
{
    let numbers = parse(input);
    let mut largest = 0;
    for number in numbers.iter()
    {
        for other in numbers.iter()
        {
            if std::ptr::eq(number, other) { continue; }
            
            let result = number.clone().add(other);
            let magnitude = result.magnitude();
            if magnitude > largest { largest = magnitude; }
        }
    }
    largest.to_string()
}

#[derive(Clone)]
struct SnailfishElement
{
    value: u32,
    depth: u32
}

#[derive(Clone)]
struct SnailfishNumber
{
    elements: Vec<SnailfishElement>
}

impl SnailfishNumber
{
    fn new() -> Self
    {
        Self { elements : Vec::new() }
    }

    fn parse(line: &str) -> Self
    {
        let mut elements = Vec::new();
        let mut depth = 0;
        for c in line.chars()
        {
            match c
            {
                '[' => { depth += 1 },
                ']' => { depth -= 1 },
                ',' => { },
                _ =>
                {
                    let value = c.to_digit(10).unwrap();
                    elements.push(SnailfishElement { value, depth: depth-1 });
                }
            }
        }
        SnailfishNumber { elements }
    }

    fn reduce(mut self) -> Self
    {
        loop
        {           
            if self.explode() { continue; }
            if !self.split() { break; }
        }
        self
    }

    fn explode(&mut self) -> bool
    {
        let Some(i) = (0..self.elements.len()).find(|i| self.elements[*i].depth == 4) else { return false; };

        //Add left to left neighbor
        if i > 0 { self.elements[i-1].value += self.elements[i].value }

        //Add right to right neighbor
        if i + 2 < self.elements.len() { self.elements[i+2].value += self.elements[i+1].value }

        //Simplify the number
        self.elements[i].value = 0;
        self.elements[i].depth -= 1;
        self.elements.remove(i+1);
        true
    }

    fn split(&mut self) -> bool
    {
        let Some(i) = (0..self.elements.len()).find(|i| self.elements[*i].value >= 10) else { return false; };

        let half = self.elements[i].value/2;
        let carry = self.elements[i].value%2;

        self.elements[i].value = half;
        self.elements[i].depth += 1;

        let new_element = SnailfishElement { value: half + carry, depth: self.elements[i].depth };
        self.elements.insert(i+1, new_element);

        true
    }

    fn add(mut self, other: &SnailfishNumber) -> Self
    {
        self.elements.extend(other.elements.clone());
        for i in 0..self.elements.len() { self.elements[i].depth += 1; }
        self.reduce()
    }

    fn magnitude(&self) -> u32
    {
        let mut elements = self.elements.clone();
        while elements.len() > 1
        {
            let i = (0..elements.len()-1).find(|i| elements[*i].depth == elements[*i+1].depth).unwrap();
            elements[i].value = 3*elements[i].value + 2*elements[i+1].value;
            if elements[i].depth > 0 { elements[i].depth -= 1; }
            elements.remove(i+1);
        }
        elements[0].value
    }
}

fn parse(input: &str) -> Vec<SnailfishNumber>
{
    input.lines().map(SnailfishNumber::parse).collect()
}