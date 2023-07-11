use std::{collections::{HashMap, BinaryHeap}, hash::{Hash, Hasher}};

//Note: Part B changes the topology of the grid in a way that is difficult to generalize over, hence a different solution

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
    ######## 2A3 ### 4B3 ### 6C3 ### 8D3 ########
    ######## 2A4 ### 4B4 ### 6C4 ### 8D4 ########
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
    _02_A3 = 9,
    _02_A4 = 10,
    _04_B1 = 11,
    _04_B2 = 12,
    _04_B3 = 13,
    _04_B4 = 14,
    _06_C1 = 15,
    _06_C2 = 16,
    _06_C3 = 17,
    _06_C4 = 18,
    _08_D1 = 19,
    _08_D2 = 20,
    _08_D3 = 21,
    _08_D4 = 22
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
            9 => Room::_02_A3,
            10 => Room::_02_A4,
            11 => Room::_04_B1,
            12 => Room::_04_B2,
            13 => Room::_04_B3,
            14 => Room::_04_B4,
            15 => Room::_06_C1,
            16 => Room::_06_C2,
            17 => Room::_06_C3,
            18 => Room::_06_C4,
            19 => Room::_08_D1,
            20 => Room::_08_D2,
            21 => Room::_08_D3,
            22 => Room::_08_D4,
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
            Amphipod::Amber => matches!(self, Room::_02_A1 | Room::_02_A2 | Room::_02_A3 | Room::_02_A4),
            Amphipod::Bronze => matches!(self, Room::_04_B1 | Room::_04_B2 | Room::_04_B3 | Room::_04_B4),
            Amphipod::Copper => matches!(self, Room::_06_C1 | Room::_06_C2 | Room::_06_C3 | Room::_06_C4),
            Amphipod::Desert => matches!(self, Room::_08_D1 | Room::_08_D2 | Room::_08_D3 | Room::_08_D4),
        }
    }

    fn is_first_room(&self) -> bool
    {
        matches!(self, Room::_02_A1 | Room::_04_B1 | Room::_06_C1 | Room::_08_D1)
    }

    fn is_second_room(&self) -> bool
    {
        matches!(self, Room::_02_A2 | Room::_04_B2 | Room::_06_C2 | Room::_08_D2)
    }

    fn is_third_room(&self) -> bool
    {
        matches!(self, Room::_02_A3 | Room::_04_B3 | Room::_06_C3 | Room::_08_D3)
    }

    fn is_fourth_room(&self) -> bool
    {
        matches!(self, Room::_02_A4 | Room::_04_B4 | Room::_06_C4 | Room::_08_D4)
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

    fn to_str(self) -> String
    {
        match self
        {
            Amphipod::None => ".",
            Amphipod::Amber => "A",
            Amphipod::Bronze => "B",
            Amphipod::Copper => "C",
            Amphipod::Desert => "D",
        }.to_owned()
    }
}

#[derive(Debug, Clone)]
struct Path
{
    start: Room,
    end: Room,
    route: Vec<Room>,
    dist: usize
}

#[derive(Copy, Clone)]
struct Move
{
    start: Room,
    end: Room,
    cost: usize
}

#[derive(Copy, Clone, Debug)]
struct Grid
{
    state: [Amphipod; 23],
    cost: usize
}

impl Grid
{
    fn parse(input: &str) -> Self
    {
        let mut state = [Amphipod::None; 23];
        let lines = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
        state[Room::_02_A1 as usize] = Amphipod::from_char(lines[2][3]);
        state[Room::_02_A4 as usize] = Amphipod::from_char(lines[3][3]);
        state[Room::_04_B1 as usize] = Amphipod::from_char(lines[2][5]);
        state[Room::_04_B4 as usize] = Amphipod::from_char(lines[3][5]);
        state[Room::_06_C1 as usize] = Amphipod::from_char(lines[2][7]);
        state[Room::_06_C4 as usize] = Amphipod::from_char(lines[3][7]);
        state[Room::_08_D1 as usize] = Amphipod::from_char(lines[2][9]);
        state[Room::_08_D4 as usize] = Amphipod::from_char(lines[3][9]);

        //Part B: insert extra lines in rows 2 and 3
        //#D#C#B#A#
        //#D#B#A#C#
        state[Room::_02_A2 as usize] = Amphipod::Desert;
        state[Room::_02_A3 as usize] = Amphipod::Desert;
        state[Room::_04_B2 as usize] = Amphipod::Copper;
        state[Room::_04_B3 as usize] = Amphipod::Bronze;
        state[Room::_06_C2 as usize] = Amphipod::Bronze;
        state[Room::_06_C3 as usize] = Amphipod::Amber;
        state[Room::_08_D2 as usize] = Amphipod::Amber;
        state[Room::_08_D3 as usize] = Amphipod::Copper;

        Grid { state, cost: 0 }
    }

    fn get_valid_moves(&self, path_map: &HashMap<Room, Vec<Path>>) -> Vec<Move>
    {
        let mut moves : Vec<Move> = Vec::new();
        for i in 0usize..=22
        {
            let amphipod = self.state[i];
            if amphipod == Amphipod::None { continue; }

            let room = Room::from(i);
            let mut valid_moves = path_map.get(&room)
                .unwrap()
                .iter()
                .filter(|path| self.route_is_empty(&path.route))
                .filter(|path| self.is_valid_path(path, amphipod))
                .map(|path| Move { start: path.start, end: path.end, cost: path.dist * amphipod as usize})
                .collect::<Vec<_>>();

            moves.append(&mut valid_moves);
        }
        moves
    }

    fn is_valid_path(&self, path: &Path, amphipod: Amphipod) -> bool
    {
        //If the amphipod is already in the correct type room group (and the rooms below it are correct), it doesn't need to move
        if self.room_group_matches(path.start, amphipod) { return false; }

        //If the destination is a hallway, it's valid
        if path.end.is_hall() { return true; }

        //Otherwise, the destination must match the amphipod type (and the rooms below it must be correct)
        self.room_group_matches(path.end, amphipod)
    }

    fn route_is_empty(&self, route: &[Room]) -> bool
    {
        route.iter().map(|&r| self.state[r as usize]).all(|a| a == Amphipod::None)
    }

    fn room_has_amphipod(&self, room: Room, amphipod: Amphipod) -> bool
    {
        self.state[room as usize] == amphipod
    }

    fn room_group_matches(&self, room: Room, amphipod: Amphipod) -> bool
    {
        if !room.is_matching_type(amphipod) { return false; }
        
        match amphipod
        {
            Amphipod::None => false,
            Amphipod::Amber => self.amber_correct_below_room(room),
            Amphipod::Bronze => self.bronze_correct_below_room(room),
            Amphipod::Copper => self.copper_correct_below_room(room),
            Amphipod::Desert => self.desert_correct_below_room(room),
        }
    }

    fn amber_correct_below_room(&self, room: Room) -> bool
    {
        match room
        {
            Room::_02_A1 => self.room_has_amphipod(Room::_02_A2, Amphipod::Amber)
                && self.room_has_amphipod(Room::_02_A3, Amphipod::Amber)
                && self.room_has_amphipod(Room::_02_A4, Amphipod::Amber),
            Room::_02_A2 => self.room_has_amphipod(Room::_02_A3, Amphipod::Amber)
                && self.room_has_amphipod(Room::_02_A4, Amphipod::Amber),
            Room::_02_A3 => self.room_has_amphipod(Room::_02_A4, Amphipod::Amber),
            Room::_02_A4 => true,
            _ => false
        }
    }

    fn bronze_correct_below_room(&self, room: Room) -> bool
    {
        match room
        {
            Room::_04_B1 => self.room_has_amphipod(Room::_04_B2, Amphipod::Bronze)
                && self.room_has_amphipod(Room::_04_B3, Amphipod::Bronze)
                && self.room_has_amphipod(Room::_04_B4, Amphipod::Bronze),
            Room::_04_B2 => self.room_has_amphipod(Room::_04_B3, Amphipod::Bronze)
                && self.room_has_amphipod(Room::_04_B4, Amphipod::Bronze),
            Room::_04_B3 => self.room_has_amphipod(Room::_04_B4, Amphipod::Bronze),
            Room::_04_B4 => true,
            _ => false
        }
    }

    fn copper_correct_below_room(&self, room: Room) -> bool
    {
        match room
        {
            Room::_06_C1 => self.room_has_amphipod(Room::_06_C2, Amphipod::Copper)
                && self.room_has_amphipod(Room::_06_C3, Amphipod::Copper)
                && self.room_has_amphipod(Room::_06_C4, Amphipod::Copper),
            Room::_06_C2 => self.room_has_amphipod(Room::_06_C3, Amphipod::Copper)
                && self.room_has_amphipod(Room::_06_C4, Amphipod::Copper),
            Room::_06_C3 => self.room_has_amphipod(Room::_06_C4, Amphipod::Copper),
            Room::_06_C4 => true,
            _ => false
        }
    }

    fn desert_correct_below_room(&self, room: Room) -> bool
    {
        match room
        {
            Room::_08_D1 => self.room_has_amphipod(Room::_08_D2, Amphipod::Desert)
                && self.room_has_amphipod(Room::_08_D3, Amphipod::Desert)
                && self.room_has_amphipod(Room::_08_D4, Amphipod::Desert),
            Room::_08_D2 => self.room_has_amphipod(Room::_08_D3, Amphipod::Desert)
                && self.room_has_amphipod(Room::_08_D4, Amphipod::Desert),
            Room::_08_D3 => self.room_has_amphipod(Room::_08_D4, Amphipod::Desert),
            Room::_08_D4 => true,
            _ => false
        }
    }

    fn generate_from_moves(&self, moves: Vec<Move>) -> Vec<Grid>
    {
        moves.into_iter().map(|m|
        {
            let mut new_grid = *self;
            let amphipod = new_grid.state[m.start as usize];
            new_grid.state[m.start as usize] = Amphipod::None;
            new_grid.state[m.end as usize] = amphipod;
            new_grid.cost += m.cost;
            new_grid
        }).collect::<Vec<_>>()
    }

    fn is_solved(&self) -> bool
    {
        self.state[Room::_02_A1 as usize] == Amphipod::Amber
        && self.state[Room::_02_A2 as usize] == Amphipod::Amber
        && self.state[Room::_02_A3 as usize] == Amphipod::Amber
        && self.state[Room::_02_A4 as usize] == Amphipod::Amber
        && self.state[Room::_04_B1 as usize] == Amphipod::Bronze
        && self.state[Room::_04_B2 as usize] == Amphipod::Bronze
        && self.state[Room::_04_B3 as usize] == Amphipod::Bronze
        && self.state[Room::_04_B4 as usize] == Amphipod::Bronze
        && self.state[Room::_06_C1 as usize] == Amphipod::Copper
        && self.state[Room::_06_C2 as usize] == Amphipod::Copper
        && self.state[Room::_06_C3 as usize] == Amphipod::Copper
        && self.state[Room::_06_C4 as usize] == Amphipod::Copper
        && self.state[Room::_08_D1 as usize] == Amphipod::Desert
        && self.state[Room::_08_D2 as usize] == Amphipod::Desert
        && self.state[Room::_08_D3 as usize] == Amphipod::Desert
        && self.state[Room::_08_D4 as usize] == Amphipod::Desert
    }

    fn heuristic(&self) -> usize
    {
        let mut num_correct = 0;
        if self.state[Room::_02_A4 as usize] == Amphipod::Amber
        { 
            num_correct += 1;
            if self.state[Room::_02_A3 as usize] == Amphipod::Amber
            {
                num_correct += 1;
                if self.state[Room::_02_A2 as usize] == Amphipod::Amber
                {
                    num_correct += 1;
                    if self.state[Room::_02_A1 as usize] == Amphipod::Amber
                    {
                        num_correct += 1;
                    }
                }
            }
        }
        if self.state[Room::_04_B4 as usize] == Amphipod::Bronze
        { 
            num_correct += 1;
            if self.state[Room::_04_B3 as usize] == Amphipod::Bronze
            {
                num_correct += 1;
                if self.state[Room::_04_B2 as usize] == Amphipod::Bronze
                {
                    num_correct += 1;
                    if self.state[Room::_04_B1 as usize] == Amphipod::Bronze
                    {
                        num_correct += 1;
                    }
                }
            }
        }
        if self.state[Room::_06_C4 as usize] == Amphipod::Copper
        { 
            num_correct += 1;
            if self.state[Room::_06_C3 as usize] == Amphipod::Copper
            {
                num_correct += 1;
                if self.state[Room::_06_C2 as usize] == Amphipod::Copper
                {
                    num_correct += 1;
                    if self.state[Room::_06_C1 as usize] == Amphipod::Copper
                    {
                        num_correct += 1;
                    }
                }
            }
        }
        if self.state[Room::_08_D4 as usize] == Amphipod::Desert
        { 
            num_correct += 1;
            if self.state[Room::_08_D3 as usize] == Amphipod::Desert
            {
                num_correct += 1;
                if self.state[Room::_08_D2 as usize] == Amphipod::Desert
                {
                    num_correct += 1;
                    if self.state[Room::_08_D1 as usize] == Amphipod::Desert
                    {
                        num_correct += 1;
                    }
                }
            }
        }
        num_correct
    }
}

impl Eq for Grid { }
impl PartialEq for Grid
{
    fn eq(&self, other: &Grid) -> bool
    {
        self.state == other.state
    }
}

impl Ord for Grid
{
    fn cmp(&self, other: &Grid) -> std::cmp::Ordering
    {
        self.heuristic().cmp(&other.heuristic())
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
        Room::_02_A2 => vec![(Room::_02_A1, 1), (Room::_02_A3, 1)],
        Room::_02_A3 => vec![(Room::_02_A2, 1), (Room::_02_A4, 1)],
        Room::_02_A4 => vec![(Room::_02_A3, 1)],
        Room::_03 => vec![(Room::_01, 2), (Room::_02_A1, 2), (Room::_04_B1, 2), (Room::_05, 2)],
        Room::_04_B1 => vec![(Room::_04_B2, 1), (Room::_03, 2), (Room::_05, 2)],
        Room::_04_B2 => vec![(Room::_04_B1, 1), (Room::_04_B3, 1)],
        Room::_04_B3 => vec![(Room::_04_B2, 1), (Room::_04_B4, 1)],
        Room::_04_B4 => vec![(Room::_04_B3, 1)],
        Room::_05 => vec![(Room::_03, 2), (Room::_04_B1, 2), (Room::_06_C1, 2), (Room::_07, 2)],
        Room::_06_C1 => vec![(Room::_06_C2, 1), (Room::_05, 2), (Room::_07, 2)],
        Room::_06_C2 => vec![(Room::_06_C1, 1), (Room::_06_C3, 1)],
        Room::_06_C3 => vec![(Room::_06_C2, 1), (Room::_06_C4, 1)],
        Room::_06_C4 => vec![(Room::_06_C3, 1)],
        Room::_07 => vec![(Room::_05, 2), (Room::_06_C1, 2), (Room::_08_D1, 2), (Room::_09, 2)],
        Room::_08_D1 => vec![(Room::_08_D2, 1), (Room::_07, 2), (Room::_09, 2)],
        Room::_08_D2 => vec![(Room::_08_D1, 1), (Room::_08_D3, 1)],
        Room::_08_D3 => vec![(Room::_08_D2, 1), (Room::_08_D4, 1)],
        Room::_08_D4 => vec![(Room::_08_D3, 1)],
        Room::_09 => vec![(Room::_07, 2), (Room::_08_D1, 2), (Room::_10, 1)],
        Room::_10 => vec![(Room::_09, 1)]
    }
}

fn get_path_map() -> HashMap<Room, Vec<Path>>
{
    const ROOMS : [Room;23] =
    [
        Room::_00, Room::_01, Room::_03, Room::_05, Room::_07, Room::_09, Room::_10,
        Room::_02_A1, Room::_02_A2, Room::_02_A3, Room::_02_A4,
        Room::_04_B1, Room::_04_B2, Room::_04_B3, Room::_04_B4,
        Room::_06_C1, Room::_06_C2, Room::_06_C3, Room::_06_C4,
        Room::_08_D1, Room::_08_D2, Room::_08_D3, Room::_08_D4
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

    let route = found_path.iter().skip(1).map(|(room, dist)| *room).collect::<Vec<_>>();
    let dist = found_path.iter().map(|(room, dist)| dist).sum();
    Path { start: room1, end: room2, route, dist }
}