use generational_arena::{Arena, Index};
use rand::Rng;

use super::battle::Battle;
use super::items::{amulets, books, misc, potions, rings, scrolls, tools};
use super::{ItemType, Objective, Quest, Reward};
use crate::a::c::Colosseum;
use crate::a::c::e::mon::{Monster, MonsterType};
use crate::a::c::e::party::Party;
use crate::a::c::e::spell::{Spell, spells};
use crate::a::c::e::spell_book::SpellBook;
use crate::a::c::e::wiz::{Acceptance, Wizard};
use crate::a::c::e::{Glyph, Style};

impl Quest {
    pub fn is_complete(&self) -> bool {
        self.is_complete.clone()
    }

    pub fn is_battle(&self) -> bool {
        for objective in &self.objectives {
            match objective {
                Objective::Kill { kind: _, count: _ } => {
                    return true;
                }
                Objective::Find { item: _ } => {
                    return false;
                }
                Objective::Free { wizard: _ } => {
                    return false;
                }
            }
        }
        false
    }

    pub fn monsters(&self, col: &mut Colosseum) -> Vec<Index> {
        use crate::a::c::ColosseumArena;
        let mut monsters = Vec::new();
        for objective in &self.objectives {
            match objective {
                Objective::Kill { kind, count } => {
                    for _ in 0..*count {
                        let mon_id = col.insert(Monster::new(synonym::for_first_name(), kind, 1));
                        monsters.push(mon_id);
                    }
                }
                _ => {}
            }
        }
        monsters
    }

    pub fn win_battle(&mut self, wizards: &mut Arena<Wizard>, monsters: &mut Arena<Monster>, party: &mut Party, battle: &Battle) {
        for enemy in battle.enemies.iter() {
            monsters.remove(*enemy);            
        }
        //let hashset = battle.allies.into_iter().filter(|i| !battle.active_allies.contains(i) && wizards.remove(*i).is_some()).collect::<HashSet<Index>>();
        //party.members.retain(|i| !hashset.contains(i));
        self.is_complete = true;
    }

    pub fn lose_battle(&mut self, wizards: &mut Arena<Wizard>, monsters: &mut Arena<Monster>, party: &mut Party, battle: &Battle) {
        for enemy in battle.enemies.iter() {
            monsters.remove(*enemy);            
        }
        //let hashset = battle.allies.into_iter().filter(|i| !battle.active_allies.contains(i) && wizards.remove(*i).is_some()).collect::<HashSet<Index>>();
        //party.members.retain(|i| !hashset.contains(i));
        self.is_complete = true;
    }


    pub fn generate() -> Quest {
        let mut rng = rand::thread_rng();
        let acceptance = Acceptance::from_style(
            match rng.gen_range(0, 5) {
                0 => Style::Elder,
                1 => Style::Eldrich,
                2 => Style::Ancient,
                3 => Style::Arcane,
                _ => Style::Void,
            },
            1,
        );
        let objectives = Self::generate_objectives(&acceptance);
        Quest {
            id: uuid::Uuid::new_v4(),
            name: Self::generate_name(),
            rewards: Self::generate_rewards(&objectives, &acceptance),
            objectives,
            is_complete: false,
        }
    }

    pub fn generate_name() -> String {
        synonym::for_quest()
            + synonym::for_of_the()
            + synonym::for_adjective()
            + synonym::for_noun()
    }

    fn generate_objectives(acceptance: &Acceptance) -> Vec<Objective> {
        let mut objectives = Vec::new();
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(1, 4);
        for _ in 0..count {
            let objective = match rng.gen_range(0, 3) {
                0 => Objective::Find {
                    item: Self::generate_item(acceptance),
                },
                1 => Objective::Free {
                    wizard: Self::generate_wizard(acceptance),
                },
                _ => {
                    let monster_type = Self::generate_monster_type();
                    Objective::Kill {
                        kind: monster_type.clone(),
                        count: match monster_type.difficulty() {
                            1 => rng.gen_range(3, 7),
                            2 => rng.gen_range(2, 5),
                            3 => rng.gen_range(1, 3),
                            _ => rng.gen_range(1, 1),
                        },
                    }
                }
            };
            objectives.push(objective);
        }
        objectives
    }

    fn generate_monster_type() -> MonsterType {
        let mut rng = rand::thread_rng();
        crate::a::c::e::mon::ALL[rng.gen_range(0, crate::a::c::e::mon::ALL.len())].clone()
    }

    fn generate_item(_acceptance: &Acceptance) -> ItemType {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, 7) {
            0 => ItemType::Ring(rings::ALL[rng.gen_range(0, 5)].clone()),
            1 => ItemType::Amulet(amulets::ALL[rng.gen_range(0, 5)].clone()),
            2 => ItemType::Scroll(scrolls::ALL[rng.gen_range(0, 5)].clone()),
            3 => ItemType::Potion(potions::ALL[rng.gen_range(0, 5)].clone()),
            4 => ItemType::Book(books::ALL[rng.gen_range(0, 20)].clone()),
            5 => ItemType::Tool(tools::ALL[rng.gen_range(0, 5)].clone()),
            _ => ItemType::Misc(misc::ALL[rng.gen_range(0, 5)].clone()),
        }
    }

    fn generate_wizard(acceptance: &Acceptance) -> Wizard {
        let _rng = rand::thread_rng();
        let mut wizard = Wizard::new(Self::generate_wizard_name());
        Self::generate_affinity(&mut wizard);
        Self::generate_acceptance(&mut wizard, acceptance);
        wizard
    }

    fn generate_wizard_name() -> String {
        let mut rng = rand::thread_rng();
        let mut name = String::new();
        let count = rng.gen_range(1, 3);
        for _ in 0..count {
            name.push_str(synonym::for_first_name());
            name.push_str(" ");
        }
        name.push_str(synonym::for_last_name());
        name.push_str(" ");
        name.push_str(synonym::for_adjective());
        name
    }

    fn generate_affinity(wizard: &mut Wizard) {
        let mut rng = rand::thread_rng();
        for _i in 0..rng.gen_range(5, 10) {
            match rng.gen_range(0, 5) {
                0 => wizard.affinity.fire += 1,
                1 => wizard.affinity.air += 1,
                2 => wizard.affinity.earth += 1,
                3 => wizard.affinity.water += 1,
                _ => wizard.affinity.void += 1,
            }
        }
    }

    fn generate_acceptance(wizard: &mut Wizard, acceptance: &Acceptance) {
        let mut rng = rand::thread_rng();
        for _i in 0..rng.gen_range(0, 2) {
            match rng.gen_range(0, 5) {
                0 => wizard.acceptance.elder += 1,
                1 => wizard.acceptance.eldrich += 1,
                2 => wizard.acceptance.ancient += 1,
                3 => wizard.acceptance.arcane += 1,
                _ => wizard.acceptance.void += 1,
            }
        }
        wizard.acceptance.add(acceptance);
    }

    fn generate_rewards(objectives: &Vec<Objective>, acceptance: &Acceptance) -> Vec<Reward> {
        let mut rewards = Vec::new();
        let mut rng = rand::thread_rng();

        let mut num_of_rewards = 1;
        for objective in objectives {
            match objective {
                Objective::Find { item } => {
                    rewards.push(Reward::Item(item.clone()));
                    num_of_rewards -= 1;
                }
                Objective::Free { .. } => num_of_rewards += 2,
                Objective::Kill { count: _, kind } => num_of_rewards += kind.difficulty() as u32,
            }
        }
        for _ in 0..num_of_rewards {
            let reward = match rng.gen_range(0, 5) {
                0 => Reward::Gold(rng.gen_range(1, 100)),
                1 => Reward::Item(Self::generate_item(acceptance)),
                2 => Reward::SpellBook(Self::generate_spell_book(acceptance)),
                3 => Reward::Glyph(Self::generate_glyph(acceptance), rng.gen_range(1, 4)),
                _ => Reward::Learn(Self::generate_spell(acceptance)),
            };
            rewards.push(reward);
        }
        rewards
    }

    fn generate_spell_book(acceptance: &Acceptance) -> SpellBook {
        let mut rng = rand::thread_rng();
        let mut spell_book = SpellBook::new();
        for _ in 0..rng.gen_range(1, 5) {
            spell_book.add_spell(Self::generate_spell(acceptance));
        }
        spell_book
    }

    fn generate_spell(acceptance: &Acceptance) -> Spell {
        let mut rng = rand::thread_rng();
        let choices = &spells::BY_STYLE[acceptance.get_highest()];
        choices[rng.gen_range(0, choices.len())].clone()
    }

    fn generate_glyph(_acceptance: &Acceptance) -> Glyph {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, 5) {
            0 => Glyph::Fire,
            1 => Glyph::Air,
            2 => Glyph::Earth,
            3 => Glyph::Water,
            _ => Glyph::Void,
        }
    }
}

pub mod synonym {
    use rand::Rng;
    pub fn for_quest() -> String {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0, QUESTS.len());
        QUESTS[index].to_string()
    }

    pub fn for_of_the() -> &'static str {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0, OF_THE.len());
        OF_THE[index]
    }

    pub fn for_adjective() -> &'static str {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0, ADJECTIVES.len());
        ADJECTIVES[index]
    }

    pub fn for_noun() -> &'static str {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0, NOUNS.len());
        NOUNS[index]
    }

    pub fn for_first_name() -> &'static str {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0, FIRST_NAME.len());
        FIRST_NAME[index]
    }

    pub fn for_last_name() -> &'static str {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0, LAST_NAME.len());
        LAST_NAME[index]
    }

    const QUESTS: [&str; 10] = [
        "Quest",
        "Adventure",
        "Journey",
        "Expedition",
        "Campaign",
        "Crusade",
        "Mission",
        "Pilgrimage",
        "Odyssey",
        "Voyage",
    ];

    const OF_THE: [&str; 10] = [
        " of the ",
        " to the ",
        " for the ",
        " searching for ",
        " in search of ",
        " in pursuit of ",
        " in the pursuit of ",
        " in the pursuit of the ",
        " in the search of the ",
        " in search of the ",
    ];

    const ADJECTIVES: [&str; 20] = [
        "beneficial",
        "costly",
        "expensive",
        "helpful",
        "important",
        "invaluable",
        "prized",
        "profitable",
        "relevant",
        "scarce",
        "treasured",
        "useful",
        "valued",
        "worthwhile",
        "dangerous",
        "deadly",
        "destructive",
        "fatal",
        "harmful",
        "lethal",
    ];

    const NOUNS: [&str; 20] = [
        "artifact",
        "book",
        "crown",
        "gem",
        "jewel",
        "key",
        "map",
        "relic",
        "ring",
        "scroll",
        "spellbook",
        "staff",
        "sword",
        "tome",
        "wand",
        "weapon",
        "amulet",
        "charm",
        "medallion",
        "necklace",
    ];

    const FIRST_NAME: [&str; 16] = [
        "Bielfazar",
        "Bilbo",
        "Forrest",
        "Greagor",
        "Grimble",
        "Ozerzia",
        "Miframp",
        "Mortimer",
        "Diazibar",
        "Grandtifel",
        "Oroduin",
        "Wizgiz",
        "Zarzoz",
        "Willow",
        "Akcrozak",
        "Esmerella",
    ];

    const LAST_NAME: [&str; 5] = ["Bilgebottom", "Butterbur", "Cotton", "Wayne", "Biddi"];
}
