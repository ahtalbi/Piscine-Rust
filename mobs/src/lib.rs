mod mobs;

use mobs::{Boss, Member};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn recruit(&mut self, member: (&str, u32)) {
        self.members.insert(member.0.to_string(), Member::new(member.1));
    }

    fn calc_power(&self) -> usize {
        self.members.values().map(|m| m.role.get_power()).sum()
    }

    fn remove_the_youngest_members(&mut self) {
        let min_age = self.members.values().map(|m| m.age).min().unwrap();
        self.members.retain(|_, member| member.age != min_age);
    }

    pub fn attack(&mut self, attacker: &mut Mob) {
        let (winner, loser) = if self.calc_power() > attacker.calc_power() {
            (self, attacker)
        } else {
            (attacker, self)
        };

        loser.remove_the_youngest_members();

        if loser.members.is_empty() {
            winner.wealth += loser.wealth;
            winner.cities.extend(loser.cities.drain());
            loser.wealth = 0;
        }
    }

    pub fn steal(&mut self, target: &mut Mob, value: u64) {
        let stolen = value.min(target.wealth);
        target.wealth -= stolen;
        self.wealth += stolen;
    }

    pub fn conquer_city(&mut self, other_mobs: &[&Mob], city_name: String) {
        let city_taken = other_mobs.iter().any(|mob| mob.cities.contains(&city_name));
        if !city_taken {
            self.cities.insert(city_name);
        }
    }
}