use std::cell::RefCell;
use std::fmt;

use ggez::event::{KeyCode, KeyMods};
use ggez::event::EventHandler;
use ggez::{event, graphics, /* timer,*/ Context, GameResult, GameError};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

const GRID_WIDTH: usize = 32;
const GRID_HEIGHT: usize = 32;

const TILE_WIDTH: f32 = WINDOW_WIDTH / (GRID_WIDTH as f32);
const TILE_HEIGHT: f32 = WINDOW_HEIGHT / (GRID_HEIGHT as f32);

#[derive(Copy, Clone, Debug)]
enum Tile {
    Empty,
    Ground,
    Player,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Stationary,
    Up,
    Down,
    Left,
    Right,
}

struct GameState {
    grid: RefCell<[[Tile; GRID_WIDTH]; GRID_HEIGHT]>,
    player_position: (usize, usize),
    player_direction: Direction,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let grid = self.grid.borrow();

        for y in 0..GRID_HEIGHT {
            write!(f, "{:?}\n", grid[y])?;
        }

        Ok(())
    }
}

impl GameState {
    fn new() -> Self {
        GameState {
            grid: RefCell::new([[Tile::Empty; GRID_WIDTH]; GRID_HEIGHT]),
            player_position: (4, GRID_HEIGHT - 2),
            player_direction: Direction::Stationary,
        }
    }

    fn update(&mut self) {
        let (x, y) = self.player_position;
        self.grid.borrow_mut()[y][x] = Tile::Empty;

        match self.player_direction {
            Direction::Up         => self.player_position.1 -= 1,
            Direction::Down       => self.player_position.1 += 1,
            Direction::Left       => self.player_position.0 -= 1,
            Direction::Right      => self.player_position.0 += 1,
            Direction::Stationary => (),
        }

        let (x, y) = self.player_position;
        self.grid.borrow_mut()[y][x] = Tile::Player;
    }

    fn set_tile(&self, x: usize, y: usize, tile: Tile) -> Result<(), &str> {
        if x >= GRID_WIDTH || y >= GRID_HEIGHT {
            return Err("Tile position out of bounds!");
        }

        self.grid.borrow_mut()[y][x] = tile;

        Ok(())
    }

    fn create_flat_ground(&self) -> Result<(), &str> {
        for i in 0..GRID_WIDTH {
            self.set_tile(i, GRID_HEIGHT - 1, Tile::Ground)?;
        }

        Ok(())
    }

    fn change_player_direction(&mut self, dir: Direction) {
        self.player_direction = dir;
    }
}

impl Direction {
    fn from_keycode(keycode: KeyCode) -> Self {
        match keycode {
            KeyCode::Up    => Direction::Up,
            KeyCode::Down  => Direction::Down,
            KeyCode::Left  => Direction::Left,
            KeyCode::Right => Direction::Right,
            _              => Direction::Stationary,
        }
    }
}

impl EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.update();

        self.set_tile(self.player_position.0, self.player_position.1, Tile::Player)
            .map_err(|err| GameError::CustomError(err.to_string()))?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::WHITE);

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let position = graphics::Rect::new(
                    (x as f32) * TILE_WIDTH,
                    (y as f32) * TILE_HEIGHT,
                    TILE_WIDTH,
                    TILE_HEIGHT,
                );

                let brown = graphics::Color::from_rgb(0x6C, 0x28, 0x16);

                let color = match self.grid.borrow()[y][x] {
                    Tile::Empty => graphics::Color::WHITE,
                    Tile::Ground => brown,
                    Tile::Player => graphics::Color::RED,
                };

                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    position,
                    color,
                )?;

                graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
            }
        }

        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        let dir = Direction::from_keycode(keycode);
        self.change_player_direction(dir);
    }
}

fn main() -> GameResult {
    let (ctx, events_loop) = ggez::ContextBuilder::new("Platormer Test", "Arcadia + Tay")
        .window_setup(ggez::conf::WindowSetup::default().title("gaymerz"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;

    let state = GameState::new();
    if let Err(error) = state.create_flat_ground() {
        panic!("Failed to draw {}", error);
    }

    //println!("{}", state);

    event::run(ctx, events_loop, state)
}
