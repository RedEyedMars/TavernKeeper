use super::{e::mon::Monster, q::Quest};

pub struct Adventure {
    pub name: String,
    pub quest: Quest,
    pub monster: Monster,
    pub location: Realm,
}

impl Adventure {
    pub fn new(name: String, quest: Quest, monster: Monster, location: Realm) -> Adventure {
        Adventure {
            name,
            quest,
            monster,
            location,
        }
    }
}

pub struct Realm {
    pub name: String,
    pub description: String,
    pub adventures: Vec<Adventure>,
}
