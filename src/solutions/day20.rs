pub fn solution_a(input: &str) -> String
{
    solve(input, 2)
}

pub fn solution_b(input: &str) -> String
{
    solve(input, 50)
}

pub fn solve(input: &str, steps: usize) -> String
{
    let mut enh = ImageEnhancement::parse(input);
    for i in 0..steps { enh.step(); }
    let num_pixels = enh.current_image.iter().map(|row|
    {
        row.iter().filter(|pixel| **pixel).count()
    }).sum::<usize>();
    num_pixels.to_string()
}

struct ImageEnhancement
{
    reference_pixels: Vec<bool>,
    outside_pixel: bool,
    current_image: Vec<Vec<bool>>
}

impl ImageEnhancement
{
    fn parse(input: &str) -> Self
    {
        let (pixel_str, img_str) = input.split_once("\r\n\r\n").unwrap();
        let reference_pixels = pixel_str.chars().map(|c| c == '#').collect::<Vec<_>>();
        let outside_pixel = false;
        let mut current_image = vec![Vec::new()];
        img_str.lines().enumerate().for_each(|(row, line)|
        {
            line.chars().enumerate().for_each(|(col, c)|
            {
                current_image[row].push(matches!(c, '#'));
            });
            current_image.push(Vec::new());
        });
        ImageEnhancement { reference_pixels, outside_pixel, current_image }
    }

    fn step(&mut self)
    {
        //Extend the image one in every direction
        for row in 0..self.current_image.len()
        {
            self.current_image[row].push(self.outside_pixel);
            self.current_image[row].insert(0, self.outside_pixel);
        }
        let empty = vec![self.outside_pixel; self.current_image[0].len()];
        self.current_image.insert(0, empty.clone());
        self.current_image.insert(self.current_image.len()-1, empty);
        
        //Calculate new pixels
        self.current_image = (0..self.current_image.len()).map(|row|
        {
            (0..self.current_image[0].len()).map(|col|
            {
                self.next_pixel(row as i32, col as i32)
            }).collect()
        }).collect();
        if self.reference_pixels[0] { self.outside_pixel = !self.outside_pixel };
    }

    fn next_pixel(&self, row: i32, col: i32) -> bool
    {
        let mut index = 0;
        [(1, 1), (1, 0), (1, -1), (0, 1), (0, 0), (0, -1), (-1, 1), (-1, 0), (-1,-1)].iter().enumerate()
        .for_each(|(pos, (i, j))|
        {
            if self.get_pixel(row+i, col+j) { index += 2usize.pow(pos as u32) }
        });
        self.reference_pixels[index]
    }

    fn get_pixel(&self, row: i32, col: i32) -> bool
    {
        let Ok(row) = usize::try_from(row) else { return self.outside_pixel; };
        let Ok(col) = usize::try_from(col) else { return self.outside_pixel; };
        *self.current_image.get(row).and_then(|row| row.get(col)).unwrap_or(&self.outside_pixel)
    }
}