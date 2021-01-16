use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::dice::*;
use crate::item_counter::ItemCounter;

pub enum DiceRollTargets {
    Exactly(usize),
    AtLeast(usize),
    AtMost(usize)
}

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct RollResultPossibility {
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
        result_map.insert(poss, 0);
        RollResults { occurrences: result_map, total: 0 }
    }

    pub fn add_die(&self, die: Die) -> RollResults {
        let mut new_results = HashMap::new();
        for side in die.sides() {
            for result in self.occurrences.keys() {
                let new_poss = result.add_symbols(side.symbols());
                if new_results.contains_key(&new_poss) {
                    new_results.get_mut(&new_poss).map(|x| *x += 1);
                } else {
                    new_results.insert(new_poss, 1);
                }
            }
        }
        let total = new_results.values().sum();
        RollResults {
            occurrences: new_results,
            total
        }
    }
}


#[cfg(test)]
mod roll_tests {
    use crate::dice::basic::*;
    use crate::rolls::*;

    //TODO: replace with proper tests
    #[test]
    fn one_d4() {
        let d4 = d4();
        let results = RollResults::new();
        let results = results.add_die(d4);
        assert_eq!(results.total, 4);

        let unwrapped = results.occurrences;
        let mut values: Vec<usize> = unwrapped.values().map(|x| *x).collect();
        values.sort();
        assert_eq!(values.len(), 4);

        let mut value_iter = values.iter();
        assert_eq!(value_iter.next(), Some(&1));
        assert_eq!(value_iter.next(), Some(&1));
        assert_eq!(value_iter.next(), Some(&1));
        assert_eq!(value_iter.next(), Some(&1));
    }

    #[test]
    fn two_d4s() {
        let d4_1 = d4();
        let d4_2 = d4();
        let results = RollResults::new();
        let results = results.add_die(d4_1);
        let results = results.add_die(d4_2);
        assert_eq!(results.total, 16);

        let unwrapped = results.occurrences;
        let mut values: Vec<usize> = unwrapped.values().map(|x| *x).collect();
        values.sort();
        assert_eq!(values.len(), 7);

        let mut value_iter = values.iter();
        assert_eq!(value_iter.next(), Some(&1));
        assert_eq!(value_iter.next(), Some(&1));
        assert_eq!(value_iter.next(), Some(&2));
        assert_eq!(value_iter.next(), Some(&2));
        assert_eq!(value_iter.next(), Some(&3));
        assert_eq!(value_iter.next(), Some(&3));
        assert_eq!(value_iter.next(), Some(&4));
    }
}