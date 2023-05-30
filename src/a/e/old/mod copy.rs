use std::cmp::Ordering;

use crate::a::GameState;
use crate::g::animation::animaton::{Animation, SizeMode};
use crate::g::animation::img::Img;
use crate::g::resources::Resources;

use crate::a::e::wiz::Wizard;
use crate::a::e::mon::MonsterType;

use packed_simd_2::u32x2;

use super::e::spells::Glyph;

pub enum Target {
    Monster { 
        kind: MonsterType,
        distance: f32,
    },
    Wizard {
        wiz: Wizard, 
        distance: f32,
    },
    //Object(Object),
}

pub struct Monster {
    kind: MonsterType,
    position: u32x2,
}
pub struct Board {
    grid_img: Img,
    wizards: Vec<Wizard>,
    monsters: Vec<Monster>,
}
impl Ord for Target {
    fn cmp(&self, other:&Self) -> Ordering {
        let dist1 = match self {
            Target::Monster { distance, ..} => distance,
            Target::Wizard { distance, .. } => distance,
        };
        let dist2 = match other {
            Target::Monster { distance, ..} => distance,
            Target::Wizard { distance, .. } => distance,
        };
        if dist1 > dist2 {
            Ordering::Less
        }
        if dist1 < dist2 {
            Ordering::Greater
        }
        Ordering::Equal
    }
}
impl PartialOrd for Target {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Target {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Target::Monster { kind, distance } => 
            {
                let kind2 = kind;
                let distance2 = distance;
                match other {
                    Target::Monster { kind, distance } => kind == kind2 && distance == distance2,
                    _ => false,
                }
            },
            Target::Wizard { wiz, distance } => 
            {
                let wiz2 = wiz;
                let distance2 = distance;
                match other {
                    Target::Wizard { wiz, distance } => wiz == wiz2 && distance == distance2,
                    _ => false,
                }
            }
        }
    }
}

impl Eq for Target { }

fn compare(x:f32, y:f32) -> Ordering {
    match x {
        x if x > y => Ordering::Greater,
        x if x < y => Ordering::Less,
        _ => Ordering::Equal,
    }
}
impl Board {
    pub fn new(res: &mut Resources) -> Result<Board, failure::Error> {
        Ok(Board {
            grid_img: Img::new(
                "bot_1.png".to_string(),
                0f32,
                SizeMode::Bot,
                Animation::MainXShift2x16,
                res,
            )?,
            wizards: Vec::new(),
            monsters: Vec::new(),
        })
    }
    pub fn execute(&mut self) -> Result<(), failure::Error> {
        Ok(())
    }
    pub fn render(&self, game: &GameState) -> Result<(), failure::Error> {
        self.grid_img.render(game);
        Ok(())
    }

    pub fn find_closest(&self, position: &u32x2, range: u8) -> Option<Target> {
        self.find_within(position, range).into_iter().min()
    }

    pub fn find_cluster_within(&self, position: &u32x2, from_origin: u8, range: u8) -> Vec<Target> {
        let r_up = (range as f32) + (from_origin as f32) / 2f32;
        let r_down = (range as f32) - (from_origin as f32) / 2f32;
        let p = *position;
        self.monsters.iter()
            .map(|mon| Target::Monster { kind: mon.kind, distance: {
                let pos = mon.position - p;
                let sum = (pos * pos).wrapping_sum();
                (sum as f32).sqrt()
            }})
            .filter(|target| *target.distance() >= r_down && *target.distance() <= r_up)
            .collect()
    }

    pub fn find_within(&self, position: &u32x2, range: u8) -> Vec<Target> {
        let r = range as f32;
        let p = *position;
        self.monsters.iter()
            .map(|mon| Target::Monster { kind: mon.kind, distance: {
                let pos = mon.position - p;
                let sum = (pos * pos).wrapping_sum();
                (sum as f32).sqrt()
            }})
            .filter(|target| *target.distance() <= r)
            .collect()
    }

    pub fn md(&mut self, x: f32, y: f32) -> Result<(), failure::Error> {
        Ok(())
    }

    pub fn mu(&mut self) -> Result<(), failure::Error> {
        Ok(())
    }
}

impl Target {
    pub fn glyph(&self) -> &Glyph {
        match self {
            Target::Monster { kind: kind, .. } => kind.glyph(),
            Target::Wizard { wiz: wiz, .. } => &wiz.glyph
        }
    }

    pub fn distance(&self) -> &f32 {
        match self {
            Target::Monster { distance: distance, .. } => distance,
            Target::Wizard { distance: distance, .. } => distance,
        }
    }
}