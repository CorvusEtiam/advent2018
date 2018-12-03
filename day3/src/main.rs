extern crate regex;

use regex::{Captures, Regex};

#[derive(Debug)]
struct Rect {
    x : usize,
    y : usize,
    w : usize,
    h : usize,
    id: usize 
}

impl Rect {
    fn from_line(capture: Captures) -> Self {
        let id : usize = capture.get(1).unwrap().as_str().parse().unwrap();
        let x : usize= capture.get(2).unwrap().as_str().parse().unwrap();
        let y : usize= capture.get(3).unwrap().as_str().parse().unwrap();
        let w : usize= capture.get(4).unwrap().as_str().parse().unwrap();
        let h : usize= capture.get(5).unwrap().as_str().parse().unwrap();
        Rect { id: id, x: x, y: y, w: w, h: h }
    }

    fn draw(&self, grid: &mut Vec<usize>) {
        for y in self.y..self.y+self.h {
            for x in self.x..self.x+self.w {
                grid[x + (y * 1000)] += 1;
            }
        }
    }

    fn check(&self, grid: &Vec<usize>) -> bool {
        for y in self.y..self.y+self.h {
            for x in self.x..self.x+self.w {
                if grid[x + (y * 1000)] != 1 { return false; }
            }
        }

        return true;
    }
}



fn main() {
    let re = Regex::new(r"#(\d+)\W+(\d+),(\d+):\W+(\d+)x(\d+)\n").unwrap();
    let data = include_str!("/home/misiek/Project/rust/advent2018/inputs/input3.txt");
    let mut rects : Vec<Rect> = Vec::new();
    let mut grid : Vec<usize> = vec![0; 1000 * 1000];
    for capture in re.captures_iter(data) {
        rects.push(Rect::from_line(capture));
    }

    for rect in rects.iter() {
        rect.draw(&mut grid);
    }
    let mut result_1 : usize = 0;
    let mut result_2 : usize = 0;
    for i in grid.iter() {
        if *i >= 2 {
            result_1 += 1;
        }
    }


    for rect in rects.iter() {
        if rect.check(&grid) {  
            result_2 = rect.id as usize;
        }
    }

    println!("Solution 1: {}", result_1);
    println!("Solution 2: {}", result_2);
}
