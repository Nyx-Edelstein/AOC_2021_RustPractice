use std::collections::HashSet;

enum FoldType
{
    X,
    Y
}

struct Fold
{
    fold_type: FoldType,
    value: u32
}

struct Foldable
{
    rows : u32,
    cols : u32,
    points: HashSet<(u32, u32)>
}

impl Foldable
{
    fn new() -> Self
    {
        Foldable { rows: 0, cols: 0, points: HashSet::new() }
    }

    fn fold_along(self, fold: &Fold) -> Self
    {
        match fold.fold_type
        {
            FoldType::X => self.fold_along_x(fold.value),
            FoldType::Y => self.fold_along_y(fold.value)            
        }
    }

    fn fold_along_x(mut self, value: u32) -> Self
    {
        let mut new_points : HashSet<(u32, u32)> = HashSet::new();
        for (x, y) in self.points
        {
            if x < value
            { 
                new_points.insert((x,y));
            }
            else
            {
                let new_x = 2*value - x;
                new_points.insert((new_x, y));
            }
        }
        self.cols = value - 1;
        self.points = new_points;
        self
    }

    fn fold_along_y(mut self, value: u32) -> Self
    {
        let mut new_points : HashSet<(u32, u32)> = HashSet::new();
        for (x, y) in self.points
        {
            if y < value
            { 
                new_points.insert((x,y));
            }
            else
            {
                let new_y = 2*value - y;
                new_points.insert((x, new_y));
            }
        }
        self.rows = value - 1;
        self.points = new_points;
        self
    }

    fn stringify(&self) -> String
    {
        let mut s = String::new();
        for row in 0..=self.rows
        {
            for col in 0..=self.cols
            {
                s += match self.points.contains(&(col, row))
                {
                    true => "#",
                    false => "."
                }
            }
            s += "\r\n";
        }

        s
    }
}

fn parse(input: &str) -> (Foldable, Vec<Fold>)
{
    let parts = input.split_once("\r\n\r\n").unwrap();

    let foldable = parts.0.lines().fold(Foldable::new(), |mut foldable, s|
    {
        let (x_str, y_str) = s.split_once(',').unwrap();
        let x = x_str.parse::<u32>().unwrap();
        let y = y_str.parse::<u32>().unwrap();

        if x > foldable.cols { foldable.cols = x; }
        if y > foldable.rows { foldable.rows = y; }
        foldable.points.insert((x,y));
        
        foldable
    });

    let folds = parts.1.lines().map(|s|
    {
        let fold_str = s.split_terminator("fold along ").last().unwrap();
        let fold_data = fold_str.split_once('=').unwrap();
        let fold_type = match fold_data.0
        {
            "x" => FoldType::X,
            "y" => FoldType::Y,
            _ => unreachable!("invalid fold input somehow")
        };
        let value = fold_data.1.parse::<u32>().unwrap();

        Fold{fold_type, value}
    }).collect::<Vec<_>>();

    (foldable, folds)
}

pub fn solution_a(input: &str) -> String
{
    let (mut foldable, folds) = parse(input);
    foldable = foldable.fold_along(&folds[0]);
    foldable.points.len().to_string()
}

pub fn solution_b(input: &str) -> String
{
    let (mut foldable, folds) = parse(input);
    for fold in folds
    {
        foldable = foldable.fold_along(&fold);
    }    
    foldable.stringify()
}
