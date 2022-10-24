use crate::{NUM_ROWS, NUM_COLS, frame::{Drawable, Frame}};

pub struct Player {
    // Player position
    x: usize,
    y: usize,
}

 impl Player {
     pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,    // Set player in the midle of x-axis
            y: NUM_ROWS - 1     // Set player in the bottom of y-axis
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
 }

 impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        // Draw a Player
        frame[self.x][self.y] = "A";
    }
 }