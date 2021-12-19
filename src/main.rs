use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, timer, Context, GameResult};
use ggez::event::EventHandler;


struct GameState;

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
    
    let state = GameState;
    event::run(ctx,events_loop, state)
}