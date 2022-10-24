use std::{time::Duration, cmp::max};

use rusty_time::timer::Timer;

use crate::{NUM_COLS, NUM_ROWS, frame::Drawable};

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    // Initate new army of invaders
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if (x > 1)                  // Army not too close to left of the screen
                    && (x < NUM_COLS - 2)   // Army not too close to right of the screen
                    && (y > 0)              // Army start from top of screen
                    && (y < 9)              // Limit Army size of 4/5 rows (becasue 9/2)
                    && (x % 2 == 0)         // Give space to each invaders in x-axis
                    && (y % 2 == 0) {       // Give space to each invaders in y-axis
                        army.push(Invader {x, y});
                    }
            }
        }
        Self { army: army, move_timer: Timer::from_millis(2000), direction: 1 }
    }

    // Update our army of invaders
    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            // Set movement downward to false
            let mut downwards = false;
            // direction go to left
            if self.direction == -1 {
                // Find the leftest x value of invaders
                let min_x = self.army.iter().map(| invader | invader.x).min().unwrap_or(0);
                // This mean that the leftest is hitting the left screen
                // We need to change the direction and move it downward
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            // direction go to right
            } else {
                // Same logic as above
                let max_x = self.army.iter().map(| invader | invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }
            if downwards {
                // Increase the Invaders speed each goes downward
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                // Move invaders downwards
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                // Move invaders 1 block in x-axis based on the direction
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            return true;
        }
        return false;
    }
}

// impl Drawable for Invader {
// }

impl Drawable for Invaders {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if (self.move_timer.time_left.as_secs_f64()
                / self.move_timer.duration.as_secs_f64()) > 0.5 {
                    "x"
            } else {
                "+"
            };
        }
    }
}