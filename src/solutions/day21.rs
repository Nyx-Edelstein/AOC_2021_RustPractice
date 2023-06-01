use std::{collections::HashMap};

fn parse(input: &str) -> [u32; 2]
{
    let (p1, p2) = input.split_once("\r\n").unwrap();
    let p1 = p1.split_once(": ").unwrap().1.parse::<u32>().unwrap();
    let p2 = p2.split_once(": ").unwrap().1.parse::<u32>().unwrap();
    [p1, p2]
}

pub fn solution_a(input: &str) -> String
{
    let mut positions = parse(input);
    let mut scores = vec![0, 0];
    let mut dice_value = 1;
    let mut num_rolls = 0;
    let mut turn = |i: usize|
    {
        positions[i] += 3 * (dice_value + 1);
        while positions[i] > 10 { positions[i] -= 10; }

        dice_value += 3;
        if dice_value > 100 { dice_value -= 100; }        

        num_rolls += 3;
        scores[i] += positions[i];
        scores[i]
    };
    loop
    {
        if turn(0) >= 1000 { break; }
        if turn(1) >= 1000 { break; }
    }
    (scores[1] * num_rolls).to_string()
}

pub fn solution_b(input: &str) -> String
{
    let mut cache: HashMap<GameState, (u64, u64)> = HashMap::new();
    let state = GameState::new(input);
    let scores = solve_recursive(state, &mut cache);
    scores.0.max(scores.1).to_string()
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct GameState
{
    positions: [u32; 2],
    scores: [u32; 2],
    turn_num: usize
}

impl GameState
{
    fn new(input: &str) -> Self
    {
        let positions = parse(input);
        GameState { positions, scores: [0,0], turn_num: 0 }
    }
}

fn solve_recursive(state: GameState, cache: &mut HashMap<GameState, (u64, u64)>) -> (u64, u64)
{
    //3d3 dice rolls as (total, frequency)
    const DIRAC_DICE: [(u32, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    //Base case
    if state.scores[0] >= 21 { return (1, 0); }
    if state.scores[1] >= 21 { return (0, 1); }

    //Recursive case
    let mut accumulated = (0, 0);
    for (total, frequency) in DIRAC_DICE
    {
        let mut next = state.clone();            
        next.positions[next.turn_num] += total;
        while next.positions[next.turn_num] > 10 { next.positions[next.turn_num] -= 10; }
        next.scores[next.turn_num] += next.positions[next.turn_num];
        next.turn_num = (next.turn_num + 1) % 2;

        let (p1_wins, p2_wins) = match cache.get(&next)
        {
            Some(result) => *result,
            None =>
            {
                let result = solve_recursive(next.clone(), cache);
                cache.insert(next, result);
                result
            }
        };
        accumulated.0 += p1_wins * frequency;
        accumulated.1 += p2_wins * frequency;
    }
    accumulated
}

