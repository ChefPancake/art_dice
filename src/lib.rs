use std::collections::HashSet;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct DiceSymbol {
    name: String
}
impl DiceSymbol {
    pub fn new(val: &str) -> Result<DiceSymbol, String> {
        let trimmed = val.trim();
        match trimmed.len() {
            0 => Err("Value cannot be empty".to_string()),
            _ => Ok(DiceSymbol { name: trimmed.to_string() })
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Clone)]
pub struct DiceSide {
    symbols: Vec<DiceSymbol>
}
impl DiceSide {
    pub fn new(symbols: Vec<DiceSymbol>) -> DiceSide {
        DiceSide { symbols }
    }

    pub fn symbols(&self) -> &[DiceSymbol] {
        &self.symbols.as_slice()
    }

    pub fn unique_symbols(&self) -> HashSet<DiceSymbol> {
        self.symbols
        .iter()
        .cloned()
        .collect::<HashSet<DiceSymbol>>()
    }
}

pub struct Die {
    sides: Vec<DiceSide>
}
impl Die {
    pub fn new(sides: Vec<DiceSide>) -> Result<Die, String> {
        match sides.len() {
            0 => Err("Die must have at least 2 sides".to_string()),
            1 => Err("Die must have at least 2 sides".to_string()),
            _ => Ok(Die { sides })
        }
    }

    pub fn sides(&self) -> &[DiceSide] {
        self.sides.as_slice()
    }

    pub fn unique_symbols(&self) -> HashSet<DiceSymbol> {
        self.sides.iter().map(|s| s.unique_symbols()).flatten().collect()
    }
}

pub struct BasicDice{}
impl BasicDice {
    fn side_of_n_symbols(n: usize, symbol: &DiceSymbol) -> DiceSide {
        let vec = 
            (0..n)
            .map(|_| symbol.clone())
            .collect::<Vec<_>>();
        DiceSide { symbols: vec }
    }

    fn n_sided_die(n: usize) -> Die {
        let pip = DiceSymbol::new("Pip").unwrap();
        let sides = 
            (1..(n+1))
            .map(|i| BasicDice::side_of_n_symbols(i, &pip))
            .collect();
        Die { sides }
    }

    pub fn four_sided() -> Die {
        BasicDice::n_sided_die(4)
    }

    pub fn six_sided() -> Die {
        BasicDice::n_sided_die(6)
    }

    pub fn eight_sided() -> Die {
        BasicDice::n_sided_die(8)
    }

    pub fn ten_sided() -> Die {
        BasicDice::n_sided_die(10)
    }

    pub fn twelve_sided() -> Die {
        BasicDice::n_sided_die(12)
    }

    pub fn twenty_sided() -> Die {
        BasicDice::n_sided_die(20)
    }
}

#[cfg(test)]
mod dice_tests {
    use super::*;

    fn assert_dice_sides(sides: &[DiceSide]) {
        for i in 0..(sides.len()) {
            assert_eq!(sides[i].symbols().len(), i+1);
        }
    }

    #[test]
    fn four_sided_die() {
        let die = BasicDice::four_sided();
        let sides = die.sides();
        let symbols = die.unique_symbols();
        assert_eq!(symbols.len(), 1);
        assert_eq!(sides.len(), 4);
        assert_dice_sides(sides);
    }

    #[test]
    fn twelve_sided_die() {
        let die = BasicDice::twelve_sided();
        let sides = die.sides();
        let symbols = die.unique_symbols();
        assert_eq!(symbols.len(), 1);
        assert_eq!(sides.len(), 12);
        assert_dice_sides(sides);
    }
}
