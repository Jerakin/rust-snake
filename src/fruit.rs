use crate::player::Part;
use crate::frame::{Drawable, Frame};
use crate::{NUM_COLS, NUM_ROWS};
use rand::Rng;

pub struct Fruit {
    pub x: usize,
    pub y: usize,
}

impl Fruit {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }

    pub fn new_rand_pos(&mut self, parts: &Vec<Part>) {
        let mut rng = rand::thread_rng();
        let mut is_ok = true;
        loop {
            self.x = rng.gen_range(4..NUM_COLS-5);
            self.y = rng.gen_range(4..NUM_ROWS-5);
            for part in parts.iter() {
                if part.x == self.x && part.y == self.y {
                    is_ok = false;
                }
            }
            if is_ok {
                break
            }
        }
    }
}

impl Drawable for Fruit {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "X";
    }
}