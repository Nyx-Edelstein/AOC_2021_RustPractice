use std::{collections::{HashMap, BinaryHeap}, hash::{Hash, Hasher}};

pub fn solution(input: &str) -> String
{
    let init_grid = Grid::parse(input);
    let path_map = get_path_map();
    let mut best_solution = usize::MAX;
    let mut to_consider = BinaryHeap::new();
    to_consider.push(init_grid);
    let mut considered : HashMap<Grid, usize> = HashMap::new();
    while let Some(current) = to_consider.pop()
    {
        considered.insert(current, current.cost);

        if current.is_solved()
        {
            if current.cost < best_solution { best_solution = current.cost; }
            continue;
        }
        if current.cost > best_solution { continue; }

        let moves = current.get_valid_moves(&path_map);
        for next in Grid::generate_from_moves(&current, moves)
        {
            if considered.contains_key(&next)
            { 
                let &lowest_cost = considered.get(&next).unwrap();
                if next.cost < lowest_cost
                {
                    considered.insert(next, next.cost);
                }
                else { continue; }
            }
            if next.cost < best_solution { to_consider.push(next) }
        }
    }
    best_solution.to_string()
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Room
{
    /*
    #############################################
    #_00 _01 _02 _03 _04 _05 _06 _07 _08 _09 _10#
    ######## 2A1 ### 4B1 ### 6C1 ### 8D1 ########
    ######## 2A2 ### 4B2 ### 6C2 ### 8D2 ########
    #############################################
    */
    _00 = 0,
    _01 = 1,
    //_02,
    _03 = 2,
    //_04,
    _05 = 3,
    //_06,
    _07 = 4,
    //_08,
    _09 = 5,
    _10 = 6,
    _02_A1 = 7,
    _02_A2 = 8,
    _04_B1 = 9,
    _04_B2 = 10,
    _06_C1 = 11,
    _06_C2 = 12,
    _08_D1 = 13,
    _08_D2 = 14
}

impl Room
{
    fn from(i: usize) -> Room
    {
        match i
        {
            0 => Room::_00,
            1 => Room::_01,
            2 => Room::_03,
            3 => Room::_05,
            4 => Room::_07,
            5 => Room::_09,
            6 => Room::_10,
            7 => Room::_02_A1,
            8 => Room::_02_A2,
            9 => Room::_04_B1,
            10 => Room::_04_B2,
            11 => Room::_06_C1,
            12 => Room::_06_C2,
            13 => Room::_08_D1,
            14 => Room::_08_D2,
            _ => unreachable!("out of range")
        }
    }

    fn is_hall(self) -> bool
    {
        matches!(self, Room::_00 | Room::_01 | Room::_03 | Room::_05 | Room::_07 | Room::_09 | Room::_10)
    }

    fn is_matching_type(self, amphipod: Amphipod) -> bool
    {
        match amphipod
        {
            Amphipod::None => false,
            Amphipod::Amber => matches!(self, Room::_02_A1 | Room::_02_A2),
            Amphipod::Bronze => matches!(self, Room::_04_B1 | Room::_04_B2),
            Amphipod::Copper => matches!(self, Room::_06_C1 | Room::_06_C2),
            Amphipod::Desert => matches!(self, Room::_08_D1 | Room::_08_D2),
        }
    }

    fn is_2nd_room(self) -> bool
    {
        matches!(self, Room::_02_A2 | Room::_04_B2 | Room::_06_C2 | Room::_08_D2)
    }

    fn get_second_room(self) -> Option<Room>
    {
        match self
        {
            Room::_02_A1 => Some(Room::_02_A2),
            Room::_04_B1 => Some(Room::_04_B2),
            Room::_06_C1 => Some(Room::_06_C2),
            Room::_08_D1 => Some(Room::_08_D2),
            _ => None
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum Amphipod
{
    None = 0,
    Amber = 1,
    Bronze = 10,
    Copper = 100,
    Desert = 1000
}

impl Amphipod
{
    fn to_char(self) -> char
    {
        match self
        {
            Amphipod::None => '.',
            Amphipod::Amber => 'A',
            Amphipod::Bronze => 'B',
            Amphipod::Copper => 'C',
            Amphipod::Desert => 'D',
        }
    }

    fn from_char(c: char) -> Self
    {
        match c
        {
            'A' => Amphipod::Amber,
            'B' => Amphipod::Bronze,
            'C' => Amphipod::Copper,
            'D' => Amphipod::Desert,
            _ => Amphipod::None   
        }
    }
}

#[derive(Debug, Clone)]
struct Path
{
    start: Room,
    end: Room,
    path: Vec<Room>,
    dist: usize
}

impl Path
{
    fn is_valid(&self, grid: &Grid, amphipod: Amphipod) -> bool
    {
        //If the amphipod is already in the correct type room, it might not need to move
        if self.start.is_matching_type(amphipod)
        {
            //If the amphipod is already in the 2nd spot, it doesn't need to move
            if self.start.is_2nd_room() { return false; }

            //If the amphipod is in the 1st spot, it doesn't need to move if the 2nd is also correct
            let second_room = self.start.get_second_room().unwrap();
            let second_room_inhabitant = grid.get_room_occupant(second_room);
            if second_room.is_matching_type(second_room_inhabitant) { return false; }
        }        

        //All spots in the path must be empty
        if self.path.iter().any(|&room| !grid.room_is_empty(room)) { return false; }

        //If the destination is a hallway, it's valid
        if self.end.is_hall() { return true; }

        //Otherwise, the destination must match the amphipod type
        if !self.end.is_matching_type(amphipod) { return false; }

        //Given matching type, the destination must either be the 2nd room OR the 1st room while the 2nd is occupied with the correct type
        if self.end.is_2nd_room() { return true; }

        let second_room = self.end.get_second_room().unwrap();
        let second_room_occupant = grid.get_room_occupant(second_room);
        self.end.is_matching_type(second_room_occupant)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Grid
{
    state: [Amphipod; 15],
    cost: usize
}

impl Grid
{
    fn parse(input: &str) -> Self
    {
        let mut state = [Amphipod::None; 15];
        let lines = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
        state[Room::_02_A1 as usize] = Amphipod::from_char(lines[2][3]);
        state[Room::_02_A2 as usize] = Amphipod::from_char(lines[3][3]);
        state[Room::_04_B1 as usize] = Amphipod::from_char(lines[2][5]);
        state[Room::_04_B2 as usize] = Amphipod::from_char(lines[3][5]);
        state[Room::_06_C1 as usize] = Amphipod::from_char(lines[2][7]);
        state[Room::_06_C2 as usize] = Amphipod::from_char(lines[3][7]);
        state[Room::_08_D1 as usize] = Amphipod::from_char(lines[2][9]);
        state[Room::_08_D2 as usize] = Amphipod::from_char(lines[3][9]);
        Grid { state, cost: 0 }
    }

    fn get_room_occupant(&self, room: Room) -> Amphipod
    {
        let index = room as usize;
        self.state[index]
    }

    fn room_is_empty(&self, room: Room) -> bool
    {
        self.get_room_occupant(room) == Amphipod::None
    }

    fn get_valid_moves(&self, path_map: &HashMap<Room, Vec<Path>>) -> Vec<(Path, usize)>
    {
        let mut moves : Vec<(Path, usize)> = Vec::new();
        for i in 0usize..=14
        {
            let amphipod = self.state[i];
            if amphipod == Amphipod::None { continue; }

            let room = Room::from(i);
            let mut valid_moves = path_map.get(&room)
                .unwrap()
                .iter()
                .filter(|path| path.is_valid(self, amphipod))
                .map(|path| (path.clone(), path.dist * amphipod as usize))
                .collect::<Vec<_>>();

            moves.append(&mut valid_moves);
        }

        //If any moves result in an amphipod going to it's destination, only consider such terminal moves
        if moves.iter().any(|(path, cost)|
        {
            path.end.is_matching_type(self.state[path.start as usize])
        })
        {
            moves.into_iter()
                .filter(|(path, cost)| path.end.is_matching_type(self.state[path.start as usize]))
                .collect::<Vec<_>>()
        }
        else
        {
            moves
        }
    }

    fn generate_from_moves(&self, moves: Vec<(Path, usize)>) -> Vec<Grid>
    {
        moves.into_iter().map(|(path, cost)|
        {
            let mut new_grid = *self;
            let amphipod = new_grid.state[path.start as usize];
            new_grid.state[path.start as usize] = Amphipod::None;
            new_grid.state[path.end as usize] = amphipod;
            new_grid.cost += cost;
            new_grid
        }).collect::<Vec<_>>()
    }

    fn is_solved(&self) -> bool
    {
        self.state[Room::_02_A1 as usize] == Amphipod::Amber
        && self.state[Room::_02_A2 as usize] == Amphipod::Amber
        && self.state[Room::_04_B1 as usize] == Amphipod::Bronze
        && self.state[Room::_04_B2 as usize] == Amphipod::Bronze
        && self.state[Room::_06_C1 as usize] == Amphipod::Copper
        && self.state[Room::_06_C2 as usize] == Amphipod::Copper
        && self.state[Room::_08_D1 as usize] == Amphipod::Desert
        && self.state[Room::_08_D2 as usize] == Amphipod::Desert
    }

    fn num_correct(&self) -> usize
    {
        let mut num_correct = 0;

        if self.state[Room::_02_A1 as usize] == Amphipod::Amber && self.state[Room::_02_A2 as usize] == Amphipod::Amber { num_correct += 2; }
        else if self.state[Room::_02_A1 as usize] == Amphipod::None && self.state[Room::_02_A2 as usize] == Amphipod::Amber { num_correct += 1;}

        if self.state[Room::_04_B1 as usize] == Amphipod::Bronze && self.state[Room::_04_B2 as usize] == Amphipod::Bronze { num_correct += 2; }
        else if self.state[Room::_04_B1 as usize] == Amphipod::None && self.state[Room::_04_B2 as usize] == Amphipod::Bronze { num_correct += 1;}

        if self.state[Room::_06_C1 as usize] == Amphipod::Copper && self.state[Room::_06_C2 as usize] == Amphipod::Copper { num_correct += 2; }
        else if self.state[Room::_06_C1 as usize] == Amphipod::None && self.state[Room::_06_C2 as usize] == Amphipod::Copper { num_correct += 1;}

        if self.state[Room::_08_D1 as usize] == Amphipod::Desert && self.state[Room::_08_D2 as usize] == Amphipod::Desert { num_correct += 2; }
        else if self.state[Room::_08_D1 as usize] == Amphipod::None && self.state[Room::_08_D2 as usize] == Amphipod::Desert { num_correct += 1;}

        num_correct
    }
}

impl Ord for Grid
{
    fn cmp(&self, other: &Grid) -> std::cmp::Ordering
    {
        self.num_correct().cmp(&other.num_correct())
            .then_with(|| other.cost.cmp(&self.cost))
    }
}

impl PartialOrd for Grid
{
    fn partial_cmp(&self, other: &Grid) -> Option<std::cmp::Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Hash for Grid
{
    fn hash<H: Hasher>(&self, s: &mut H)
    {
        self.state.hash(s)
    }
}

fn connecting_rooms(room: Room) -> Vec<(Room, usize)>
{
    match room
    {
        Room::_00 => vec![(Room::_01, 1)],
        Room::_01 => vec![(Room::_00, 1), (Room::_02_A1, 2), (Room::_03, 2)],
        Room::_02_A1 => vec![(Room::_02_A2, 1), (Room::_01, 2), (Room::_03, 2)],
        Room::_02_A2 => vec![(Room::_02_A1, 1)],
        Room::_03 => vec![(Room::_01, 2), (Room::_02_A1, 2), (Room::_04_B1, 2), (Room::_05, 2)],
        Room::_04_B1 => vec![(Room::_04_B2, 1), (Room::_03, 2), (Room::_05, 2)],
        Room::_04_B2 => vec![(Room::_04_B1, 1)],
        Room::_05 => vec![(Room::_03, 2), (Room::_04_B1, 2), (Room::_06_C1, 2), (Room::_07, 2)],
        Room::_06_C1 => vec![(Room::_06_C2, 1), (Room::_05, 2), (Room::_07, 2)],
        Room::_06_C2 => vec![(Room::_06_C1, 1)],
        Room::_07 => vec![(Room::_05, 2), (Room::_06_C1, 2), (Room::_08_D1, 2), (Room::_09, 2)],
        Room::_08_D1 => vec![(Room::_08_D2, 1), (Room::_07, 2), (Room::_09, 2)],
        Room::_08_D2 => vec![(Room::_08_D1, 1)],
        Room::_09 => vec![(Room::_07, 2), (Room::_08_D1, 2), (Room::_10, 1)],
        Room::_10 => vec![(Room::_09, 1)]
    }
}

fn get_path_map() -> HashMap<Room, Vec<Path>>
{
    const ROOMS : [Room;15] =
    [
        Room::_00, Room::_01, Room::_03, Room::_05, Room::_07, Room::_09, Room::_10,
        Room::_02_A1, Room::_02_A2, Room::_04_B1, Room::_04_B2, Room::_06_C1, Room::_06_C2, Room::_08_D1, Room::_08_D2
    ];

    let mut paths_by_room : HashMap<Room, Vec<Path>> = HashMap::new();
    for room1 in ROOMS
    {
        paths_by_room.insert(room1, Vec::new());
        for room2 in ROOMS
        {
            //Disallow connections between two halls
            //If an amphipod moves into a hall, it must continue moving and then stop, so we can simplify this to a single move
            if room1 == room2 { continue; }
            if room1.is_hall() && room2.is_hall() { continue; }

            let path = find_path(room1, room2);
            paths_by_room.get_mut(&room1).unwrap().push(path);
        }
    }
    paths_by_room
}

fn find_path(room1: Room, room2: Room) -> Path
{
    let mut found_path : Vec<(Room, usize)> = Vec::new();
    let mut visited = vec![room1];
    let mut to_consider = vec![vec![(room1, 0)]];
    while let Some(current) = to_consider.pop()
    {
        let &(last_room, last_dist) = current.last().unwrap();

        let connections = connecting_rooms(last_room)
            .into_iter()
            .filter(|(room, cost)| !visited.contains(room))
            .collect::<Vec<_>>();

        connections.iter().for_each(|&connection|
        {
            let mut new_path = current.to_vec();
            new_path.push(connection);

            if (connection.0) == room2
            { 
                found_path = new_path;
                return;
            }

            to_consider.push(new_path);
            visited.push(connection.0);
        });

        if !found_path.is_empty() { break; }
    }

    let path = found_path.iter().skip(1).map(|(room, dist)| room).copied().collect::<Vec<_>>();
    let dist = found_path.iter().map(|(room, dist)| dist).sum();
    Path { start: room1, end: room2, path, dist }
}