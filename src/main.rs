use std::cell::RefCell;
use std::fmt;

use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, timer, Context, GameResult};
use ggez::event::EventHandler;


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
}

struct GameState {
    grid: RefCell<[[Tile; GRID_WIDTH]; GRID_HEIGHT]>,
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
            self.set_tile(i, GRID_HEIGHT - 1, Tile::Ground)?;
        }

        Ok(())
    }
}

impl EventHandler<ggez::GameError> for GameState{
    fn update(&mut self, _ctx: &mut Context) -> GameResult{
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult{
        graphics::clear(ctx, graphics::Color::WHITE);

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let position = graphics::Rect::new(
                    (x as f32) * TILE_WIDTH,
                    (y as f32) * TILE_HEIGHT,
                    TILE_WIDTH,
                    TILE_HEIGHT
                );

                let brown = graphics::Color::from_rgb(0x6C, 0x28, 0x16);

                let color = match self.grid.borrow()[y][x] {
                    Tile::Empty  => graphics::Color::WHITE,
                    Tile::Ground => brown,
                };

                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    position,
                    color,
                )?;

                graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
            }
        }

        graphics::present(ctx)
    }
}

fn main() -> GameResult {
    let (ctx,events_loop) = ggez::ContextBuilder::new("Platormer Test", "Arcadia + Tay")
        .window_setup(ggez::conf::WindowSetup::default().title("gaymerz"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;
    
    let mut state = GameState::new();
    state.create_flat_ground();

    println!("{}", state);

    event::run(ctx,events_loop, state)
}
