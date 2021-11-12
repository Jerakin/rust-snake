use crate::{NUM_COLS, NUM_ROWS};
use crate::frame::{Drawable, Frame};
use crate::fruit::Fruit;


pub struct Part {
    pub x: usize,
    pub y: usize,
    active: bool,
    turns: usize,
}

pub struct Player {
    direction_x: isize,
    direction_y: isize,
    pub parts: Vec<Part>,
}


impl Player {
    pub fn new() -> Self {
        Self {
            direction_x: 0,
            direction_y: 0,
            parts: vec![Part{x:NUM_COLS / 2, y:NUM_ROWS / 2, active:true, turns:0}],
        }
    }

    pub fn move_left(&mut self) {
        if self.direction_x != 1 {
            self.direction_x = -1;
            self.direction_y = 0;
        }
    }

    pub fn move_right(&mut self) {
        if self.direction_x != -1 {
            self.direction_x = 1;
            self.direction_y = 0;
        }
    }

    pub fn move_up(&mut self) {
        if self.direction_y != 1 {
            self.direction_x = 0;
            self.direction_y = -1;
        }
    }

    pub fn move_down(&mut self) {
        if self.direction_y != -1 {
            self.direction_x = 0;
            self.direction_y = 1;
        }
    }

    pub fn consume(&mut self, fruit: &mut Fruit) {
        if fruit.x == self.parts[0].x && fruit.y == self.parts[0].y {
            self.parts.push(Part{x:self.parts[0].x, y:self.parts[0].y, active:false, turns:self.parts.len()});
            fruit.new_rand_pos(&self.parts);
        }
    }

    pub fn update(&mut self) {
        let copy_of_parts = self.parts.clone();
        for (index, part) in self.parts.iter_mut().enumerate().rev() {
            if part.turns > 0 {
                part.turns -= 1;
            }
            else {
                if index == 0 {
                    if part.x as isize + self.direction_x == -1 {
                        part.x = NUM_COLS-1;
                    } else {
                        part.x = (part.x as isize + self.direction_x) as usize % NUM_COLS;
                    }

                    if part.y as isize + self.direction_x == -1 {
                        part.y = NUM_ROWS -1;
                    } else {
                        part.y = (part.y as isize + self.direction_y) as usize % NUM_ROWS;
                    }

                } else {
                    let current_part = &copy_of_parts[index - 1];
                    part.x = current_part.x;
                    part.y = current_part.y;
                }
                if part.turns == 0 {
                    part.active = true;
                }
            }
        }
    }

    // The player hit himself
    pub fn detect_hit(&self) -> bool {
        if self.parts.len() == 1 {
            return false;
        }
        for (index, part) in self.parts.iter().enumerate() {
            if index == 0 {
                continue
            }
            if part.active && part.x == self.parts[0].x && part.y == self.parts[0].y {
                return true;
            }
        }
        return false;
    }
}

impl Clone for Part {
    fn clone(&self) -> Self {
        return Part{x: self.x, y: self.y, active: self.active, turns: self.turns};
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        for (index, part) in self.parts.iter().enumerate() {
            if index == 0 {
                frame[part.x][part.y] = "□";
            } else {
                frame[part.x][part.y] = "■";
            }
        }
    }
}
