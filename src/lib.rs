use crossterm::{terminal, execute, event, style, cursor, ExecutableCommand};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: i16,
    y: i16,
}

struct Snake {
    body: VecDeque<Point>,
    direction: Point,
}

impl Snake {
    fn new(start: Point) -> Snake {
        let mut body = VecDeque::new();
        body.push_back(start);
        Snake {
            body,
            direction: Point {x: 1, y: 0}
        }
    }

    fn move_forward(&mut self) {
        let head = self.head();
        let new_head = Point {
            x: head.x + self.direction.x,
            y: head.y + self.direction.y,
        };
        self.body.push_front(new_head);
        self.body.pop_back();
    }

    fn grow(&mut self) {
        let tail = *self.body.back().unwrap();
        self.body.push_back(tail);
    }

    fn head(&self) -> Point {
        *self.body.front().unwrap()
    }

    fn is_collision(&self) -> bool {
        let head = &self.head();
        for body_segment in self.body.iter().skip(1) {
            if *body_segment == *head {
                return true;
            }
        }
        false
    }
}

struct Food {
    location: Point,
}

impl Food {
    fn new(location: Point) -> Food {
        Food {location}
    }
}

struct Game {
    snake: Snake,
    food: Food,
    width: i32,
    height: i32,
}

impl Game {
    // TODO, Whole lotta hard coded shit here
    fn new() -> Game {
        Game {
            snake: Snake::new(Point {x: 1, y: 1}),
            food: Food::new()
        }
    }
}

