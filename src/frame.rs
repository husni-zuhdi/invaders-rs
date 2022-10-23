use crate::{NUM_COLS, NUM_ROWS};

// Create Frame Vector of Vector of borrowed static string
pub type Frame = Vec<Vec<& 'static str>>;

// Create a new Frame by generate the row first then the collumns
pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            col.push(" ");
        }
        cols.push(col);
    }
    cols
}

// Drawable trait
pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}