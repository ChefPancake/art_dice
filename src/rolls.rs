use std::collections::HashMap;
use crate::dice::*;
use crate::item_counter::ItemCounter;

pub enum DiceRollTargets {
    Exactly(usize),
    AtLeast(usize),
    AtMost(usize)
}

#[derive(Eq, PartialEq, Clone, Hash)]
struct RollResultPossibility {
    symbols: ItemCounter<DiceSymbol>
}

impl RollResultPossibility {
    pub fn new() -> RollResultPossibility {
        RollResultPossibility {
            symbols: ItemCounter::new()
        }
    }

    pub fn add_symbols(&self, symbols: &[DiceSymbol]) -> RollResultPossibility {
        let mut symbol_count = self.clone().symbols;
        for symbol in symbols {
            symbol_count.add(symbol);
        }
        RollResultPossibility { symbols: symbol_count }
    }
}

pub struct RollResults {
    occurrences: HashMap<RollResultPossibility, usize>,
    total: usize
}

impl RollResults {
    pub fn new() -> RollResults {
        let poss = RollResultPossibility::new();
        let mut result_map = HashMap::new();
        result_map.insert(poss, 1);
        RollResults { occurrences: result_map, total: 1 }
    }

    pub fn add_die(&self, die: Die) -> RollResults {
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
        RollResults {
            occurrences: new_results,
            total: total
        }
    }

    pub fn get_odds(&self, target: &DiceRollTargets, symbols: &[DiceSymbol]) -> f64 {
        if self.total == 0 {
            return 0.0;
        }

        let mut total_occurrences = 0;
        for poss in self.occurrences.keys() {
            let mut count: usize = 0;
            for symbol in symbols {
                count += match poss.symbols.get_count(&symbol) {
                    Some(y) => y,
                    None => 0
                };
            }
            let cond = match target {
                DiceRollTargets::Exactly(x) => count == *x,
                DiceRollTargets::AtLeast(x) => count >= *x,
                DiceRollTargets::AtMost(x) => count <= *x
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
    use crate::dice::basic::*;
    use crate::rolls::*;

    fn test_results_exactly(results: &RollResults, symbols: &[DiceSymbol], count: usize, expected: f64) {
        let target = DiceRollTargets::Exactly(count);
        let odds = results.get_odds(&target, &symbols);
        assert_eq!(odds, expected);
    }

    #[test]
    fn one_d4() {
        let d4_1 = d4();
        let results = RollResults::new();
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
        let results = RollResults::new();
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
        let results = RollResults::new();
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
            RollResults::new()
            .add_die(d4())
            .add_die(d4())
            .add_die(d4());
        
        let symbols = d4().unique_symbols();
        assert_eq!(results.total, 4*4*4);
        test_results_exactly(&results, &symbols, 7, 0.1875);
    }

    #[test]
    fn all_basic_dice() {
        let results = 
            RollResults::new()
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
        let results = RollResults::new();
        let symbols = d4().unique_symbols();
        assert_eq!(results.total, 1);
        test_results_exactly(&results, &symbols, 0, 1.0);

    }

}