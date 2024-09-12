use std::io::{stdout, Write};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand, Result,
};
use rand::Rng;
use std::{thread, time::Duration};

fn main() -> Result<()> {
    // Initialize the terminal
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(Hide)?;
    stdout.execute(Clear(ClearType::All))?;

    const FIRE_WIDTH: usize = 80;
    const FIRE_HEIGHT: usize = 40;

    // Fire color palette
    let palette = [
        Color::Rgb { r: 7, g: 7, b: 7 },
        Color::Rgb { r: 31, g: 7, b: 7 },
        Color::Rgb { r: 47, g: 15, b: 7 },
        Color::Rgb { r: 71, g: 15, b: 7 },
        Color::Rgb { r: 87, g: 23, b: 7 },
        Color::Rgb { r: 103, g: 31, b: 7 },
        Color::Rgb { r: 119, g: 31, b: 7 },
        Color::Rgb { r: 143, g: 39, b: 7 },
        Color::Rgb { r: 159, g: 47, b: 7 },
        Color::Rgb { r: 175, g: 63, b: 7 },
        Color::Rgb { r: 191, g: 71, b: 7 },
        Color::Rgb { r: 199, g: 71, b: 7 },
        Color::Rgb { r: 223, g: 79, b: 7 },
        Color::Rgb { r: 223, g: 87, b: 7 },
        Color::Rgb { r: 223, g: 87, b: 7 },
        Color::Rgb { r: 215, g: 95, b: 7 },
        Color::Rgb { r: 215, g: 95, b: 7 },
        Color::Rgb { r: 215, g: 103, b: 15 },
        Color::Rgb { r: 207, g: 111, b: 15 },
        Color::Rgb { r: 207, g: 119, b: 15 },
        Color::Rgb { r: 207, g: 127, b: 15 },
        Color::Rgb { r: 207, g: 135, b: 23 },
        Color::Rgb { r: 199, g: 135, b: 23 },
        Color::Rgb { r: 199, g: 143, b: 23 },
        Color::Rgb { r: 199, g: 151, b: 31 },
        Color::Rgb { r: 191, g: 159, b: 31 },
        Color::Rgb { r: 191, g: 159, b: 31 },
        Color::Rgb { r: 191, g: 167, b: 39 },
        Color::Rgb { r: 191, g: 167, b: 39 },
        Color::Rgb { r: 191, g: 175, b: 47 },
        Color::Rgb { r: 183, g: 175, b: 47 },
        Color::Rgb { r: 183, g: 183, b: 47 },
        Color::Rgb { r: 183, g: 183, b: 55 },
        Color::Rgb { r: 207, g: 207, b: 111 },
        Color::Rgb { r: 223, g: 223, b: 159 },
        Color::Rgb { r: 239, g: 239, b: 199 },
        Color::Rgb { r: 255, g: 255, b: 255 },
    ];

    let mut fire_pixels = vec![0u8; FIRE_WIDTH * FIRE_HEIGHT];

    // Initialize the bottom row of the fire
    for x in 0..FIRE_WIDTH {
        fire_pixels[(FIRE_HEIGHT - 1) * FIRE_WIDTH + x] = (palette.len() - 1) as u8;
    }

    loop {
        // Update fire pixels based on the algorithm
        for x in 0..FIRE_WIDTH {
            for y in 1..FIRE_HEIGHT {
                let src = y * FIRE_WIDTH + x;
                let mut rng = rand::thread_rng();
                let rand_idx = rng.gen_range(0..3);
                let dst = src - rand_idx + 1;
                let intensity = fire_pixels[src] as i16 - rand_idx as i16;
                fire_pixels[src - FIRE_WIDTH] = if intensity < 0 { 0 } else { intensity as u8 };
            }
        }

        // Render the fire pixels to the terminal
        for y in 0..FIRE_HEIGHT {
            for x in 0..FIRE_WIDTH {
                let intensity = fire_pixels[y * FIRE_WIDTH + x] as usize;
                let color = palette[intensity];
                stdout.execute(MoveTo(x as u16, y as u16))?;
                stdout.execute(SetForegroundColor(color))?;
                print!("█");
            }
        }

        stdout.execute(ResetColor)?;
        stdout.flush()?;

        // Check for user input to exit
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Char('q') || key_event.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }

    // Restore terminal state
    stdout.execute(ResetColor)?;
    stdout.execute(Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
