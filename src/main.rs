use std::cell::RefCell;

use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, timer, Context, GameResult};
use ggez::event::EventHandler;


const GRID_WIDTH: usize = 32;
const GRID_HEIGHT: usize = 32;

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Ground,
}

struct GameState {
    grid: RefCell<[[Tile; GRID_WIDTH]; GRID_HEIGHT]>,
}

impl GameState {
    fn new() -> Self {
        GameState {
            grid: RefCell::new([[Tile::Empty; GRID_WIDTH]; GRID_HEIGHT]),
        }
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
            self.set_tile(i, GRID_HEIGHT, Tile::Ground)?;
        }

        Ok(())
    }
}

impl EventHandler<ggez::GameError> for GameState{
    fn update(&mut self, _ctx: &mut Context) -> GameResult{
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult{
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx,events_loop) = ggez::ContextBuilder::new("Platormer Test", "Arcadia + Tay")
        .window_setup(ggez::conf::WindowSetup::default().title("gaymerz"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0,600.0))
        .build()?;
    
    let mut state = GameState::new();
    state.create_flat_ground();

    event::run(ctx,events_loop, state)
}
