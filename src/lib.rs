use crossterm::{terminal, execute, event, style, cursor, ExecutableCommand};

use std::io::{stdout, Write};
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

pub struct Snake {
    body: VecDeque<Point>,
    pub direction: Point,
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

    pub fn head(&self) -> Point {
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

pub struct Food {
    location: Point,
}

impl Food {
    fn new(location: Point) -> Food {
        Food {location}
    }
}

pub struct Game {
    pub snake: Snake,
    food: Food,
    width: i16,
    height: i16,
}

impl Game {
    // TODO, Whole lotta hard coded shit here
    pub fn new() -> Game {
        Game {
            snake: Snake::new(Point {x: 1, y: 1}),
            food: Food::new(Point{x: 10, y: 10}),
            width: 120,
            height: 120,
        }
    }

    pub fn build(width: i16, height: i16, food: Point, start_loc: Point) -> Game {
        //
        Game{
            snake: Snake::new(start_loc),
            food: Food::new(food),
            width,
            height,
        }
    }

    pub fn draw(&self) {
        let mut stdout = stdout();

        // clear that terminal
        stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        
        // draw the snake
        for segment in &self.snake.body {
            // set the cursor
            stdout.execute(
                cursor::MoveTo(segment.x as u16, segment.y as u16)
            ).unwrap();
            print!("u");
        }
        stdout.execute(cursor::MoveTo(self.snake.body[0].x as u16, self.snake.body[0].y as u16)).unwrap();
        print!("O"); 

        // draw the food
        stdout.execute(cursor::MoveTo(self.food.location.x as u16, self.food.location.y as u16)).unwrap();
        print!("x");

        stdout.flush().unwrap();
    }

    pub fn update(&mut self) {
        self.snake.move_forward();
        // check for eaten food
        if self.food.location == self.snake.head() {
            self.snake.grow();
            // new food
            self.food.location = Point {
                x: self.food.location.y * 3 % self.width,
                y: self.food.location.x * 3 % self.width,
            }
        }
    }

    pub fn is_over(&self) -> bool {
        let head = self.snake.head();
        head.x < 0 || head.x >= self.width || head.y < 0 || head.y >= self.height || self.snake.is_collision()
    }


}

