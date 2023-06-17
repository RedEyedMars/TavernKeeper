

pub mod c;
mod input;
pub mod q;
pub mod realms;

use crate::a::c::Colosseum;
use crate::g::render_gl::Viewport;
use crate::g::resources::Resources;
use sdl2::EventPump;

use std::time::Instant;
use q::battle::Battle;
use c::e::wiz::Wizard;
use c::e::mon::Monster;

use self::c::e::mon::MonsterType;
use self::q::battle::{Tick, BattleEvent};

pub struct GameState {
    pub clock: Instant,
    pub viewport: Viewport,
    pub animation_state: u8,
    pub res: Resources,
    battle: Option<Battle>,
    tick: Tick,
    col: Colosseum,
}

pub fn setup() -> Result<GameState, failure::Error> {
    let res = Resources::from_relative_exe_path("assets").unwrap();

    let viewport = Viewport::for_window(900, 700);
    Ok(GameState {
        clock: Instant::now(),
        viewport,
        animation_state: 0u8,
        battle: None,
        tick: Tick::new(),
        col: Colosseum::new()?,
        res,
    })
}

pub fn run(event_pump: &mut EventPump, game: &mut GameState) -> Result<bool, failure::Error> {
    if input::detect_input(event_pump, game)? {
        return Ok(false);
    }

    game.animation_state = (game.clock.elapsed().as_millis() / 400 % 2) as u8;
    if !game.execute()? {
        return Ok(false);
    }
    game.render(&game)?;
    //
    Ok(true)
}

impl GameState {
    pub fn execute(&mut self) -> Result<bool, failure::Error> {
        use c::ColosseumArena;
        if let None = self.battle {
            let wiz_id = self.col.insert(Wizard::new("Bob".to_string()));
            let wiz: &mut Wizard = self.col.get_mut(wiz_id);
            wiz.add_spell_to_book(c::e::spell::spells::fire::FIREBALL.clone());
            
            let wiz_id = self.col.insert(Wizard::new("Rob".to_string()));
            let wiz: &mut Wizard = self.col.get_mut(wiz_id);
            wiz.add_spell_to_book(c::e::spell::spells::air::LIGHTNING.clone());

            let mon_id = self.col.insert(Monster::new("Tod", &MonsterType::Goblin, 1));
            let mon_id2 = self.col.insert(Monster::new("Sod", &MonsterType::Goblin, 1));
            let mon_id3 = self.col.insert(Monster::new("Vod", &MonsterType::Goblin, 1));

            self.battle = Some(Battle::new(vec![wiz_id], vec![mon_id, mon_id2, mon_id3]));
        }
        self.tick = self.battle.as_mut().unwrap().tick(&mut self.tick, &mut self.col);
        if self.tick.iter().any(|event| event == &BattleEvent::Victory || event == &BattleEvent::Defeat) {
            self.col.insert(self.battle.clone().unwrap());
            self.col.save().unwrap();
            return Ok(false);
        }
        Ok(true)
    }

    pub fn render(&self, _game: &GameState) -> Result<(), failure::Error> {
        Ok(())
    }

    pub fn md(&mut self, _x: f32, _y: f32) -> Result<(), failure::Error> {
        Ok(())
    }

    pub fn mu(&mut self) -> Result<(), failure::Error> {
        Ok(())
    }
}
