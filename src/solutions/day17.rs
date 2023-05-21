use itertools::Itertools;

pub fn solution_a(input: &str) -> String
{
    let target_area = TargetArea::new(input);
    let highest_y = ((target_area.y_min+1)*target_area.y_min)/2;
    highest_y.to_string()
}

pub fn solution_b(input: &str) -> String
{
    let target_area = TargetArea::new(input);
    (1..=target_area.x_max).cartesian_product(target_area.y_min..=-target_area.y_min)
        .filter(|(x, y)| target_area.check_solution(*x, *y))
        .count()
        .to_string()
}

#[derive(Debug)]
struct TargetArea
{
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32
}

impl TargetArea
{
    fn new(input: &str) -> Self
    {
        let data = input.trim_start_matches("target area: x=").split_once(", y=").unwrap();
        let x_data = data.0.split_once("..").unwrap();
        let y_data = data.1.split_once("..").unwrap();
        let x_min = x_data.0.parse::<i32>().unwrap();
        let x_max = x_data.1.parse::<i32>().unwrap();
        let y_min = y_data.0.parse::<i32>().unwrap();
        let y_max = y_data.1.parse::<i32>().unwrap();
        TargetArea { x_min, x_max, y_min, y_max }
    }

    fn check_solution(&self, mut x_vel: i32, mut y_vel: i32) -> bool
    {
        let mut x = 0;
        let mut y = 0;
        while self.position_is_valid(x, y)
        {
            if self.is_in_target_area(x, y) { return true; }
            x += x_vel;
            y += y_vel;
            if x_vel > 0 { x_vel -= 1 }
            y_vel -= 1;            
        }
        false
    }

    fn position_is_valid(&self, x: i32, y: i32) -> bool
    {
        x <= self.x_max && y >= self.y_min
    }

    fn is_in_target_area(&self, x: i32, y: i32) -> bool
    {
        self.x_min <= x && x <= self.x_max && self.y_min <= y && y <= self.y_max
    }
}