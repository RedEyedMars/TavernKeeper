pub mod adventures;

use adventures::Adventure;

use super::c::e::wiz::{Affinity, Acceptance};

pub enum RealmLocation {
    City {
        name: String,
        description: String,
        difficulty: u8,
    },
    Dungeon {
        name: String,
        description: String,
        difficulty: u8,
    },
    Wilderness {
        name: String,
        description: String,
        difficulty: u8,
    },
}

pub struct Realm {
    pub name: String,
    pub description: String,
    pub locations: Vec<RealmLocation>,
    pub location_links: Vec<(usize, usize)>,
    pub adventures: Vec<Adventure>,
    pub affinity: Affinity,
    pub acceptance: Acceptance,
}