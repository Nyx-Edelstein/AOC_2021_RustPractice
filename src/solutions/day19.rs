use std::{collections::{HashSet, HashMap}, ptr::eq};

type P3 = (i32, i32, i32);
type CoordTransform = (fn(P3) -> i32, fn(P3) -> i32, fn(P3) -> i32);

pub fn solution_a(input: &str) -> String
{
    let scanner = solve(input);
    scanner.beacons.len().to_string()
}

pub fn solution_b(input: &str) -> String
{
    let scanners = solve(input).scanners;
    let max_manhattan_distance = scanners.iter().flat_map(|a|
    {
        scanners.iter().map(|b| dist_manhattan(*a, *b))
    }).max().unwrap();
    max_manhattan_distance.to_string()
}

fn solve(input: &str) -> Scanner
{
    //Algorithm: iterate over pairs of scanners, and if they align, fold one into the other
    //Eventually we are left with a single contiguous region, along with the list of all other scanner positions
    //(The first scanner is always assumed to be at the origin)
    let mut scanners = Scanner::parse(input);
    while scanners.len() > 1
    {
        let pairs = (0..scanners.len()).flat_map(|i|
        {
            ((i+1)..scanners.len()).map(move |j| (i, j))
        }).filter(|(i,j)| i != j).collect::<Vec<_>>();
        for (i, j) in pairs
        {
            let b = &scanners[j].clone();
            let a = scanners.get_mut(i).unwrap();
            if a.try_align(b)
            {
                scanners.remove(j);
                break;
            }
        };
    }
    scanners[0].clone()
}

#[derive(Clone, Debug)]
struct Scanner
{
    position: P3,
    beacons: HashSet<P3>,
    beacon_metrics: HashMap<u64, (P3, P3)>,
    scanners: Vec<P3>,
}

impl Scanner
{
    fn parse(input: &str) -> Vec<Scanner>
    {
        let mut scanners = input.split("\r\n\r\n").map(Scanner::new).collect::<Vec<_>>();
        scanners[0].position = (0,0,0);
        scanners
    }

    fn new(data: &str) -> Self
    {
        let beacons = data.lines().skip(1).map(|line|
        {
            let coords = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
            (coords[0], coords[1], coords[2])
        }).collect::<HashSet<_>>();

        //Calculate a unique and rotation/translation invariant value for every pair of beacons
        let beacon_metrics = beacons.iter().flat_map(|a|
        {
            beacons.iter().filter(|&b| !eq(a, b)).map(|b| distance_metric(*a, *b))
        }).collect::<HashMap<_, _>>();

        Scanner { position: (0,0,0), beacons, beacon_metrics, scanners: Vec::new() }
    }

    fn try_align(&mut self, other: &Scanner) -> bool
    {
        //We know that two regions match if they have 12 points in common
        //Here, we compare metrics between pairs of points, so we need (12 choose 2) = 66 matches
        //The metrics are rotation/translation invariant, so we can do this check without any expensive computation
        let matches = other.beacon_metrics.iter()
            .filter(|(m, pair)| self.beacon_metrics.contains_key(m))
            .map(|(m, pair)| *m)
            .collect::<HashSet<_>>();
        if matches.len() < 66 { return false; }

        //We now know that the regions overlap, so we just need to find the right transform to align the two scanners
        //We do this by iterating over all possible rotations and then translating them to match
        for rot in rotations()
        {
            //Figure out the offset based on a common unique metric
            //This is kind of hacky and only works because shared points always appear in the same order between scanners
            //It lets us avoid another layer of iteration though!
            //(And if we really wanted, we could create a cannonial ordering of points)
            let m = matches.iter().nth(matches.len()/2).unwrap();
            let a = self.beacon_metrics.get(m).unwrap().0;
            let b = other.beacon_metrics.get(m).unwrap().0;
            let offset = (a.0-rot.0(b), a.1-rot.1(b), a.2-rot.2(b));

            //With the rotation and offset, we can now define a linear transformation that should map the beacons in place
            let t = |p: &P3| (rot.0(*p) + offset.0, rot.1(*p) + offset.1, rot.2(*p) + offset.2);
            let transformed_beacons = other.beacons.iter().map(t).collect::<Vec<_>>();
            let matching_beacons = transformed_beacons.iter().filter(|b| self.beacons.contains(b)).count();
            if matching_beacons >= 12
            {
                let transformed_metrics = other.beacon_metrics.iter()
                    .map(|(&m,(a,b))| (m, (t(a), t(b))))
                    .collect::<HashMap<_,_>>();
                let transformed_scanners = other.scanners.iter().map(t).collect::<Vec<_>>();

                self.beacons.extend(transformed_beacons);
                self.beacon_metrics.extend(transformed_metrics);
                self.scanners.extend(transformed_scanners);
                self.scanners.push(offset);
                return true;
            }
        }
        false
    }
}

fn distance_metric(a: P3, b: P3) -> (u64, (P3, P3))
{
    let metric = ((dist_euclidean(a, b) * 10000000.0) + dist_manhattan(a, b) as f64).round() as u64;
    (metric, (a, b))
}

fn dist_euclidean(a: P3, b: P3) -> f64
{
    let x = (a.0-b.0).pow(2) as f64;
    let y = (a.1-b.1).pow(2) as f64;
    let z = (a.2-b.2).pow(2) as f64;
    (x + y + z).sqrt()
}

fn dist_manhattan(a: P3, b: P3) -> i32
{
    let x = (a.0-b.0).abs();
    let y = (a.1-b.1).abs();
    let z = (a.2-b.2).abs();
    x + y + z
}

fn rotations() -> Vec<CoordTransform>
{
    const ROTATIONS : [CoordTransform; 24] =
    [
        (xpos, ypos, zpos), (xpos, zneg, ypos), (xpos, yneg, zneg), (xpos, zpos, yneg),
        (xneg, ypos, zneg), (xneg, zpos, ypos), (xneg, yneg, zpos), (xneg, zneg, yneg),
        (ypos, xpos, zneg), (ypos, zpos, xpos), (ypos, xneg, zpos), (ypos, zneg, xneg),
        (yneg, xpos, zpos), (yneg, zneg, xpos), (yneg, xneg, zneg), (yneg, zpos, xneg),
        (zpos, xpos, ypos), (zpos, yneg, xpos), (zpos, xneg, yneg), (zpos, ypos, xneg),
        (zneg, xpos, yneg), (zneg, ypos, xpos), (zneg, xneg, ypos), (zneg, yneg, xneg),
    ];
    ROTATIONS.into_iter().collect()
}
fn xpos(coord: P3) -> i32 { coord.0 }
fn xneg(coord: P3) -> i32 { -coord.0 }
fn ypos(coord: P3) -> i32 { coord.1 }
fn yneg(coord: P3) -> i32 { -coord.1 }
fn zpos(coord: P3) -> i32 { coord.2 }
fn zneg(coord: P3) -> i32 { -coord.2 }