use std::collections::{HashMap};

pub fn solution_a(input: &str) -> String
{
    solve(input, false)
}

pub fn solution_b(input: &str) -> String
{
    solve(input, true)
}

fn solve(input: &str, can_revisit: bool) -> String
{
    let connections = parse(input);
    let mut valid_paths = 0;
    let mut to_consider = vec![Path::new(can_revisit)];
    while let Some(path) = to_consider.pop()
    {
        let Some(next) = connections.get(path.prev()) else { continue };
        for &node in next
        {
            //Don't backtrack to the start node
            if node == "start"
            {
                continue;
            }

            //If we reach the end, we're done
            if node == "end"
            {
                valid_paths += 1;
                continue;
            }

            //Do not revisit small caves (unless in part b, then we can revisit exactly one cave twice)
            let node_is_small_cave = node.chars().all(|c| c.is_lowercase());
            let is_revisit = node_is_small_cave && path.contains(node);
            if is_revisit && !path.can_revisit
            {
                continue;
            }

            let new_path = path.append(node.to_owned(), is_revisit);
            to_consider.push(new_path);
        }
    }

    valid_paths.to_string()
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>>
{
    input.lines().map(|line: &str| line.split_once('-').unwrap())
        .fold(HashMap::new(), |mut map, (node_a, node_b)|
        {
            if map.contains_key(node_a)
            {
                map.get_mut(node_a).unwrap().push(node_b);
            }            
            else
            {
                map.insert(node_a, vec![node_b]);
            }

            if map.contains_key(node_b)
            {
                map.get_mut(node_b).unwrap().push(node_a);
            }            
            else
            {
                map.insert(node_b, vec![node_a]);
            }
            
            map
        })
}

#[derive(Clone)]
struct Path
{
    data: Vec<String>,
    can_revisit : bool
}

impl Path
{
    fn new(can_revisit: bool) -> Self
    {
        let start_node = String::from("start");
        let data = vec![start_node];

        Path { data, can_revisit }
    }

    fn contains(&self, node: &str) -> bool
    {
        self.data.iter().any(|s| *s == *node)
    }

    fn prev(&self) -> &str
    {
        self.data.last().unwrap().as_str()
    }

    fn append(&self, node: String, is_revisit: bool) -> Path
    {
        let mut new_path = self.clone();
        new_path.data.push(node);
        new_path.can_revisit &= !is_revisit;

        new_path
    }
}


