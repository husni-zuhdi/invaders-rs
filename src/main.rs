use std::error::Error;
use rusty_audio::Audio;

fn main() -> Result <(), Box<dyn Error>>{
    let mut audio = Audio::new();
    audio.add("explode", "static/explode.wav");
    audio.add("lose", "static/lose.wav");
    audio.add("move", "static/move.wav");
    audio.add("pew", "static/pew.wav");
    audio.add("startup", "static/startup.wav");
    audio.add("win", "static/win.wav");
    audio.play("startup");

    // Cleanup
    audio.wait();
    Ok(())
}
