use std::cmp::Ordering;

use crate::a::GameState;
use crate::g::animation::animaton::{Animation, SizeMode};
use crate::g::animation::img::Img;
use crate::g::resources::Resources;

use crate::a::e::mon::MonsterType;
use crate::a::e::wiz::Wizard;

use packed_simd_2::u32x2;

use super::e::spell::Spell;
use super::e::spell_book::SpellBook;
use super::e::Glyph;

use items::Item;

pub mod items;
pub mod quests;

#[derive(PartialEq, Eq, Clone)]
pub enum ItemType {
    Ring(Item),
    Amulet(Item),
    Scroll(Item),
    Potion(Item),
    Book(Item),
    Tool(Item),
    Misc(Item),
}

#[derive(PartialEq, Eq, Clone)]
pub enum Reward {
    Gold(u32),
    Item(ItemType),
    SpellBook(SpellBook),
    Glyph(Glyph, u32),
    Learn(Spell),
    Unlock(Quest),
}

#[derive(PartialEq, Eq, Clone)]
pub enum Objective {
    Kill { kind: MonsterType, count: u32 },
    Find { item: ItemType },
    Free { wizard: Wizard },
}

#[derive(PartialEq, Eq, Clone)]
pub struct Quest {
    id: uuid::Uuid,
    name: String,
    objectives: Vec<Objective>,
    rewards: Vec<Reward>,
}
