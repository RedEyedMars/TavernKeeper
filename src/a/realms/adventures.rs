use uuid::Uuid;

use crate::a::c::Colosseum;
use crate::a::c::e::mon::Monster;
use crate::a::c::e::wiz::Wizard;
use crate::a::q::battle::{Battle, Tick, BattleEvent};

use super::super::{q::Quest, c::e::party::Party};
use super::RealmLocation;

enum AdventureEndEvent {
    Success,
    Failure,
}
enum AdventureEvent {
    Start(Uuid),// party
    End(AdventureEndEvent),
    BeginQuest(usize),// quest index
    CompleteQuest(usize),// quest index
    BeginBattle(usize),
    BattleFinish(usize, BattleEvent),// battle index, battle event
}

pub struct Adventure {
    pub name: String,
    current_quest: Option<usize>,
    pub quests: Vec<Quest>,
    pub location: RealmLocation,
    pub party: Option<Party>,
    events: Vec<AdventureEvent>,
    current_battle: Option<Battle>,
    battles: Vec<Battle>,
    tick: Tick,
}

impl Adventure {
    pub fn new(name: String, quests: Vec<Quest>, location: RealmLocation) -> Adventure {
        Adventure {
            name,
            location,
            party: None,
            current_quest: None,
            quests,
            events: Vec::new(),
            current_battle: None,
            battles: Vec::new(),
            tick: Tick::new(),
        }
    }

    pub fn start(&mut self, party: Party) {
        self.events.push(AdventureEvent::Start(party.uuid));
        self.party = Some(party);
    }

    pub fn execute(&mut self, col: &mut Colosseum) {
        if self.current_quest.is_none() {
            self.current_quest = Some(0);
            self.events.push(AdventureEvent::BeginQuest(self.current_quest.unwrap()));
        }
        let mut quest = &mut self.quests[self.current_quest.unwrap()];
        while quest.is_complete() {
            if self.current_quest.unwrap() == self.quests.len() - 1 {
                self.events.push(AdventureEvent::End(AdventureEndEvent::Success));
                return;
            }
            self.events.push(AdventureEvent::CompleteQuest(self.current_quest.unwrap()));
            self.current_quest = Some(self.current_quest.unwrap() + 1);
            quest = &mut self.quests[self.current_quest.unwrap()];
            self.events.push(AdventureEvent::BeginQuest(self.current_quest.unwrap()));
        }

        if quest.is_battle() {
            if self.current_battle.is_none() {
                let battle = Battle::new(
                    self.party.as_ref().unwrap().members.clone(), 
                quest.monsters(col));
                self.events.push(AdventureEvent::BeginBattle(self.battles.len()));
                self.battles.push(battle);
            }

            if self.current_battle.is_some() {
                self.tick = self.current_battle.as_mut().unwrap().tick(&mut self.tick, col);
            }
/*
            if let Some(event) = self.tick.iter().find(|evnt| *evnt == &BattleEvent::Victory || *evnt == &BattleEvent::Defeat) {
                if let BattleEvent::Victory = event {
                    quest.win_battle(wizards, monsters, self.party.as_mut().unwrap(), self.current_battle.as_ref().unwrap());
                } else {
                    quest.lose_battle(wizards, monsters, self.party.as_mut().unwrap(), self.current_battle.as_ref().unwrap());
                }
                self.events.push(AdventureEvent::BattleFinish(self.battles.len(), event.clone()));
                self.current_battle = None;
                self.tick = Tick::new();
            }
*/
        }
    }

}
