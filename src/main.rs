use std::error::Error;
use std::sync::mpsc::channel;
use std::{io, thread};
use std::time::{Duration, Instant};

use crossterm::{terminal, ExecutableCommand, event};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use snake::frame::Drawable;
use snake::player::Player;
use snake::fruit::Fruit;
use snake::{frame, render};

fn main() -> Result <(), Box<dyn Error>> {
    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop
    let (render_tx, render_rx) = channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut player = Player::new();
    let mut instant = Instant::now();

    let mut fruit = Fruit::new();
    fruit.new_rand_pos(&player.parts);

    'gameloop: loop {
        //Per-frame init
        let _delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = frame::new_frame();

        //Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Right | KeyCode::Char('d') => player.move_right(),
                    KeyCode::Left | KeyCode::Char('a') => player.move_left(),
                    KeyCode::Up | KeyCode::Char('w') => player.move_up(),
                    KeyCode::Down | KeyCode::Char('s') => player.move_down(),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update();
        player.consume(&mut fruit);
        if player.detect_hit() {
            break 'gameloop;
        }

        // Draw & Render
        let drawables: Vec<&dyn Drawable> = vec![&player, &fruit];
        for drawable in drawables {
            drawable.draw(&mut curr_frame)
        }

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(250));
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}