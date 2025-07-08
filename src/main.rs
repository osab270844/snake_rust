use macroquad::prelude::*;
use ::rand::prelude::*;
use std::collections::VecDeque;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const CELL_SIZE: f32 = 20.0;
const CELL_NUMBER_X: i32 = (WINDOW_WIDTH / CELL_SIZE) as i32;
const CELL_NUMBER_Y: i32 = (WINDOW_HEIGHT / CELL_SIZE) as i32;

#[derive(Clone, Copy, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: VecDeque<Position>,
    direction: Direction,
    grow_next: bool,
}

impl Snake {
    fn new() -> Self {
        let mut body = VecDeque::new();
        body.push_back(Position::new(5, 10));
        body.push_back(Position::new(4, 10));
        body.push_back(Position::new(3, 10));
        
        Self {
            body,
            direction: Direction::Right,
            grow_next: false,
        }
    }
    
    fn update(&mut self) {
        let head = *self.body.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => Position::new(head.x, head.y - 1),
            Direction::Down => Position::new(head.x, head.y + 1),
            Direction::Left => Position::new(head.x - 1, head.y),
            Direction::Right => Position::new(head.x + 1, head.y),
        };
        
        self.body.push_front(new_head);
        
        if !self.grow_next {
            self.body.pop_back();
        } else {
            self.grow_next = false;
        }
    }
    
    fn grow(&mut self) {
        self.grow_next = true;
    }
    
    fn change_direction(&mut self, new_direction: Direction) {
        match (&self.direction, &new_direction) {
            (Direction::Up, Direction::Down) => return,
            (Direction::Down, Direction::Up) => return,
            (Direction::Left, Direction::Right) => return,
            (Direction::Right, Direction::Left) => return,
            _ => self.direction = new_direction,
        }
    }
    
    fn check_wall_collision(&self) -> bool {
        let head = *self.body.front().unwrap();
        head.x < 0 || head.x >= CELL_NUMBER_X || head.y < 0 || head.y >= CELL_NUMBER_Y
    }
    
    fn check_self_collision(&self) -> bool {
        let head = *self.body.front().unwrap();
        self.body.iter().skip(1).any(|&segment| segment == head)
    }
    
    fn draw(&self) {
        for segment in &self.body {
            let x = segment.x as f32 * CELL_SIZE;
            let y = segment.y as f32 * CELL_SIZE;
            draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, DARKGREEN);
        }
    }
}

struct Food {
    position: Position,
}

impl Food {
    fn new() -> Self {
        Self {
            position: Self::random_position(),
        }
    }
    
    fn random_position() -> Position {
        let mut rng = thread_rng();
        Position::new(
            rng.gen_range(0..CELL_NUMBER_X),
            rng.gen_range(0..CELL_NUMBER_Y),
        )
    }
    
    fn randomize(&mut self, snake_body: &VecDeque<Position>) {
        loop {
            self.position = Self::random_position();
            if !snake_body.contains(&self.position) {
                break;
            }
        }
    }
    
    fn draw(&self) {
        let x = self.position.x as f32 * CELL_SIZE;
        let y = self.position.y as f32 * CELL_SIZE;
        draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, RED);
    }
}

struct Game {
    snake: Snake,
    food: Food,
    score: u32,
    game_over: bool,
    last_update: f64,
    update_interval: f64,
}

impl Game {
    fn new() -> Self {
        Self {
            snake: Snake::new(),
            food: Food::new(),
            score: 0,
            game_over: false,
            last_update: get_time(),
            update_interval: 0.15, // Update every 150ms
        }
    }
    
    fn update(&mut self) {
        if self.game_over {
            return;
        }
        
        let current_time = get_time();
        if current_time - self.last_update >= self.update_interval {
            self.snake.update();
            self.check_food_collision();
            self.check_game_over();
            self.last_update = current_time;
        }
    }
    
    fn check_food_collision(&mut self) {
        let head = *self.snake.body.front().unwrap();
        if head == self.food.position {
            self.snake.grow();
            self.score += 1;
            self.food.randomize(&self.snake.body);
        }
    }
    
    fn check_game_over(&mut self) {
        if self.snake.check_wall_collision() || self.snake.check_self_collision() {
            self.game_over = true;
        }
    }
    
    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Up) {
            self.snake.change_direction(Direction::Up);
        }
        if is_key_pressed(KeyCode::Down) {
            self.snake.change_direction(Direction::Down);
        }
        if is_key_pressed(KeyCode::Left) {
            self.snake.change_direction(Direction::Left);
        }
        if is_key_pressed(KeyCode::Right) {
            self.snake.change_direction(Direction::Right);
        }
        
        // Restart game on space when game over
        if self.game_over && is_key_pressed(KeyCode::Space) {
            *self = Game::new();
        }
    }
    
    fn draw_background(&self) {
        clear_background(Color::from_rgba(175, 215, 70, 255));
        
        // Draw grass pattern
        let grass_color = Color::from_rgba(167, 209, 61, 255);
        for row in 0..CELL_NUMBER_Y {
            for col in 0..CELL_NUMBER_X {
                let should_draw = if row % 2 == 0 {
                    col % 2 == 0
                } else {
                    col % 2 == 1
                };
                
                if should_draw {
                    let x = col as f32 * CELL_SIZE;
                    let y = row as f32 * CELL_SIZE;
                    draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, grass_color);
                }
            }
        }
    }
    
    fn draw(&self) {
        self.draw_background();
        self.food.draw();
        self.snake.draw();
        
        // Draw score
        let score_text = format!("{}", self.score);
        draw_text(&score_text, WINDOW_WIDTH - 60.0, WINDOW_HEIGHT - 40.0, 36.0, BLACK);
        
        // Draw game over screen
        if self.game_over {
            let game_over_text = "GAME OVER";
            let restart_text = "Press SPACE to restart";
            
            draw_text(
                game_over_text,
                WINDOW_WIDTH / 2.0 - 100.0,
                WINDOW_HEIGHT / 2.0 - 20.0,
                48.0,
                BLACK,
            );
            draw_text(
                restart_text,
                WINDOW_WIDTH / 2.0 - 120.0,
                WINDOW_HEIGHT / 2.0 + 20.0,
                24.0,
                BLACK,
            );
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake Game - Rust".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    
    loop {
        game.handle_input();
        game.update();
        game.draw();
        
        next_frame().await;
    }
}
