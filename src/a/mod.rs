use failure::err_msg;

pub mod e;
mod input;
pub mod q;
pub mod realms;

use crate::g::render_gl::Viewport;
use crate::g::resources::Resources;
use sdl2::EventPump;
use sdl2::Sdl;
use std::time::Instant;

pub struct GameState {
    pub clock: Instant,
    pub viewport: Viewport,
    pub animation_state: u8,
    pub res: Resources,
}

pub fn setup() -> Result<GameState, failure::Error> {
    let mut res = Resources::from_relative_exe_path("assets").unwrap();

    let viewport = Viewport::for_window(900, 700);
    Ok(GameState {
        clock: Instant::now(),
        viewport,
        animation_state: 0u8,
        res,
    })
}

pub fn run(event_pump: &mut EventPump, game: &mut GameState) -> Result<bool, failure::Error> {
    if input::detect_input(event_pump, game)? {
        return Ok(false);
    }

    game.animation_state = (game.clock.elapsed().as_millis() / 400 % 2) as u8;
    game.execute()?;
    game.render(&game)?;
    //
    Ok(true)
}

impl GameState {
    pub fn execute(&mut self) -> Result<(), failure::Error> {
        Ok(())
    }

    pub fn render(&self, game: &GameState) -> Result<(), failure::Error> {
        Ok(())
    }

    pub fn md(&mut self, x: f32, y: f32) -> Result<(), failure::Error> {
        Ok(())
    }

    pub fn mu(&mut self) -> Result<(), failure::Error> {
        Ok(())
    }
}
