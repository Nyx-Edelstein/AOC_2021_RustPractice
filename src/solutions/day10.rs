use itertools::Itertools;

fn parse(input: &str) -> Vec<Chunk>
{
    input.lines()
        .map(Chunk::new)
        .collect()
}

#[derive(Debug)]
struct Chunk
{
    tokens: String
}

impl Chunk
{
    fn new(line: &str) -> Self
    {
        Chunk { tokens: line.to_owned() }
    }

    fn scan_for_error(mut self) -> Option<char>
    {
        self = self.simplify();        
        if let Some(pos) = self.tokens.find([')', ']', '}', '>'])
        {
            return self.tokens.chars().nth(pos)
        }
        None
    }

    fn simplify(mut self) -> Self
    {
        let mut prev_len = 0;
        while prev_len != self.tokens.len()
        {
            prev_len = self.tokens.len();
            self.tokens = self.tokens.replace("()", "")
                .replace("[]", "")
                .replace("{}", "")
                .replace("<>", "");
        }
        self
    }

    fn validate(mut self) -> Option<Self>
    {
        self = self.simplify();
        match self.tokens.contains([')', ']', '}', '>'])
        {
            true => None,
            false => Some(self)
        }
    }

    fn complete(self) -> String
    {
        self.tokens
            .replace('(', ")")
            .replace('[', "]")
            .replace('{', "}")
            .replace('<', ">")
            .chars()
            .rev()
            .collect()
    }
}

pub fn solution_a(input: &str) -> String
{
    let chunks = parse(input);
    let invalid_chunk_sum = chunks.into_iter()
        .filter_map(|chunk| chunk.scan_for_error())
        .map(|c| match c
        {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!("error character is incorrect somehow")
        }).sum::<u32>();

    invalid_chunk_sum.to_string()
}

pub fn solution_b(input: &str) -> String
{
    let chunks = parse(input);
    let completed_chunk_scores = chunks.into_iter()
        .filter_map(|chunk| chunk.validate())
        .map(|chunk| chunk.complete())
        .map(get_score)
        .sorted()
        .collect::<Vec<_>>();

    let middle_score = completed_chunk_scores[completed_chunk_scores.len()/2];
    middle_score.to_string()
}

fn get_score(compl_str: String) -> u64
{
    compl_str.chars().fold(0, |score, c| match c
    {
        ')' => score * 5 + 1,
        ']' => score * 5 + 2,
        '}' => score * 5 + 3,
        '>' => score * 5 + 4,
        _ => unreachable!("invalid input")
    })
}

