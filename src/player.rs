use std::time::Duration;

use crate::{NUM_ROWS, NUM_COLS, frame::{Drawable, Frame}, shot::Shot};

pub struct Player {
    // Player position
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

 impl Player {
     pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,    // Set player in the midle of x-axis
            y: NUM_ROWS - 1,    // Set player in the bottom of y-axis
            shots: Vec::new(),
        }
     }

     pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
     }

     pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
     }

    //  pub fn move_down(&mut self) {
    //     if self.y > 0 {
    //         self.y -= 1;
    //     }
    //  }

    //  pub fn move_up(&mut self) {
    //     if self.y < NUM_ROWS - 1 {
    //         self.y += 1;
    //     }
    //  }

    // Player can shoot no more than 2 ammunition
    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }

    // Update the timer of each shot
    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        // Retain the shot if it's not dead
        self.shots.retain(|shot| !shot.dead());
    }
 }

 impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        // Draw a Player
        frame[self.x][self.y] = "A";

        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
 }