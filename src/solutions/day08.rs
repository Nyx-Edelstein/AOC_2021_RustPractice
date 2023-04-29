use std::{collections::{HashSet, HashMap}};

pub fn solution_a(input: &str) -> String
{
    let count_1_4_7_8 = input.lines()
        .map(|s| s.split(" | ").nth(1).unwrap())
        .flat_map(|s| s.split(' '))
        .map(|s| match s.len()
        {
            2 => 1,
            3 => 1,
            4 => 1,
            7 => 1,
            _ => 0
        }).sum::<u32>();

    count_1_4_7_8.to_string()
}

pub fn solution_b(input: &str) -> String
{
    let display_sum: u32 = input.lines()
        .map(Display::new)
        .map(|d| d.solve())
        .sum();

    display_sum.to_string()
}

const DISPLAY_ZERO: &str = "abcefg";
const DISPLAY_ONE: &str = "cf";
const DISPLAY_TWO: &str = "acdeg";
const DISPLAY_THREE: &str = "acdfg";
const DISPLAY_FOUR: &str = "bcdf";
const DISPLAY_FIVE: &str = "abdfg";
const DISPLAY_SIX: &str = "abdefg";
const DISPLAY_SEVEN: &str = "acf";
const DISPLAY_EIGHT: &str = "abcdefg";
const DISPLAY_NINE: &str = "abcdfg";

#[derive(Debug, Clone)]
struct Display
{
    mixed_clusters: Vec<String>,
    display_clusters: Vec<String>,
    segment_maps: HashMap<char, HashSet<char>>
}

impl Display
{
    fn new(display_str: &str) -> Self
    {
        let parts: Vec<_> = display_str.split(" | ")
            .filter(|&s| !s.is_empty())
            .collect();

        let mut mixed_clusters = Self::parse(parts[0]);
        mixed_clusters.sort_by_key(|s| std::cmp::Reverse(s.len()));
        let display_clusters = Self::parse(parts[1]);

        let all_segments: HashSet<char> = "abcdefg".chars().collect();
        let mut segment_maps = HashMap::new();
        segment_maps.insert('a', all_segments.clone());
        segment_maps.insert('b', all_segments.clone());
        segment_maps.insert('c', all_segments.clone());
        segment_maps.insert('d', all_segments.clone());
        segment_maps.insert('e', all_segments.clone());
        segment_maps.insert('f', all_segments.clone());
        segment_maps.insert('g', all_segments);

        Display { mixed_clusters, display_clusters, segment_maps }
    }

    fn parse(part: &str) -> Vec<String>
    {
        part.split(' ').map(|s|
        {
            let mut v : Vec<_> = s.chars().collect();
            v.sort();
            v.into_iter().collect::<String>()
        }).collect()
    }

    fn solve(self) -> u32
    {
        let mut to_consider = vec![self; 1];
        while let Some(current) = to_consider.pop()
        {
            let next = current.expand();
            for node in next.into_iter()
            {
                if node.is_solved()
                {
                    return node.map_display_clusters();
                }
                else if node.is_valid()
                {
                    to_consider.push(node);
                }                
            }
            to_consider.sort_by(|a, b| b.mixed_clusters.cmp(&a.mixed_clusters))  
        }
        unreachable!("did not find a solution (should not happen!)");
    }

    fn expand(mut self) -> Vec<Display>
    {
        let mixed_cluster = self.mixed_clusters.pop().unwrap_or_default();
        let segments : &[&str] = match mixed_cluster.len()
        {
            0 => &[], //empty
            2 => &[DISPLAY_ONE],
            3 => &[DISPLAY_SEVEN],
            4 => &[DISPLAY_FOUR],
            5 => &[DISPLAY_TWO, DISPLAY_THREE, DISPLAY_FIVE],
            6 => &[DISPLAY_ZERO, DISPLAY_SIX, DISPLAY_NINE],
            7 => &[], //no information since all segments are on
            _ => unreachable!("invalid input somehow"),
        };

        segments.iter().map(|&orig_segs|
        {
            self.update_segment_maps(&mixed_cluster, orig_segs)
        }).collect()
    }

    fn update_segment_maps(&self, mixed_cluster: &str, orig_segs: &str) -> Display
    {
        let mut node = self.clone();

        for c1 in mixed_cluster.chars()
        {
            let segment = node.segment_maps.get_mut(&c1).unwrap();
            segment.retain(|&c| orig_segs.contains(c));
        }

        let complement : Vec<_> = "abcdefg".chars().filter(|&c| !mixed_cluster.contains(c)).collect();
        for c2 in complement
        {
            let segment = node.segment_maps.get_mut(&c2).unwrap();
            segment.retain(|&c| !orig_segs.contains(c));
        }
        
        node
    }

    fn map_display_clusters(&self) -> u32
    {
        let a = self.map(&self.display_clusters[0]);
        let b = self.map(&self.display_clusters[1]);
        let c = self.map(&self.display_clusters[2]);
        let d = self.map(&self.display_clusters[3]);

        a*1000 + b*100 + c*10 + d
    }

    fn map(&self, cluster: &str) -> u32
    {
        let mut display_chars : Vec<_> = cluster.chars()
            .map(|c| *self.segment_maps.get(&c).unwrap().iter().next().unwrap())
            .collect();
        display_chars.sort();

        let display_str = display_chars.into_iter().collect::<String>();
        match display_str.as_str()
        {
            DISPLAY_ZERO => 0,
            DISPLAY_ONE => 1,
            DISPLAY_TWO => 2,
            DISPLAY_THREE => 3,
            DISPLAY_FOUR => 4,
            DISPLAY_FIVE => 5,
            DISPLAY_SIX => 6,
            DISPLAY_SEVEN => 7,
            DISPLAY_EIGHT => 8,
            DISPLAY_NINE => 9,
            _ => 0
        }
    }

    fn is_solved(&self) -> bool
    {
        self.segment_maps.iter().all(|s| s.1.len() == 1)
    }

    fn is_valid(&self) -> bool
    {
        self.segment_maps.iter().all(|s| !s.1.is_empty())
    }
}