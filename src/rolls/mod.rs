use itertools::Itertools;
use itertools::Product;
use std::collections::HashMap;
use crate::dice::*;
use crate::item_counter::ItemCounter;

#[cfg(test)]
mod tests;

#[derive(Eq, PartialEq, Clone, Hash)]
struct RollResultPossibility {
    symbols: ItemCounter<DieSymbol>
}

impl RollResultPossibility {
    pub fn new() -> RollResultPossibility {
        RollResultPossibility {
            symbols: ItemCounter::new()
        }
    }

    pub fn add_symbols(&self, symbols: &[DieSymbol]) -> RollResultPossibility {
        let mut symbol_count = self.clone().symbols;
        for symbol in symbols {
            symbol_count.add(symbol);
        }
        RollResultPossibility { symbols: symbol_count }
    }

    pub fn total_count(&self) -> usize {
        self.symbols.total_count()
    }
}

/// Represents the type of targets for a given roll
#[derive(Copy, Clone, PartialEq, Eq)]
enum RollTargetTypes {
    Exactly,
    AtLeast,
    AtMost
}

#[derive(Copy, Clone, PartialEq, Eq)]
/// Represents the target for a given roll
pub struct RollTarget<'a> {
    target_type: RollTargetTypes,
    amount: usize,
    symbols: &'a [DieSymbol]
}

impl<'a> RollTarget<'a> {
    /// Returns an instance of a target that is exactly N of provided symbols
    pub fn exactly_n_of(n: usize, symbols: &'a [DieSymbol]) -> RollTarget {
        RollTarget {
            target_type: RollTargetTypes::Exactly,
            amount: n,
            symbols
        }
    }
    /// Returns an instance of a target that is at least N of provided symbols
    pub fn at_least_n_of(n: usize, symbols: &'a [DieSymbol]) -> RollTarget {
        RollTarget {
            target_type: RollTargetTypes::AtLeast,
            amount: n,
            symbols
        }
    }
    /// Returns an instance of a target that is at most N of provided symbols
    pub fn at_most_n_of(n: usize, symbols: &'a [DieSymbol]) -> RollTarget {
        RollTarget {
            target_type: RollTargetTypes::AtMost,
            amount: n,
            symbols
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum RollCollectionTypes {
    CollectAll,
    TakeHighestN(usize),
    TakeLowestN(usize),
    RemoveHighestN(usize),
    RemoveLowestN(usize)
}

#[derive(Copy, Clone, PartialEq, Eq)]
/// Defines the policy used to collect dice after a roll based on [`DieSymbol`](crate::dice::DieSymbol) occurrences
pub struct RollCollectionPolicy<'a> {
    coll_type: RollCollectionTypes,
    symbols: &'a [DieSymbol]
}

impl<'a> RollCollectionPolicy<'a> {
    /// Policy for collecting all dice in the roll
    pub fn collect_all(symbols: &'a [DieSymbol]) -> RollCollectionPolicy {
        RollCollectionPolicy {
            coll_type: RollCollectionTypes::CollectAll,
            symbols
        }
    }

    /// Policy for taking the highest N dice, ordering by number of matching symbols
    pub fn take_highest_n_of(n:usize, symbols: &'a [DieSymbol]) -> RollCollectionPolicy {
        RollCollectionPolicy {
            coll_type: RollCollectionTypes::TakeHighestN(n),
            symbols
        }
    }

    /// Policy for taking the lowest N dice, ordering by number of matching symbols
    pub fn take_lowest_n_of(n:usize, symbols: &'a [DieSymbol]) -> RollCollectionPolicy {
        RollCollectionPolicy {
            coll_type: RollCollectionTypes::TakeLowestN(n),
            symbols
        }
    }
    
    /// Policy for removing the highest N dice and collecting the rest, ordering by number of matching symbols
    pub fn remove_highest_n_of(n:usize, symbols: &'a [DieSymbol]) -> RollCollectionPolicy {
        RollCollectionPolicy {
            coll_type: RollCollectionTypes::RemoveHighestN(n),
            symbols
        }
    }
    
    /// Policy for removing the lowest N dice and collecting the rest, ordering by number of matching symbols
    pub fn remove_lowest_n_of(n:usize, symbols: &'a [DieSymbol]) -> RollCollectionPolicy {
        RollCollectionPolicy {
            coll_type: RollCollectionTypes::RemoveLowestN(n),
            symbols
        }
    }
}

/// Tracks the probabilities of a roll of one or more dice
pub struct RollProbabilities {
    occurrences: HashMap<RollResultPossibility, usize>,
    total: usize
}

impl RollProbabilities {
    fn collect_symbols(roll: &[&DieSide], policy: &RollCollectionPolicy) -> Vec<DieSymbol> {
        let mut filtered_sides: Vec<Vec<DieSymbol>> =
            roll.iter()
            .map(|x| 
                x.symbols().iter()
                .filter(|y| policy.symbols.contains(y))
                .cloned().collect())
            .collect();
        filtered_sides.sort_by(|x,y| x.len().cmp(&y.len()));
        filtered_sides.reverse();
        let sides_len = filtered_sides.len();
        match policy.coll_type {
            RollCollectionTypes::CollectAll => 
                filtered_sides.iter()
                .flatten().cloned().collect(),
            RollCollectionTypes::TakeHighestN(n) => 
                filtered_sides.iter().take(n)
                .flatten().cloned().collect(),
            RollCollectionTypes::TakeLowestN(n) => 
                filtered_sides.iter().skip(sides_len - n)
                .flatten().cloned().collect(),
            RollCollectionTypes::RemoveHighestN(n) =>
                filtered_sides.iter().skip(n)
                .flatten().cloned().collect(),
            RollCollectionTypes::RemoveLowestN(n) =>
                filtered_sides.iter().take(sides_len - n)
                .flatten().cloned().collect()
        }
    }

    /// Creates a new instance of [`RollProbabilities`](crate::rolls::RollProbabilities) based on the provided collection of [`Dice`](crate::dice::Die). 
    /// Die sides are collected based on the provided [`RollCollectionPolicy`](crate::rolls::RollCollectionPolicy). 
    /// Returns `Err` if provided slice contains no elements, else returns `Ok`.
    /// 
    /// # Example
    /// ```rust
    /// # use std::error::Error;
    /// # use art_dice::dice::{DieSymbol, DieSide, Die};
    /// # use art_dice::dice::standard;
    /// # use art_dice::rolls::{RollTarget, RollProbabilities, RollCollectionPolicy};
    /// # fn main() -> Result<(), String> {
    /// let symbols = vec![ standard::pip() ] ;
    /// let policy = RollCollectionPolicy::collect_all(&symbols);
    /// let dice = vec![standard::d4(), standard::d4()];
    /// 
    /// let two_d4s = RollProbabilities::new(&dice, policy)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(dice: &[Die], policy: &RollCollectionPolicy) -> Result<RollProbabilities, String> {
        if dice.len() == 0 {
            return Err("must include at least one die".to_string());
        }
        let mut occur = HashMap::new();
        for roll in dice.into_iter()
                .map(|x| x.sides())
                .multi_cartesian_product() {
            let collected = Self::collect_symbols(&roll, policy);
            let new_poss = 
                RollResultPossibility::new()
                .add_symbols(&collected);
            if occur.contains_key(&new_poss) {
                occur.get_mut(&new_poss).map(|x| *x += 1);
            } else {
                occur.insert(new_poss, 1);
            }
        }
        let total = occur.values().sum();
        Ok(RollProbabilities {
            occurrences: occur,
            total: total
        })
    }

    /// Retrieves the probability of the roll achieving the [`RollTarget`](crate::rolls::RollTarget). 
    /// Note that the roll's [`DieSymbols`](crate::dice::DieSymbol) will have been filtered down based
    /// on the [`RollCollectionPolicy`](crate::rolls::RollCollectionPolicy) used to generate the probability
    /// 
    /// # Examples
    /// ```rust
    /// # use std::error::Error;
    /// # use art_dice::dice::{DieSymbol, DieSide, Die};
    /// # use art_dice::dice::standard;
    /// # use art_dice::rolls::{RollTarget, RollProbabilities, RollCollectionPolicy};
    /// # fn main() -> Result<(), String> {
    /// let dice = vec![standard::d4(), standard::d4()];
    /// let symbols = vec![ standard::pip() ];
    /// let policy = RollCollectionPolicy::collect_all(&symbols);
    /// let two_d4s = RollProbabilities::new(&dice, policy)?;
    /// 
    /// let exactly_3 = two_d4s.get_odds(RollTarget::exactly_n_of(3, &symbols));
    /// let at_least_6 = two_d4s.get_odds(RollTarget::at_least_n_of(6, &symbols));
    /// let at_most_5 = two_d4s.get_odds(RollTarget::at_most_n_of(5, &symbols));
    /// 
    /// assert_eq!(exactly_3, 0.125);
    /// assert_eq!(at_least_6, 0.375);
    /// assert_eq!(at_most_5, 0.625);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_odds(&self, target: RollTarget) -> f64 {
        if self.total == 0 {
            return 0.0;
        }

        let mut total_occurrences = 0;
        for poss in self.occurrences.keys() {
            let mut count: usize = 0;
            for symbol in target.symbols {
                count += poss.symbols.get_count(&symbol);
            }
            let cond = match target.target_type {
                RollTargetTypes::Exactly => count == target.amount,
                RollTargetTypes::AtLeast => count >= target.amount,
                RollTargetTypes::AtMost => count <= target.amount
            };
            if cond {
                total_occurrences += self.occurrences[poss];
            }
        }
        return (total_occurrences as f64) / (self.total as f64);
    }

    pub fn roll_against(&self, other: &Self) -> RollCompare {
        let (mut wins, mut ties, mut losses) = (0,0,0);

        for (this_poss, other_poss) in self.occurrences.iter().cartesian_product(other.occurrences.iter()) {
            let this_val = this_poss.0.total_count();
            let other_val = other_poss.0.total_count();
            let occurrences = this_poss.1 * other_poss.1;

            match (this_val > other_val, this_val == other_val) {
                (_, true) => ties += occurrences,
                (true, false) => wins += occurrences,
                (false, false) => losses += occurrences
            }
        }
        
        return RollCompare::new(wins, ties, losses);
    }
}

pub struct RollCompare {
    wins: usize,
    ties: usize,
    losses: usize,
    total: usize
}

impl RollCompare {
    pub fn new(wins: usize, ties: usize, losses: usize) -> RollCompare {
        let total = wins + ties + losses;
        RollCompare {
            wins,
            ties,
            losses,
            total
        }
    }

    pub fn win_odds(&self) -> f64 {
        if self.total == 0 {
            return 0.0
        }
        (self.wins as f64) / (self.total as f64)
    }

    pub fn tie_odds(&self) -> f64 {
        if self.total == 0 {
            return 0.0
        }
        (self.ties as f64) / (self.total as f64)
    }

    pub fn loss_odds(&self) -> f64 {
        if self.total == 0 {
            return 0.0
        }
        (self.losses as f64) / (self.total as f64)
    }
}