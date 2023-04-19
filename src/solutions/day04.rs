struct BingoBoard
{
    board: [(u32, bool); 25],
    numbers_called : u32,
    last_called : u32
}

impl BingoBoard
{
    fn new(input : &str) -> Self
    {
        let b : [(u32, bool); 25] = input.split_whitespace()
            .map(|data|
            {
                (data.parse::<u32>().unwrap(), false)
            }).collect::<Vec<_>>()
            .try_into()
            .unwrap();
        
        Self {board: b, numbers_called: 0, last_called: 0}
    }

    fn call_number(mut self, n: &u32) -> Self
    {
        if self.has_bingo()
        {
            return self
        }

        //Find if the board contains the number
        let found_positions : Vec<_>  = self.board.iter()
            .enumerate()
            .filter(|(pos, b)| b.0 == *n)
            .map(|(pos, b)| pos)
            .collect::<Vec<_>>();
        let found_position = found_positions.first();
        
        //Mark the position if found
        if let Some(&pos) = found_position
        {
            self.board[pos].1 = true;
        }

        self.numbers_called += 1;
        self.last_called = *n;
        self
    }

    fn get_score(&self, last_called : u32) -> Option<u32>
    {
        let has_bingo = self.has_bingo();
        if !has_bingo
        {
            return None;
        }

        //Calculate score
        let sum_unmarked : u32 = self.board.iter()
            .filter(|(n, marked)| !(*marked))
            .map(|(n, marked)| *n)
            .sum();

        Some(sum_unmarked * last_called)
    }

    fn has_bingo(&self) -> bool
    {
        //Any row?
        for i in 0..5
        {
            let row = [self.board[5*i], self.board[5*i+1], self.board[5*i+2], self.board[5*i+3], self.board[5*i+4]]
                .iter()
                .filter(|(n, marked)| *marked)
                .count();
            if row == 5
            {
                return true
            }
        }

        //Any col?
        for i in 0..5
        {
            let col = [self.board[i], self.board[5+i], self.board[10+i], self.board[15+i], self.board[20+i]]
                .iter()
                .filter(|(n, marked)| *marked)
                .count();
            if col == 5
            {
                return true
            }
        }

        false
    }
}

fn parse(input : &str) -> (Vec<u32>, Vec<BingoBoard>)
{
    let data : Vec<&str> = input.split("\r\n\r\n").collect();
    let numbers : Vec<u32> = data[0].split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let boards = data[1..].iter()
        .map(|&s| BingoBoard::new(s))
        .collect();

    (numbers, boards)
}

pub fn solution_a(input: &str) -> String
{
    let (numbers, mut boards) = parse(input);

    for n in numbers
    {
        boards = boards.into_iter()
            .map(|b| b.call_number(&n))
            .collect();

        let scores : Vec<_> = boards.iter()
            .map(|b| b.get_score(n))
            .filter(|score| (*score).is_some())
            .map(|x| x.unwrap())
            .collect();

        let score = scores.first();
        if let Some(&s) = score
        {
            return s.to_string();
        }
    }

    unreachable!("Did not find a bingo (should not happen!)");
}

pub fn solution_b(input: &str) -> String
{
    let (numbers, mut boards) = parse(input);

    for n in numbers
    {
        boards = boards.into_iter()
            .map(|b| b.call_number(&n))
            .collect();        
    }

    boards.sort_by_key(|b| b.numbers_called);
    let last_winning_board = boards.last().unwrap();
    let score = last_winning_board.get_score(last_winning_board.last_called).unwrap();
    
    score.to_string()
}