use std::collections::HashSet;

pub mod basic;

#[derive(Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
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