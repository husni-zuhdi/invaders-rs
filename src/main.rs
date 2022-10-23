use std::{error::Error, io, time::Duration, sync::mpsc, thread};
use crossterm::{terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, cursor::{Show, Hide}, ExecutableCommand, event::{self, Event, KeyCode}};
use invaders_rs::{frame::{self, new_frame}, render};
use rusty_audio::Audio;

fn main() -> Result <(), Box<dyn Error>>{
    // Initialize rusty_audio and add audio files
    let mut audio = Audio::new();
    audio.add("explode", "static/explode.wav");
    audio.add("lose", "static/lose.wav");
    audio.add("move", "static/move.wav");
    audio.add("pew", "static/pew.wav");
    audio.add("startup", "static/startup.wav");
    audio.add("win", "static/win.wav");
    audio.play("startup");

    // Initialize Terminal
    let mut stdout = io::stdout();
    terminal::is_raw_mode_enabled()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Rendering Loop in seperate threads using mpsc (for prod, use crossbeam instead)
    let (render_tx, render_rx) = mpsc::channel();
    // Handling multithread rendereing
    let render_handle = thread::spawn(move || {
        // Initialize a new frame and stdout
        let mut last_frame = frame::new_frame(); 
        let mut stdout = io::stdout();
        // Force render the first frame
        render::render(&mut stdout, &last_frame, &last_frame, true);
        // Loop to recive a new frame and render it
        loop {
            let current_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &mut last_frame, &current_frame, true);
            last_frame = current_frame;
        }
    });

    // Game Loop
    'gameloop: loop {
        // Per-frame initialization
        let current_frame = new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Draw and render Section
        // Ignore the error by naming the rendering tranciver '_'
        // Because it's expected that the gameloop will start before the mpsc channel is ready
        // to recieving any input. Maybe we can improve it? IDK man
        let _ = render_tx.send(current_frame);
        // Sleep to wait for the render loop ready
        // Configure the sleep time. Each computer is different (IDK why ???)
        thread::sleep(Duration::from_millis(20));
    }

    // Cleanup
    // Drop the channel and wait the channel to finish
    drop(render_tx);
    render_handle.join().unwrap();
    // Turnoff audio
    audio.wait();
    // Turnoff Terminal
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
