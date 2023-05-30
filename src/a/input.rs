use sdl2::EventPump;

use crate::a::GameState;

pub fn detect_input(
    event_pump: &mut EventPump,
    game: &mut GameState,
) -> Result<bool, failure::Error> {
    for event in event_pump.poll_iter() {
        match event {
            sdl2::event::Event::Quit { .. } => return Ok(true),
            sdl2::event::Event::Window {
                win_event: sdl2::event::WindowEvent::Resized(w, h),
                ..
            } => {
                game.viewport.update_size(w, h);
                game.viewport.set_used();
            }
            sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                game.md(
                    (x as f32 - game.viewport.w as f32 / 2f32) / game.viewport.w as f32,
                    (y as f32 - game.viewport.h as f32 / 2f32) / game.viewport.h as f32,
                )?;
            }
            sdl2::event::Event::MouseButtonUp { .. } => {
                game.mu()?;
            }
            _ => {}
        }
    }
    Ok(false)
}
