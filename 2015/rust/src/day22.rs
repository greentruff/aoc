use std::cmp::PartialEq;
use std::collections::HashMap;

use crate::aoc::Solution;
use crate::day22::Difficulty::{EasyMode, HardMode};
use crate::day22::Spell::*;
use crate::day22::TurnResult::*;

pub fn solve(input: &str) -> Solution {
    let boss = parse_boss(input);
    let part1 = most_efficient_fight(
        STARTING_HEALTH,
        STARTING_MANA,
        boss.hitpoints,
        boss.damage,
        EasyMode,
    );
    let part2 = most_efficient_fight(
        STARTING_HEALTH,
        STARTING_MANA,
        boss.hitpoints,
        boss.damage,
        HardMode,
    );
    Solution::new(22, &part1, &part2)
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Difficulty {
    EasyMode,
    HardMode,
}

fn most_efficient_fight(
    health: i32,
    mana: i32,
    boss_health: i32,
    boss_damage: i32,
    difficulty: Difficulty,
) -> i32 {
    let mut combinations = Vec::new();
    let mut next_combinations = Vec::new();

    next_combinations.push(FightProgress {
        health,
        mana,
        effects: HashMap::new(),
        boss_health,
        boss_damage,
        past_spells: Vec::new(),
        total_mana_spent: 0,
    });

    let mut min_mana_win = i32::MAX;

    while !next_combinations.is_empty() {
        while let Some(fight) = next_combinations.pop() {
            for next_spell in Spell::VALUES {
                let have_mana = spell_stats(&next_spell).cost <= fight.mana;
                let no_cooldown = *fight.effects.get(&next_spell).unwrap_or(&0) <= 1;
                let is_lower_mana = fight.total_mana_spent < min_mana_win;
                if have_mana && no_cooldown && is_lower_mana {
                    combinations.push((fight.clone(), next_spell));
                }
            }
        }

        while let Some((fight, spell)) = combinations.pop() {
            let mut fight = fight;
            match fight.do_turn(&spell, difficulty) {
                Won => {
                    if fight.total_mana_spent < min_mana_win {
                        min_mana_win = fight.total_mana_spent;
                    }
                }
                InProgress => {
                    next_combinations.push(fight);
                }
                Lost | Invalid => {}
            }
        }
    }

    min_mana_win
}

const STARTING_MANA: i32 = 500;
const STARTING_HEALTH: i32 = 50;

struct Boss {
    hitpoints: i32,
    damage: i32,
}

fn parse_boss(input: &str) -> Boss {
    let mut hitpoints = 0;
    let mut damage = 0;
    for line in input.lines() {
        let mut parts = line.split(": ");
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            match key {
                "Hit Points" => {
                    hitpoints = value.parse().unwrap();
                }
                "Damage" => {
                    damage = value.parse().unwrap();
                }
                _ => {}
            }
        }
    }
    Boss { hitpoints, damage }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    const VALUES: [Self; 5] = [MagicMissile, Drain, Shield, Poison, Recharge];
}

struct SpellStats {
    cost: i32,
    turns: i32,
    damage: i32,
    heal: i32,
    regen: i32,
    armor: i32,
}

/*
   Magic Missile costs 53 mana. It instantly does 4 damage.
   Drain costs 73 mana. It instantly does 2 damage and heals you for 2 hit points.
   Shield costs 113 mana. It starts an effect that lasts for 6 turns. While it is active, your armor is increased by 7.
   Poison costs 173 mana. It starts an effect that lasts for 6 turns. At the start of each turn while it is active, it deals the boss 3 damage.
   Recharge costs 229 mana. It starts an effect that lasts for 5 turns. At the start of each turn while it is active, it gives you 101 new mana.
*/
const MAGIC_MISSILE_STATS: SpellStats = SpellStats {
    cost: 53,
    turns: 1,
    damage: 4,
    heal: 0,
    regen: 0,
    armor: 0,
};
const DRAIN_STATS: SpellStats = SpellStats {
    cost: 73,
    turns: 1,
    damage: 2,
    heal: 2,
    regen: 0,
    armor: 0,
};
const SHIELD_STATS: SpellStats = SpellStats {
    cost: 113,
    turns: 6,
    damage: 0,
    heal: 0,
    regen: 0,
    armor: 7,
};
const POISON_STATS: SpellStats = SpellStats {
    cost: 173,
    turns: 6,
    damage: 3,
    heal: 0,
    regen: 0,
    armor: 0,
};
const RECHARGE_STATS: SpellStats = SpellStats {
    cost: 229,
    turns: 5,
    damage: 0,
    heal: 0,
    regen: 101,
    armor: 0,
};

fn spell_stats(spell: &Spell) -> &SpellStats {
    match spell {
        MagicMissile => &MAGIC_MISSILE_STATS,
        Drain => &DRAIN_STATS,
        Shield => &SHIELD_STATS,
        Poison => &POISON_STATS,
        Recharge => &RECHARGE_STATS,
    }
}

#[derive(Debug, Eq, PartialEq)]
enum TurnResult {
    Won,
    Lost,
    InProgress,
    Invalid,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct FightProgress {
    health: i32,
    mana: i32,
    effects: HashMap<Spell, i32>,

    boss_health: i32,
    boss_damage: i32,

    past_spells: Vec<Spell>,
    total_mana_spent: i32,
}

impl FightProgress {
    fn apply_turn_effects(&mut self) -> i32 {
        let mut armor = 0;
        for (eff_spell, turns) in self.effects.iter_mut() {
            let eff_stats = spell_stats(eff_spell);
            if *turns > 0 {
                *turns -= 1;

                self.boss_health -= eff_stats.damage;
                self.health += eff_stats.heal;
                self.mana += eff_stats.regen;
                armor += eff_stats.armor;
            }
        }
        armor
    }

    fn do_turn(&mut self, turn_spell: &Spell, difficulty: Difficulty) -> TurnResult {
        if difficulty == HardMode {
            self.health -= 1;
            if self.health == 0 {
                return Lost;
            }
        }

        let turn_spell_stats = spell_stats(turn_spell);
        if turn_spell_stats.cost > self.mana {
            eprintln!(
                "attempted to cast {:?} with insufficient mana {:?} (Need {})",
                turn_spell, self.mana, turn_spell_stats.cost
            );
            return Invalid;
        }
        if *self.effects.get(turn_spell).unwrap_or(&0) > 1 {
            eprintln!("attempted to cast {turn_spell:?} on cooldown");
            return Invalid;
        }
        self.mana -= turn_spell_stats.cost;

        self.past_spells.push(turn_spell.clone());
        self.total_mana_spent += turn_spell_stats.cost;

        self.apply_turn_effects();

        if turn_spell_stats.turns == 1 {
            self.boss_health -= turn_spell_stats.damage;
            self.health += turn_spell_stats.heal;
        } else {
            *self.effects.entry(turn_spell.clone()).or_default() += turn_spell_stats.turns;
        }
        if self.boss_health <= 0 {
            return Won;
        }

        let armor = self.apply_turn_effects();
        if self.boss_health <= 0 {
            return Won;
        }
        let boss_damage = i32::max(1, self.boss_damage - armor);
        self.health -= boss_damage;
        if self.health <= 0 {
            return Lost;
        }

        InProgress
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut fight = FightProgress {
            health: 10,
            mana: 250,
            boss_health: 13,
            boss_damage: 8,

            past_spells: Vec::new(),
            total_mana_spent: 0,
            effects: HashMap::new(),
        };

        let turn_result = fight.do_turn(&Poison, EasyMode);

        assert_eq!(turn_result, InProgress);
        assert_eq!(
            fight,
            FightProgress {
                health: 2,
                mana: 77,
                boss_health: 10,
                boss_damage: 8,
                past_spells: Vec::from([Poison]),
                effects: HashMap::from([(Poison, 5)]),
                total_mana_spent: 173,
            }
        );

        assert_eq!(fight.do_turn(&MagicMissile, EasyMode), Won);
        assert_eq!(fight.total_mana_spent, 173 + 53);
    }

    #[test]
    fn example1_solution() {
        let expected = [Poison, MagicMissile]
            .iter()
            .map(|s| spell_stats(s).cost)
            .sum::<i32>();
        assert_eq!(most_efficient_fight(10, 250, 13, 8, EasyMode), expected)
    }

    #[test]
    fn example2() {
        let mut fight = FightProgress {
            health: 10,
            mana: 250,
            boss_health: 14,
            boss_damage: 8,

            past_spells: Vec::new(),
            total_mana_spent: 0,
            effects: HashMap::new(),
        };

        let turn_result = fight.do_turn(&Recharge, EasyMode);

        assert_eq!(turn_result, InProgress);
        assert_eq!(
            fight,
            FightProgress {
                health: 2,
                mana: 122,
                boss_health: 14,
                boss_damage: 8,
                past_spells: Vec::from([Recharge]),
                effects: HashMap::from([(Recharge, 4)]),
                total_mana_spent: 229,
            }
        );

        let turn_result = fight.do_turn(&Shield, EasyMode);

        assert_eq!(turn_result, InProgress);
        assert_eq!(
            fight,
            FightProgress {
                health: 1,
                mana: 211,
                boss_health: 14,
                boss_damage: 8,
                past_spells: Vec::from([Recharge, Shield]),
                effects: HashMap::from([(Recharge, 2), (Shield, 5)]),
                total_mana_spent: 229 + 113,
            }
        );

        let turn_result = fight.do_turn(&Drain, EasyMode);

        assert_eq!(turn_result, InProgress);
        assert_eq!(
            fight,
            FightProgress {
                health: 2,
                mana: 340,
                boss_health: 12,
                boss_damage: 8,
                past_spells: Vec::from([Recharge, Shield, Drain]),
                effects: HashMap::from([(Recharge, 0), (Shield, 3)]),
                total_mana_spent: 229 + 113 + 73,
            }
        );

        let turn_result = fight.do_turn(&Poison, EasyMode);

        assert_eq!(turn_result, InProgress);
        assert_eq!(
            fight,
            FightProgress {
                health: 1,
                mana: 167,
                boss_health: 9,
                boss_damage: 8,
                past_spells: Vec::from([Recharge, Shield, Drain, Poison]),
                effects: HashMap::from([(Recharge, 0), (Shield, 1), (Poison, 5)]),
                total_mana_spent: 229 + 113 + 73 + 173,
            }
        );

        let turn_result = fight.do_turn(&MagicMissile, EasyMode);

        assert_eq!(turn_result, Won);
        assert_eq!(fight.total_mana_spent, 229 + 113 + 73 + 173 + 53);
    }
}
