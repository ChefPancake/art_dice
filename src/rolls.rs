use itertools::Itertools;
use std::collections::HashMap;
use crate::dice::*;
use crate::item_counter::ItemCounter;

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
}

/// Represents the type of targets for a given roll
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RollTargets {
    /// The target roll is exactly N symbols
    Exactly(usize),
    /// the target roll is at least N symbols
    AtLeast(usize),
    /// the target roll is at most N symbols
    AtMost(usize)
}

/// Tracks the probabilities of a roll of one or more dice
pub struct RollProbabilities {
    occurrences: HashMap<RollResultPossibility, usize>,
    total: usize
}

impl RollProbabilities {
    /// Creates a new instance of [`RollProbabilities`](crate::rolls::RollProbabilities) containing a collection of [`Dice`](crate::dice::Die). Returns Err if provided slice contains no elements, else returns Ok.
    /// 
    /// # Example
    /// ```rust
    /// # use std::error::Error;
    /// # use art_dice::dice::{DieSymbol, DieSide, Die};
    /// # use art_dice::dice::standard;
    /// # use art_dice::rolls::{RollTargets, RollProbabilities};
    /// # fn main() -> Result<(), String> {
    /// let dice = vec![standard::d4(), standard::d4()];
    /// 
    /// let two_d4s = RollProbabilities::new(&dice)?;
    /// # Ok(())
    /// # }
    /// ```
    /// 
    pub fn new(dice: &[Die]) -> Result<RollProbabilities, String> {
        if dice.len() == 0 {
            return Err("must include at least one die".to_string());
        }
        let mut occur = HashMap::new();
        for roll in dice.into_iter()
                .map(|x| x.sides())
                .multi_cartesian_product() {
            let symbols: Vec<DieSymbol> = roll.iter().map(|x| x.symbols()).flatten().cloned().collect();
            let new_poss = 
                RollResultPossibility::new()
                .add_symbols(&symbols);
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

    /// Retrieves the probability of the roll achieving the [`RollTarget`](crate::rolls::RollTargets) counting all of the provided symbols
    /// 
    /// # Examples
    /// ```rust
    /// # use std::error::Error;
    /// # use art_dice::dice::{DieSymbol, DieSide, Die};
    /// # use art_dice::dice::standard;
    /// # use art_dice::rolls::{RollTargets, RollProbabilities};
    /// # fn main() -> Result<(), String> {
    /// let dice = vec![standard::d4(), standard::d4()];
    /// let two_d4s = RollProbabilities::new(&dice)?;
    /// 
    /// let symbols = vec![ standard::pip() ];
    /// 
    /// let exactly_3 = two_d4s.get_odds(RollTargets::Exactly(3), &symbols);
    /// let at_least_6 = two_d4s.get_odds(RollTargets::AtLeast(6), &symbols);
    /// let at_most_5 = two_d4s.get_odds(RollTargets::AtMost(5), &symbols);
    /// 
    /// assert_eq!(exactly_3, 0.125);
    /// assert_eq!(at_least_6, 0.375);
    /// assert_eq!(at_most_5, 0.625);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_odds(&self, target: RollTargets, symbols: &[DieSymbol]) -> f64 {
        if self.total == 0 {
            return 0.0;
        }

        let mut total_occurrences = 0;
        for poss in self.occurrences.keys() {
            let mut count: usize = 0;
            for symbol in symbols {
                count += poss.symbols.get_count(&symbol);
            }
            let cond = match target {
                RollTargets::Exactly(x) => count == x,
                RollTargets::AtLeast(x) => count >= x,
                RollTargets::AtMost(x) => count <= x
            };
            if cond {
                total_occurrences += self.occurrences[poss];
            }
        }
        return (total_occurrences as f64) / (self.total as f64);
    }
}


#[cfg(test)]
mod roll_tests {
    use crate::dice::standard::*;
    use crate::rolls::*;

    fn test_results_exactly(results: &RollProbabilities, symbols: &[DieSymbol], count: usize, expected: f64) {
        let target = RollTargets::Exactly(count);
        let odds = results.get_odds(target, &symbols);
        assert_eq!(odds, expected);
    }

    #[test]
    fn one_d4() {
        let results = RollProbabilities::new( &vec![d4()] ).unwrap();
        assert_eq!(results.total, 4);
        
        let symbols = d4().unique_symbols();
        
        test_results_exactly(&results, &symbols, 1, 0.25);
        test_results_exactly(&results, &symbols, 2, 0.25);
        test_results_exactly(&results, &symbols, 3, 0.25);
        test_results_exactly(&results, &symbols, 4, 0.25);
    }

    #[test]
    fn two_d4s() {
        let results = RollProbabilities::new(&vec![ d4(), d4() ]).unwrap();
        assert_eq!(results.total, 16);

        let symbols = d4().unique_symbols();
        
        test_results_exactly(&results, &symbols, 1, 0.0);
        test_results_exactly(&results, &symbols, 2, 0.0625);
        test_results_exactly(&results, &symbols, 3, 0.125);
        test_results_exactly(&results, &symbols, 4, 0.1875);
        test_results_exactly(&results, &symbols, 5, 0.25);
        test_results_exactly(&results, &symbols, 6, 0.1875);
        test_results_exactly(&results, &symbols, 7, 0.125);
        test_results_exactly(&results, &symbols, 8, 0.0625);
    }

    #[test]
    fn d4_and_d8() {
        let results = RollProbabilities::new(&vec![ d4(), d8() ]).unwrap();
        assert_eq!(results.total, 32);

        let symbols = d4().unique_symbols();
        
        test_results_exactly(&results, &symbols, 1, 0.0);
        test_results_exactly(&results, &symbols, 2, 0.03125);
        test_results_exactly(&results, &symbols, 3, 0.0625);
        test_results_exactly(&results, &symbols, 4, 0.09375);
        test_results_exactly(&results, &symbols, 5, 0.125);
        test_results_exactly(&results, &symbols, 6, 0.125);
        test_results_exactly(&results, &symbols, 7, 0.125);
        test_results_exactly(&results, &symbols, 8, 0.125);
        test_results_exactly(&results, &symbols, 9, 0.125);
        test_results_exactly(&results, &symbols, 10, 0.09375);
        test_results_exactly(&results, &symbols, 11, 0.0625);
        test_results_exactly(&results, &symbols, 12, 0.03125);
    }

    #[test]
    fn three_d4s() {
        let results = RollProbabilities::new(&vec![ d4(), d4(), d4() ] ).unwrap();
            
        let symbols = d4().unique_symbols();
        assert_eq!(results.total, 4*4*4);
        test_results_exactly(&results, &symbols, 7, 0.1875);
    }

    #[test]
    fn four_through_ten() {
        let results = RollProbabilities::new(&vec![ d4(), d6(), d8(), d10() ] ).unwrap();
    
        assert_eq!(results.total, 4*6*8*10);
    }
}