use crate::parser;

pub struct Window {
    width: isize,
    height: isize,
    pixels: Vec<char>,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: isize,
    y: isize
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Square(isize, isize),
    Circle(isize),
}

impl Window {

    pub fn resize(&mut self, width: isize, height: isize) {
        let mut new_pixels = vec![' '; (width * height) as usize];
        'y: for y in 0..self.height {
            if y >= height {
                break;
            }
            for x in 0..self.width {
                if x >= width {
                    continue 'y;
                }
                new_pixels[(y * width + x) as usize] = self.pixels[(y * self.width + x) as usize];
            }
        }
        self.width = width;
        self.height = height;
        self.pixels = new_pixels;
    }

    pub fn new(width: isize, height: isize) -> Self {
        Self {
            width,
            height,
            pixels: vec![' '; (width * height) as usize],
        }
    }

    pub fn comm_new(&mut self, width: isize, height: isize) {
            self.width = width;
            self.height = height;
            self.pixels = vec![' '; (width * height) as usize];
    }

    pub fn fill(&mut self, chr: char) {
        self.pixels = vec![chr; (self.width * self.height) as usize];
    }

    pub fn replace(&mut self, old_chr: char, new_char: char) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.pixels[(y * self.width + x) as usize] == old_chr {
                    self.pixels[(y * self.width + x) as usize] = new_char;
                }
            }
        }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!{"{}",self.pixels[(x + y*self.width) as usize]};
            }
            println!();
        }
    }

    pub fn draw(&mut self, point: Point, shape: Shape, chr:char) {
        match shape {
            Shape::Circle(radius) => {
                let x = point.x;
                let y = point.y;
                let x1 = if x < radius { 0 } else { x - radius};
                let x2 = if x + radius > self.width - 1 { self.width - 1 } else { x + radius };
                let y1 = if y < radius { 0 } else { y - radius};
                let y2 = if y + radius > self.height - 1 { self.height - 1 } else { y + radius };

                for cur_y in y1..=y2 {
                    for cur_x in x1..=x2 {
                        let dist = f64::sqrt((isize::pow(cur_y - y, 2) + isize::pow(cur_x - x,2 ))as f64);
                        if dist <= radius as f64 {
                            self.pixels[(cur_y * self.width + cur_x) as usize] = chr;
                        }
                    }
                }
            },
            Shape::Square(length, height) => {
                let x = point.x;
                let y = point.y;
                let x1 = if x < length { 0 } else { x - length };
                let x2 = if x + length > self.width - 1 { self.width - 1 } else { x + length };
                let y1 = if y < height { 0 } else { y - height};
                let y2 = if y + height > self.height - 1 { self.height - 1 } else { y + height };
                for cur_x in x1..=x2 {
                    for cur_y in y1..=y2 {
                        self.pixels[(cur_y * self.width + cur_x) as usize] = chr;
                    }
                }

            },

        }

    }
}