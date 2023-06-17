use crate::a::c::e::{Glyph, Style};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ItemAbility {
    Augment(Glyph, u8),
    Lean(Style, u8),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Item {
    value: u32,
    name: &'static str,
    glyph: Glyph,
    ability: ItemAbility,
}

impl Item {
    pub const fn new(value: u32, name: &'static str, glyph: Glyph, ability: ItemAbility) -> Self {
        Self {
            value,
            name,
            glyph,
            ability,
        }
    }
}

pub mod rings {
    use super::{Glyph, Item, ItemAbility};

    pub const RING_OF_FIRE: Item = Item::new(
        100,
        "Ring of Fire",
        Glyph::Fire,
        ItemAbility::Augment(Glyph::Fire, 1),
    );
    pub const RING_OF_WATER: Item = Item::new(
        100,
        "Ring of Water",
        Glyph::Water,
        ItemAbility::Augment(Glyph::Water, 1),
    );
    pub const RING_OF_AIR: Item = Item::new(
        100,
        "Ring of Air",
        Glyph::Air,
        ItemAbility::Augment(Glyph::Air, 1),
    );
    pub const RING_OF_EARTH: Item = Item::new(
        100,
        "Ring of Earth",
        Glyph::Earth,
        ItemAbility::Augment(Glyph::Earth, 1),
    );
    pub const RING_OF_VOID: Item = Item::new(
        100,
        "Ring of Void",
        Glyph::Void,
        ItemAbility::Augment(Glyph::Void, 1),
    );

    pub const ALL: [Item; 5] = [
        RING_OF_FIRE,
        RING_OF_WATER,
        RING_OF_AIR,
        RING_OF_EARTH,
        RING_OF_VOID,
    ];
}

pub mod amulets {
    use super::{Glyph, Item, ItemAbility};

    pub const AMULET_OF_FIRE: Item = Item::new(
        100,
        "Amulet of Fire",
        Glyph::Fire,
        ItemAbility::Augment(Glyph::Fire, 1),
    );
    pub const AMULET_OF_WATER: Item = Item::new(
        100,
        "Amulet of Water",
        Glyph::Water,
        ItemAbility::Augment(Glyph::Water, 1),
    );
    pub const AMULET_OF_AIR: Item = Item::new(
        100,
        "Amulet of Air",
        Glyph::Air,
        ItemAbility::Augment(Glyph::Air, 1),
    );
    pub const AMULET_OF_EARTH: Item = Item::new(
        100,
        "Amulet of Earth",
        Glyph::Earth,
        ItemAbility::Augment(Glyph::Earth, 1),
    );
    pub const AMULET_OF_VOID: Item = Item::new(
        100,
        "Amulet of Void",
        Glyph::Void,
        ItemAbility::Augment(Glyph::Void, 1),
    );

    pub const ALL: [Item; 5] = [
        AMULET_OF_FIRE,
        AMULET_OF_WATER,
        AMULET_OF_AIR,
        AMULET_OF_EARTH,
        AMULET_OF_VOID,
    ];
}

pub mod scrolls {
    use super::{Glyph, Item, ItemAbility};

    pub const SCROLL_OF_FIRE: Item = Item::new(
        100,
        "Scroll of Fire",
        Glyph::Fire,
        ItemAbility::Augment(Glyph::Fire, 1),
    );
    pub const SCROLL_OF_WATER: Item = Item::new(
        100,
        "Scroll of Water",
        Glyph::Water,
        ItemAbility::Augment(Glyph::Water, 1),
    );
    pub const SCROLL_OF_AIR: Item = Item::new(
        100,
        "Scroll of Air",
        Glyph::Air,
        ItemAbility::Augment(Glyph::Air, 1),
    );
    pub const SCROLL_OF_EARTH: Item = Item::new(
        100,
        "Scroll of Earth",
        Glyph::Earth,
        ItemAbility::Augment(Glyph::Earth, 1),
    );
    pub const SCROLL_OF_VOID: Item = Item::new(
        100,
        "Scroll of Void",
        Glyph::Void,
        ItemAbility::Augment(Glyph::Void, 1),
    );

    pub const ALL: [Item; 5] = [
        SCROLL_OF_FIRE,
        SCROLL_OF_WATER,
        SCROLL_OF_AIR,
        SCROLL_OF_EARTH,
        SCROLL_OF_VOID,
    ];
}

pub mod potions {
    use super::{Glyph, Item, ItemAbility};

    pub const POTION_OF_FIRE: Item = Item::new(
        100,
        "Potion of Fire",
        Glyph::Fire,
        ItemAbility::Augment(Glyph::Fire, 1),
    );
    pub const POTION_OF_WATER: Item = Item::new(
        100,
        "Potion of Water",
        Glyph::Water,
        ItemAbility::Augment(Glyph::Water, 1),
    );
    pub const POTION_OF_AIR: Item = Item::new(
        100,
        "Potion of Air",
        Glyph::Air,
        ItemAbility::Augment(Glyph::Air, 1),
    );
    pub const POTION_OF_EARTH: Item = Item::new(
        100,
        "Potion of Earth",
        Glyph::Earth,
        ItemAbility::Augment(Glyph::Earth, 1),
    );
    pub const POTION_OF_VOID: Item = Item::new(
        100,
        "Potion of Void",
        Glyph::Void,
        ItemAbility::Augment(Glyph::Void, 1),
    );

    pub const ALL: [Item; 5] = [
        POTION_OF_FIRE,
        POTION_OF_WATER,
        POTION_OF_AIR,
        POTION_OF_EARTH,
        POTION_OF_VOID,
    ];
}

pub mod books {
    use super::{Glyph, Item, ItemAbility, Style};
    use std::collections::HashMap;

    pub const FIERY_BOOK_OF_ELDRICH_HORRORS: Item = Item::new(
        100,
        "Fiery Book of Eldrich Horrors",
        Glyph::Fire,
        ItemAbility::Lean(Style::Eldrich, 1),
    );
    pub const WATERY_BOOK_OF_ELDRICH_HORRORS: Item = Item::new(
        100,
        "Watery Book of Eldrich Horrors",
        Glyph::Water,
        ItemAbility::Lean(Style::Eldrich, 1),
    );
    pub const AIRY_BOOK_OF_ELDRICH_HORRORS: Item = Item::new(
        100,
        "Airy Book of Eldrich Horrors",
        Glyph::Air,
        ItemAbility::Lean(Style::Eldrich, 1),
    );
    pub const EARTHY_BOOK_OF_ELDRICH_HORRORS: Item = Item::new(
        100,
        "Earthy Book of Eldrich Horrors",
        Glyph::Earth,
        ItemAbility::Lean(Style::Eldrich, 1),
    );
    pub const VOIDY_BOOK_OF_ELDRICH_HORRORS: Item = Item::new(
        100,
        "Voidy Book of Eldrich Horrors",
        Glyph::Void,
        ItemAbility::Lean(Style::Eldrich, 1),
    );

    pub const FIERY_BOOK_OF_ANCIENT_ARTS: Item = Item::new(
        100,
        "Fiery Book of Ancient Arts",
        Glyph::Fire,
        ItemAbility::Lean(Style::Ancient, 1),
    );
    pub const WATERY_BOOK_OF_ANCIENT_ARTS: Item = Item::new(
        100,
        "Watery Book of Ancient Arts",
        Glyph::Water,
        ItemAbility::Lean(Style::Ancient, 1),
    );
    pub const AIRY_BOOK_OF_ANCIENT_ARTS: Item = Item::new(
        100,
        "Airy Book of Ancient Arts",
        Glyph::Air,
        ItemAbility::Lean(Style::Ancient, 1),
    );
    pub const EARTHY_BOOK_OF_ANCIENT_ARTS: Item = Item::new(
        100,
        "Earthy Book of Ancient Arts",
        Glyph::Earth,
        ItemAbility::Lean(Style::Ancient, 1),
    );
    pub const VOIDY_BOOK_OF_ANCIENT_ARTS: Item = Item::new(
        100,
        "Voidy Book of Ancient Arts",
        Glyph::Void,
        ItemAbility::Lean(Style::Ancient, 1),
    );

    pub const FIERY_BOOK_OF_ARCANE_SECRETS: Item = Item::new(
        100,
        "Fiery Book of Arcane Secrets",
        Glyph::Fire,
        ItemAbility::Lean(Style::Arcane, 1),
    );
    pub const WATERY_BOOK_OF_ARCANE_SECRETS: Item = Item::new(
        100,
        "Watery Book of Arcane Secrets",
        Glyph::Water,
        ItemAbility::Lean(Style::Arcane, 1),
    );
    pub const AIRY_BOOK_OF_ARCANE_SECRETS: Item = Item::new(
        100,
        "Airy Book of Arcane Secrets",
        Glyph::Air,
        ItemAbility::Lean(Style::Arcane, 1),
    );
    pub const EARTHY_BOOK_OF_ARCANE_SECRETS: Item = Item::new(
        100,
        "Earthy Book of Arcane Secrets",
        Glyph::Earth,
        ItemAbility::Lean(Style::Arcane, 1),
    );
    pub const VOIDY_BOOK_OF_ARCANE_SECRETS: Item = Item::new(
        100,
        "Voidy Book of Arcane Secrets",
        Glyph::Void,
        ItemAbility::Lean(Style::Arcane, 1),
    );

    pub const FIERY_BOOK_OF_ELDER_KNOWLEDGE: Item = Item::new(
        100,
        "Fiery Book of Elder Knowledge",
        Glyph::Fire,
        ItemAbility::Lean(Style::Elder, 1),
    );
    pub const WATERY_BOOK_OF_ELDER_KNOWLEDGE: Item = Item::new(
        100,
        "Watery Book of Elder Knowledge",
        Glyph::Water,
        ItemAbility::Lean(Style::Elder, 1),
    );
    pub const AIRY_BOOK_OF_ELDER_KNOWLEDGE: Item = Item::new(
        100,
        "Airy Book of Elder Knowledge",
        Glyph::Air,
        ItemAbility::Lean(Style::Elder, 1),
    );
    pub const EARTHY_BOOK_OF_ELDER_KNOWLEDGE: Item = Item::new(
        100,
        "Earthy Book of Elder Knowledge",
        Glyph::Earth,
        ItemAbility::Lean(Style::Elder, 1),
    );
    pub const VOIDY_BOOK_OF_ELDER_KNOWLEDGE: Item = Item::new(
        100,
        "Voidy Book of Elder Knowledge",
        Glyph::Void,
        ItemAbility::Lean(Style::Elder, 1),
    );

    pub const ALL: [Item; 20] = [
        FIERY_BOOK_OF_ELDRICH_HORRORS,
        WATERY_BOOK_OF_ELDRICH_HORRORS,
        AIRY_BOOK_OF_ELDRICH_HORRORS,
        EARTHY_BOOK_OF_ELDRICH_HORRORS,
        VOIDY_BOOK_OF_ELDRICH_HORRORS,
        FIERY_BOOK_OF_ANCIENT_ARTS,
        WATERY_BOOK_OF_ANCIENT_ARTS,
        AIRY_BOOK_OF_ANCIENT_ARTS,
        EARTHY_BOOK_OF_ANCIENT_ARTS,
        VOIDY_BOOK_OF_ANCIENT_ARTS,
        FIERY_BOOK_OF_ARCANE_SECRETS,
        WATERY_BOOK_OF_ARCANE_SECRETS,
        AIRY_BOOK_OF_ARCANE_SECRETS,
        EARTHY_BOOK_OF_ARCANE_SECRETS,
        VOIDY_BOOK_OF_ARCANE_SECRETS,
        FIERY_BOOK_OF_ELDER_KNOWLEDGE,
        WATERY_BOOK_OF_ELDER_KNOWLEDGE,
        AIRY_BOOK_OF_ELDER_KNOWLEDGE,
        EARTHY_BOOK_OF_ELDER_KNOWLEDGE,
        VOIDY_BOOK_OF_ELDER_KNOWLEDGE,
    ];

    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref BY_STYLE: HashMap<Style, Item> = {
            let mut books = HashMap::with_capacity(20);
            books.insert(Style::Eldrich, FIERY_BOOK_OF_ELDRICH_HORRORS);
            books.insert(Style::Eldrich, WATERY_BOOK_OF_ELDRICH_HORRORS);
            books.insert(Style::Eldrich, AIRY_BOOK_OF_ELDRICH_HORRORS);
            books.insert(Style::Eldrich, EARTHY_BOOK_OF_ELDRICH_HORRORS);
            books.insert(Style::Eldrich, VOIDY_BOOK_OF_ELDRICH_HORRORS);

            books.insert(Style::Ancient, FIERY_BOOK_OF_ANCIENT_ARTS);
            books.insert(Style::Ancient, WATERY_BOOK_OF_ANCIENT_ARTS);
            books.insert(Style::Ancient, AIRY_BOOK_OF_ANCIENT_ARTS);
            books.insert(Style::Ancient, EARTHY_BOOK_OF_ANCIENT_ARTS);
            books.insert(Style::Ancient, VOIDY_BOOK_OF_ANCIENT_ARTS);

            books.insert(Style::Arcane, FIERY_BOOK_OF_ARCANE_SECRETS);
            books.insert(Style::Arcane, WATERY_BOOK_OF_ARCANE_SECRETS);
            books.insert(Style::Arcane, AIRY_BOOK_OF_ARCANE_SECRETS);
            books.insert(Style::Arcane, EARTHY_BOOK_OF_ARCANE_SECRETS);
            books.insert(Style::Arcane, VOIDY_BOOK_OF_ARCANE_SECRETS);

            books.insert(Style::Elder, FIERY_BOOK_OF_ELDER_KNOWLEDGE);
            books.insert(Style::Elder, WATERY_BOOK_OF_ELDER_KNOWLEDGE);
            books.insert(Style::Elder, AIRY_BOOK_OF_ELDER_KNOWLEDGE);
            books.insert(Style::Elder, EARTHY_BOOK_OF_ELDER_KNOWLEDGE);
            books.insert(Style::Elder, VOIDY_BOOK_OF_ELDER_KNOWLEDGE);
            books
        };
    }
}

pub mod tools {
    use super::{Glyph, Item, ItemAbility};

    pub const TOOL_OF_FIRE: Item = Item::new(
        100,
        "Tool of Fire",
        Glyph::Fire,
        ItemAbility::Augment(Glyph::Fire, 1),
    );
    pub const TOOL_OF_WATER: Item = Item::new(
        100,
        "Tool of Water",
        Glyph::Water,
        ItemAbility::Augment(Glyph::Water, 1),
    );
    pub const TOOL_OF_AIR: Item = Item::new(
        100,
        "Tool of Air",
        Glyph::Air,
        ItemAbility::Augment(Glyph::Air, 1),
    );
    pub const TOOL_OF_EARTH: Item = Item::new(
        100,
        "Tool of Earth",
        Glyph::Earth,
        ItemAbility::Augment(Glyph::Earth, 1),
    );
    pub const TOOL_OF_VOID: Item = Item::new(
        100,
        "Tool of Void",
        Glyph::Void,
        ItemAbility::Augment(Glyph::Void, 1),
    );

    pub const ALL: [Item; 5] = [
        TOOL_OF_FIRE,
        TOOL_OF_WATER,
        TOOL_OF_AIR,
        TOOL_OF_EARTH,
        TOOL_OF_VOID,
    ];
}

pub mod misc {
    use super::{Glyph, Item, ItemAbility};

    pub const MISC_OF_FIRE: Item = Item::new(
        100,
        "Misc of Fire",
        Glyph::Fire,
        ItemAbility::Augment(Glyph::Fire, 1),
    );
    pub const MISC_OF_WATER: Item = Item::new(
        100,
        "Misc of Water",
        Glyph::Water,
        ItemAbility::Augment(Glyph::Water, 1),
    );
    pub const MISC_OF_AIR: Item = Item::new(
        100,
        "Misc of Air",
        Glyph::Air,
        ItemAbility::Augment(Glyph::Air, 1),
    );
    pub const MISC_OF_EARTH: Item = Item::new(
        100,
        "Misc of Earth",
        Glyph::Earth,
        ItemAbility::Augment(Glyph::Earth, 1),
    );
    pub const MISC_OF_VOID: Item = Item::new(
        100,
        "Misc of Void",
        Glyph::Void,
        ItemAbility::Augment(Glyph::Void, 1),
    );

    pub const ALL: [Item; 5] = [
        MISC_OF_FIRE,
        MISC_OF_WATER,
        MISC_OF_AIR,
        MISC_OF_EARTH,
        MISC_OF_VOID,
    ];
}
