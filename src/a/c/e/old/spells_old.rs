/*use maplit::hashmap;
use std::collections::HashMap;

use std::cmp::Ordering;

use crate::a::b::{Board, Target};
use crate::a::e::wiz::{Wizard, WizardState};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Item {
    Gem(Glyph, u8, u16),
    Potion(Glyph, u8, u16),
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Potion {
    Mana,
    Health,
    Sanity,
}

pub enum Damage {
    Single((Glyph, u32)),
    Dual((Glyph, u32), (Glyph, u32)),
    Tri((Glyph, u32), (Glyph, u32), (Glyph, u32)),
}

impl Damage {
    fn str(&self, s: u32) -> Damage {
        use Damage::*;
        match self {
            Single((g, d)) => Single((g.clone(), d * s)),
            Dual((g1, d1), (g2, d2)) => Dual((g1.clone(), d1 * s), (g2.clone(), d2 * s)),
            Tri((g1, d1), (g2, d2), (g3, d3)) => Tri((g1.clone(), d1 * s), (g2.clone(), d2 * s), (g3.clone(), d3 * s)),
        }
    }

    fn amp(&self, glyph: Glyph) -> u32 {
        use Damage::*;
        match self {
            Single((g, d)) if *g == glyph => *d * 2u32,
            Single((g, d)) => *d,
            Dual((g1, d1), (g2, d2)) if *g1 == glyph => *d1 * 2u32 + *d2,
            Dual((g1, d1), (g2, d2)) if *g2 == glyph => *d1 + *d2 * 2u32,
            Dual((g1, d1), (g2, d2)) => *d1 + *d2,
            Tri((g1, d1), (g2, d2), (g3, d3)) if *g1 == glyph => *d1 * 2u32 + *d2 + *d3,
            Tri((g1, d1), (g2, d2), (g3, d3)) if *g2 == glyph => *d1 + *d2 * 2u32 + *d3,
            Tri((g1, d1), (g2, d2), (g3, d3)) if *g3 == glyph => *d1 + *d2 + *d3 * 2u32,
            Tri((g1, d1), (g2, d2), (g3, d3)) => *d1 + *d2 + *d3,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Glyph {
    Fire,
    Water,
    Earth,
    Air,
    Void,
}

impl Glyph {
    fn opposite(&self) -> Glyph {
        use Glyph::*;
        match self {
            Fire => Earth,
            Earth => Air,
            Air => Water,
            Water => Fire,
            Void => Void,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum TargetType {
    Me(u8), Closest(u8), CenterOfCluster(u8, u8),
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Ability {
    AoE, Cone, Point, Line,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Recipe {
    pub glyph: Glyph,
    pub target: TargetType,
    pub ability: Ability,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Spell {
    ExplodeSelf, // Fire Self Aoe
    FlameWall, // Fire Closest Line
    FireBomb, // Fire CenterOfCluster Aoe
    Flare, // Fire Closest Point
    FlameWhip, // Fire CenterOfCluster Cone

    ArrowOfPower, // Void Closest Point
    EldrichSpikes, // Void Self Line
    UnendingHunger, // Void CenterOfCluster Point
}
use packed_simd_2::u32x2;
impl TargetType {
    fn from(&self, board: &mut Board, position: &u32x2) -> Vec<crate::a::b::Target> {
        use TargetType::*;
        match self {
            Me(range) => board.find_within(position, *range),
            CenterOfCluster(from_origin_range, ability_range) => board.find_cluster_within(position, *from_origin_range, *ability_range),
            &Closest(range) => board.find_closest(position, range).map_or(vec![], |closest| vec![closest]),
        }
    }
}
impl Ability {  

    fn dmg(&self) -> u32 {
        match self {
            AoE => 1u32,
            Cone => 2u32,
            Line => 2u32,
            Point => 3u32,
        }
    }
}
impl Spell {
    fn recipe(&self) -> Recipe {
        match self {
            ExplodeSelf => Recipe { glyph: Glyph::Fire, target: TargetType::Me(2u8), ability: Ability::AoE },
            FlameWall => Recipe { glyph: Glyph::Fire, target: TargetType::Closest(5u8), ability: Ability::Line },
            FireBomb => Recipe { glyph: Glyph::Fire, target: TargetType::CenterOfCluster(4u8, 2u8), ability: Ability::AoE },
            Flare => Recipe { glyph: Glyph::Fire, target: TargetType::Closest, ability: Ability::Point },
            FlameWhip => Recipe { glyph: Glyph::Fire, target: TargetType::CenterOfCluster(2u8, 2u8), ability: Ability::Cone },

            ArrowOfPower => Recipe { glyph: Glyph::Void, target: TargetType::Closest, ability: Ability::Point },
            EldrichSpikes => Recipe { glyph: Glyph::Void, target: TargetType::Me, ability: Ability::Line },
            UnendingHunger => Recipe { glyph: Glyph::Void, target: TargetType::CenterOfCluster, ability: Ability::Point },
        }
    }

    fn strength(&self, book: &SpellBook) -> u32 {
        use Glyph::*;
        match self.recipe().glyph {
            Fire => book.fire_str,
            Air => book.air_str,
            Earth => book.earth_str,
            Water => book.water_str,
            Void => book.void_str,
        }
    }

    fn act(&self, book: &SpellBook, wiz: &Wizard, board: &mut Board) -> bool {
        let strength = book.spell_strengths.get(self).unwrap();
        // TODO
        true
    }

    fn compute_score(&self, book: &SpellBook, wiz: &Wizard, board: &mut Board) -> i32 {
        match wiz.state {
            WizardState::Hunting => {
                if let Some(target) = self.recipe().target.from(&mut board, &wiz.position, self.recipe().ability.range()) {
                    target.compute_dmg(self, book)
                } else {
                    -50
                }
            }
            _ => -100,
        }
    }

    fn score(a: &Spell, b: &Spell, scores: &HashMap<Spell, i32>) -> Ordering {
        match scores.get(b).unwrap() - scores.get(a).unwrap() {
            0 => Ordering::Equal,
            1_i32..=i32::MAX => Ordering::Greater,
            i32::MIN..=-1_i32 => Ordering::Less,
        }
    }
}

pub struct SpellBook {
    spells: Vec<Spell>,
    spell_strengths: HashMap<Spell, u32>,
    spell_dmg: HashMap<Glyph, HashMap<Spell, u32>>,
    max_dmg: HashMap<Glyph, u32>,
    fire_str: u32,
    air_str: u32,
    earth_str: u32,
    water_str: u32,
    void_str: u32,
}

impl SpellBook {
    pub fn new() -> SpellBook {
        SpellBook {
            spells: Vec::new(),
            spell_strengths: HashMap::new(),
            spell_dmg: HashMap::new(),
            max_dmg: HashMap::new(),
            fire_str: 0u32,
            air_str: 0u32,
            water_str: 0u32,
            earth_str: 0u32,
            void_str: 0u32,
        }
    }

    fn dmg(&self, spell: &Spell, target_glyph: &Glyph) -> Option<&u32> {
        self.spell_dmg
            .get(&target_glyph.opposite())
            .unwrap()
            .get(spell)
    }

    fn max_dmg(&self, target_glyph: &Glyph) -> Option<&u32> {
        self.max_dmg.get(&target_glyph.opposite())
    }

    fn act(&self, wiz: &mut Wizard, board: &mut Board) {
        let mut scores = HashMap::new();
        for spell in self.spells.iter() {
            scores.insert(spell.clone(), spell.compute_score(self, &wiz, board));
        }
        let mut tmp_spells = self.spells.clone();
        tmp_spells.sort_by(|a, b| Spell::score(a, b, &scores));
        if let Some(spell) = tmp_spells.get(0) {
            spell.act(self, wiz, board);
        }
    }

    fn refresh_spells(&mut self) {
        use Glyph::*;
        let mut strengths = HashMap::new();
        let mut dmgs = hashmap! { Fire => HashMap::new(), Earth => HashMap::new(), Earth => HashMap::new(), Air => HashMap::new(), Void => HashMap::new() };
        let mut max_dmg =
            hashmap! { Fire => 0u32, Earth => 0u32, Earth => 0u32, Air => 0u32, Void => 0u32 };
        for spell in self.spells.iter() {
            let strength = spell.strength(self);
            let dmg = spell.dmg().str(strength);

            let f_dmg = dmg.amp(Fire);
            let w_dmg = dmg.amp(Water);
            let a_dmg = dmg.amp(Air);
            let e_dmg = dmg.amp(Earth);
            let v_dmg = dmg.amp(Void);
            dmgs.get_mut(&Fire).unwrap().insert(spell.clone(), f_dmg);
            dmgs.get_mut(&Water).unwrap().insert(spell.clone(), w_dmg);
            dmgs.get_mut(&Air).unwrap().insert(spell.clone(), a_dmg);
            dmgs.get_mut(&Earth).unwrap().insert(spell.clone(), e_dmg);
            dmgs.get_mut(&Void).unwrap().insert(spell.clone(), v_dmg);
            strengths.insert(spell.clone(), strength);
            if f_dmg > *max_dmg.get(&Fire).unwrap() {
                max_dmg.insert(Fire, f_dmg);
            }
            if w_dmg > *max_dmg.get(&Water).unwrap() {
                max_dmg.insert(Water, w_dmg);
            }
            if e_dmg > *max_dmg.get(&Earth).unwrap() {
                max_dmg.insert(Fire, e_dmg);
            }
            if a_dmg > *max_dmg.get(&Air).unwrap() {
                max_dmg.insert(Fire, a_dmg);
            }
            if v_dmg > *max_dmg.get(&Void).unwrap() {
                max_dmg.insert(Fire, v_dmg);
            }
        }
        self.spell_strengths = strengths;
        self.spell_dmg = dmgs;
        self.max_dmg = max_dmg;
    }
}
*/