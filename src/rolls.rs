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

/// Tracks the probabilities of a roll of zero or more dice
pub struct RollProbabilities {
    dice: Vec<Die>,
    occurrences: HashMap<RollResultPossibility, usize>,
    total: usize
}

impl RollProbabilities {
    /// Creates a new, empty instance of [`RollProbabilities`](crate::rolls::RollProbabilities)
    /// 
    /// # Example
    /// ```rust
    /// # use std::error::Error;
    /// # use art_dice::dice::{DieSymbol, DieSide, Die};
    /// # use art_dice::rolls::RollProbabilities;
    /// # fn main() -> Result<(), String> {
    /// let results = RollProbabilities::new();
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> RollProbabilities {
        let poss = RollResultPossibility::new();
        let mut result_map = HashMap::new();
        result_map.insert(poss, 1);
        RollProbabilities { 
            dice: Vec::new(),
            occurrences: result_map, 
            total: 1 
        }
    }

    /// Returns a new [`RollProbabilities`](crate::rolls::RollProbabilities) that reflects the current probabilities with the added [`Die`](crate::dice::Die)
    /// 
    /// # Example
    /// ```rust
    /// # use std::error::Error;
    /// # use art_dice::dice::{DieSymbol, DieSide, Die};
    /// # use art_dice::dice::standard;
    /// # use art_dice::rolls::RollProbabilities;
    /// # fn main() -> Result<(), String> {
    /// let prob = RollProbabilities::new();
    /// let d4_die = standard::d4();
    /// let d6_die = standard::d6();
    /// 
    /// let prob_with_d4 = prob.add_die(d4_die);
    /// let prob_with_d4_and_d6 = prob_with_d4.add_die(d6_die);
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_die(&self, die: Die) -> RollProbabilities {
        let mut new_results = HashMap::new();
        for side in die.sides() {
            for result in self.occurrences.keys() {
                let occur = self.occurrences[result];
                let new_poss = result.add_symbols(side.symbols());
                if new_results.contains_key(&new_poss) {
                    new_results.get_mut(&new_poss).map(|x| *x += occur);
                } else {
                    new_results.insert(new_poss, occur);
                }
            }
        }
        let total = new_results.values().sum();
        let mut new_dice = self.dice.clone();
        new_dice.push(die);
        RollProbabilities {
            dice: new_dice,
            occurrences: new_results,
            total: total
        }
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
    /// let two_d4s = 
    ///     RollProbabilities::new()
    ///     .add_die(standard::d4())
    ///     .add_die(standard::d4());
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
    /// 
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
        let d4_1 = d4();
        let results = RollProbabilities::new();
        let results = results.add_die(d4_1);
        assert_eq!(results.total, 4);
        
        let symbols = d4().unique_symbols();
        
        test_results_exactly(&results, &symbols, 1, 0.25);
        test_results_exactly(&results, &symbols, 2, 0.25);
        test_results_exactly(&results, &symbols, 3, 0.25);
        test_results_exactly(&results, &symbols, 4, 0.25);
    }

    #[test]
    fn two_d4s() {
        let d4_1 = d4();
        let d4_2 = d4();
        let results = RollProbabilities::new();
        let results = results.add_die(d4_1);
        let results = results.add_die(d4_2);
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
        let d4_1 = d4();
        let d8_1 = d8();
        let results = RollProbabilities::new();
        let results = results.add_die(d4_1);
        let results = results.add_die(d8_1);
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
        let results = 
            RollProbabilities::new()
            .add_die(d4())
            .add_die(d4())
            .add_die(d4());
        
        let symbols = d4().unique_symbols();
        assert_eq!(results.total, 4*4*4);
        test_results_exactly(&results, &symbols, 7, 0.1875);
    }

    #[test]
    fn all_standard_dice() {
        let results = 
            RollProbabilities::new()
            .add_die(d4())
            .add_die(d6())
            .add_die(d8())
            .add_die(d10())
            .add_die(d12())
            .add_die(d20());
        assert_eq!(results.total, 4*6*8*10*12*20);
    }

    #[test]
    fn no_dice_exactly_zero() {
        let results = RollProbabilities::new();
        let symbols = d4().unique_symbols();
        assert_eq!(results.total, 1);
        test_results_exactly(&results, &symbols, 0, 1.0);

    }

}