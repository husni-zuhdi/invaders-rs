use std::io::{Stdout, Write};
use crate::frame::Frame;
use crossterm::{QueueableCommand, style::{SetBackgroundColor, Color}, terminal::{Clear, ClearType}, cursor::{MoveTo}};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, current_frame: &Frame, force: bool) {
    // Check is force render?
    if force {
        // Set background color to blue
        stdout.queue(SetBackgroundColor(Color::Red)).unwrap();
        // Clear all stdout
        stdout.queue(Clear(ClearType::All)).unwrap();
        // Then reset background color to black
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x, coll) in current_frame.iter().enumerate() {
        for (y, s) in coll.iter().enumerate() {
            // Check if the s str is not same as the last frame or force is true
            if *s != last_frame[x][y] || force {
                // Move cursor to x, y
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                // Print the dereference s str to the x,y
                print!("{}", *s);
            }
        }
    }

    // Flush all our queues
    stdout.flush().unwrap();
}
