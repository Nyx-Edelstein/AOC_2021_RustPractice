pub fn solution_a(input: &str) -> String
{
    get_active_cuboids(input).into_iter()
        .map(|c|
        {
            let x_min = c.x_min.max(-50);
            let x_max = c.x_max.min(50);
            let y_min = c.y_min.max(-50);
            let y_max = c.y_max.min(50);
            let z_min = c.z_min.max(-50);
            let z_max = c.z_max.min(50);
            Cuboid::new(c.state, x_min, x_max, y_min, y_max, z_min, z_max)
        })
        .filter(|c| !c.is_empty())
        .map(|c| c.volume())
        .sum::<u64>().to_string()
}

pub fn solution_b(input: &str) -> String
{
    get_active_cuboids(input).into_iter()
        .map(|c| c.volume())
        .sum::<u64>()
        .to_string()
}

fn get_active_cuboids(input: &str) -> Vec<Cuboid>
{
    let mut active_cuboids : Vec<Cuboid> = Vec::new();
    let initial_cuboids : Vec<Cuboid> = input.lines().map(Cuboid::parse).collect();
    for cuboid in initial_cuboids
    {
        match cuboid.state
        {
            true =>
            {
                let mut subregions = vec![cuboid];
                for active in active_cuboids.iter()
                {
                    subregions = subregions.into_iter()
                        .flat_map(|subregion| subregion.subtract(active))
                        .collect();
                }
                active_cuboids.extend(subregions);
            }
            false =>
            {
                active_cuboids = active_cuboids.into_iter()
                    .flat_map(|active| active.subtract(&cuboid))
                    .collect();
            }
        }
    }
    active_cuboids
}

#[derive(Debug, Clone, Copy)]
struct Cuboid
{
    state: bool,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32
}

impl Cuboid
{
    fn parse(line: &str) -> Self
    {
        let (state, rest) = line.split_once(' ').unwrap();
        let state = matches!(state, "on");
        let c = rest.split(',')
            .map(|s| s.split_once('=').unwrap().1)
            .map(|s| s.split_once("..").unwrap())
            .map(|s| (s.0.parse::<i32>().unwrap(), s.1.parse::<i32>().unwrap()))
            .collect::<Vec<_>>();
        Cuboid::new(state, c[0].0, c[0].1, c[1].0, c[1].1, c[2].0, c[2].1)
    }

    fn new(state: bool, x_min: i32, x_max: i32, y_min: i32, y_max: i32, z_min: i32, z_max: i32) -> Self
    {
        Cuboid { state, x_min, x_max, y_min, y_max, z_min, z_max }
    }

    fn is_empty(&self) -> bool
    {
        self.x_min > self.x_max || self.y_min > self.y_max || self.z_min > self.z_max
    }

    fn overlaps(&self, other: &Cuboid) -> bool {
        self.x_min <= other.x_max
            && self.x_max >= other.x_min
            && self.y_min <= other.y_max
            && self.y_max >= other.y_min
            && self.z_min <= other.z_max
            && self.z_max >= other.z_min
    }

    fn subtract(&self, other: &Cuboid) -> Vec<Cuboid>
    {
        match self.overlaps(other)
        {
            false => vec![*self],
            true =>
            {
                [
                    self.x_cut_low(other),
                    self.x_cut_high(other),
                    self.y_cut_low(other),
                    self.y_cut_high(other),
                    self.z_cut_low(other),
                    self.z_cut_high(other)
                ].into_iter().filter(|c| !c.is_empty()).collect()
            }
        }
    }

    fn x_cut_low(&self, other: &Cuboid) -> Cuboid
    {
        let state = self.state;
        let x_min = self.x_min;
        let x_max = other.x_min-1;
        let y_min = self.y_min;
        let y_max = self.y_max;
        let z_min = self.z_min;
        let z_max = self.z_max;
        Cuboid::new(state, x_min, x_max, y_min, y_max, z_min, z_max)
    }

    fn x_cut_high(&self, other: &Cuboid) -> Cuboid
    {
        let state = self.state;
        let x_min = other.x_max+1;
        let x_max = self.x_max;
        let y_min = self.y_min;
        let y_max = self.y_max;
        let z_min = self.z_min;
        let z_max = self.z_max;
        Cuboid::new(state, x_min, x_max, y_min, y_max, z_min, z_max)
    }

    fn y_cut_low(&self, other: &Cuboid) -> Cuboid
    {
        let state = self.state;
        let x_min = self.x_min.max(other.x_min);
        let x_max = self.x_max.min(other.x_max);
        let y_min = self.y_min;
        let y_max = other.y_min-1;
        let z_min = self.z_min;
        let z_max = self.z_max;
        Cuboid::new(state, x_min, x_max, y_min, y_max, z_min, z_max)
    }

    fn y_cut_high(&self, other: &Cuboid) -> Cuboid
    {
        let state = self.state;
        let x_min = self.x_min.max(other.x_min);
        let x_max = self.x_max.min(other.x_max);
        let y_min = other.y_max+1;
        let y_max = self.y_max;
        let z_min = self.z_min;
        let z_max = self.z_max;
        Cuboid::new(state, x_min, x_max, y_min, y_max, z_min, z_max)
    }

    fn z_cut_low(&self, other: &Cuboid) -> Cuboid
    {
        let state = self.state;
        let x_min = self.x_min.max(other.x_min);
        let x_max = self.x_max.min(other.x_max);
        let y_min = self.y_min.max(other.y_min);
        let y_max = self.y_max.min(other.y_max);
        let z_min = self.z_min;
        let z_max = other.z_min-1;
        Cuboid::new(state, x_min, x_max, y_min, y_max, z_min, z_max)
    }

    fn z_cut_high(&self, other: &Cuboid) -> Cuboid
    {
        let state = self.state;
        let x_min = self.x_min.max(other.x_min);
        let x_max = self.x_max.min(other.x_max);
        let y_min = self.y_min.max(other.y_min);
        let y_max = self.y_max.min(other.y_max);
        let z_min = other.z_max+1;
        let z_max = self.z_max;
        Cuboid::new(state, x_min, x_max, y_min, y_max, z_min, z_max)
    }

    fn volume(&self) -> u64
    {
        (self.x_max.abs_diff(self.x_min) + 1) as u64
            * (self.y_max.abs_diff(self.y_min) + 1) as u64
            * (self.z_max.abs_diff(self.z_min) + 1) as u64
    }
}