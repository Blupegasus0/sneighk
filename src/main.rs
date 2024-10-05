use crossterm::{terminal, execute, event, style, cursor, ExecutableCommand};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

use std::time::{Duration, Instant};

use sneighk::{Point, Game, Snake, Food};

fn main() {
    let mut game = Game::new();
    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();

    enable_raw_mode().unwrap();

    // Run loop 
    loop {
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let event::Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    event::KeyCode::Up => game.snake.direction = Point { x: 0, y: -1 },
                    event::KeyCode::Down => game.snake.direction = Point { x: 0, y: 1 },
                    event::KeyCode::Left => game.snake.direction = Point { x: -1, y: 0 },
                    event::KeyCode::Right => game.snake.direction = Point { x: 1, y: 0 },
                    event::KeyCode::Esc => break, // exit game
                    _ => {},
                }
            }
        }

        // Update game state
        if last_tick.elapsed() >= tick_rate {
            game.update();
            game.draw();
            last_tick = Instant::now();
        }

        // Check if game is over
        if game.is_over() {
            break;
        }
    }

    println!("Game Over");
    disable_raw_mode().unwrap();

}
