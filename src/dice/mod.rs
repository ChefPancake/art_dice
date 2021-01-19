pub mod standard;

#[derive(Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
/// Represents an instance of a symbol found on a die
pub struct DieSymbol {
    name: String
}
impl DieSymbol {
    /// Creates a new [`DieSymbol`](crate::dice::DieSymbol). Returns an [`Err`](std::Err) if input is empty or only whitespace, otherwise returns [`Ok`](std::Ok)
    /// 
    /// # Example
    /// ```rust
    /// # use std::error::Error;
    /// # use art_dice::dice::DieSymbol;
    /// # fn main() -> Result<(), String> {
    /// let symbol = DieSymbol::new("Pip")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(val: &str) -> Result<DieSymbol, String> {
        let trimmed = val.trim();
        match trimmed.len() {
            0 => Err("Value cannot be empty".to_string()),
            _ => Ok(DieSymbol { name: trimmed.to_string() })
        }
    }

    /// The underlying name value of the [`DieSymbol`](crate::dice::DieSymbol)
    /// 
    /// # Example
    /// ```rust
    /// # use std::error::Error;
    /// # use art_dice::dice::DieSymbol;
    /// # fn main() -> Result<(), String> {
    /// let symbol = DieSymbol::new("Pip")?;
    /// 
    /// assert_eq!("Pip".to_string(), *symbol.name());
    /// # Ok(())
    /// # }
    /// ```
    pub fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Clone)]
/// Represents a side of a die and contains a collection of [`DieSymbols`](crate::dice::DieSymbol)
pub struct DieSide {
    symbols: Vec<DieSymbol>
}
impl DieSide {
    /// Creates a new [`DieSide`](crate::dice::DieSide) with a collection of [`DieSymbols`](crate::dice::DieSymbol). Input collection may be empty, representing a blank side
    /// 
    /// # Example
    /// ```rust
    /// # use std::error::Error;
    /// # use art_dice::dice::{DieSymbol, DieSide};
    /// # fn main() -> Result<(), String> {
    /// let symbols = vec![DieSymbol::new("Pip")?];
    /// 
    /// let side = DieSide::new(symbols);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(symbols: Vec<DieSymbol>) -> DieSide {
        DieSide { symbols }
    }

    /// Returns a slice of all [`DieSymbols`](crate::dice::DieSymbol) on the [`DieSide`](crate::dice::DieSide)
    /// 
    /// # Example
    /// ```rust
    /// # use std::error::Error;
    /// # use art_dice::dice::{DieSymbol, DieSide};
    /// # fn main() -> Result<(), String> {
    /// let pip = DieSymbol::new("Pip")?;
    /// let symbols = vec![ pip.clone() ];
    /// let side = DieSide::new(symbols);
    /// 
    /// let side_symbols = side.symbols();
    /// 
    /// assert_eq!(side_symbols.iter().next().unwrap().name(), pip.name());
    /// # Ok(())
    /// # }
    /// ```
    pub fn symbols(&self) -> &[DieSymbol] {
        &self.symbols.as_slice()
    }
}

#[derive(Clone)]
/// Represents a die containing a collection of all its [`DieSides`](crate::dice::DieSide)
pub struct Die {
    sides: Vec<DieSide>
}
impl Die {
    /// Creates a new instance of a [`Die`](crate::Dice::Die) with its [`DieSides`](crate::dice::DieSide). Returns `Err` if input sides has fewer than 2 sides (a coin), else returns `Ok`
    /// 
    /// # Example
    /// ```rust
    /// # use std::error::Error;
    /// # use art_dice::dice::{DieSymbol, DieSide, Die};
    /// # fn main() -> Result<(), String> {
    /// let heads = vec![ DieSymbol::new("Heads")? ];
    /// let heads_side = DieSide::new(heads);
    /// let tails = vec![ DieSymbol::new("Tails")? ];
    /// let tails_side = DieSide::new(tails);
    /// let sides = vec![ heads_side, tails_side ];
    /// 
    /// let coin = Die::new(sides)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(sides: Vec<DieSide>) -> Result<Die, String> {
        match sides.len() {
            0 => Err("Die must have at least 2 sides".to_string()),
            1 => Err("Die must have at least 2 sides".to_string()),
            _ => Ok(Die { sides })
        }
    }

    /// Returns a slice of all [`DieSides`](crate::dice::DieSide) in the [`Die`](crate::Dice::Die)
    /// 
    /// # Example
    /// ```rust
    /// # use std::error::Error;
    /// # use art_dice::dice::{DieSymbol, DieSide, Die};
    /// # fn main() -> Result<(), String> {
    /// # let heads = vec![ DieSymbol::new("Heads")? ];
    /// # let heads_side = DieSide::new(heads);
    /// # let tails = vec![ DieSymbol::new("Tails")? ];
    /// # let tails_side = DieSide::new(tails);
    /// let sides = vec![ heads_side, tails_side ];
    /// let coin = Die::new(sides)?;
    /// 
    /// let coin_sides = coin.sides();
    /// 
    /// assert_eq!(coin_sides.len(), 2);
    /// # Ok(())
    /// # }
    /// ```
    pub fn sides(&self) -> &[DieSide] {
        self.sides.as_slice()
    }

    /// Returns a distinct collection of all [`DieSymbols`](crate::dice::DieSymbol) represented on all [`DieSides`](crate::dice::DieSide) of the [`Die`](crate::Dice::Die) as a `Vec`
    /// 
    /// # Example
    /// ```rust
    /// # use std::error::Error;
    /// # use art_dice::dice::{DieSymbol, DieSide, Die};
    /// # fn main() -> Result<(), String> {
    /// # let heads = vec![ DieSymbol::new("Heads")? ];
    /// # let heads_side = DieSide::new(heads);
    /// # let tails = vec![ DieSymbol::new("Tails")? ];
    /// # let tails_side = DieSide::new(tails);
    /// let sides = vec![ heads_side, tails_side ];
    /// let coin = Die::new(sides)?;
    /// 
    /// let unique_symbols = coin.unique_symbols();
    /// 
    /// assert_eq!(unique_symbols.len(), 2);
    /// # Ok(())
    /// # }
    /// ```
    pub fn unique_symbols(&self) -> Vec<DieSymbol> {
        let mut unique = Vec::new();
        for symbol in
                self.sides.iter()
                .map(|s| s.symbols())
                .flatten()
                .cloned()
                .collect::<Vec<DieSymbol>>() {
            if !unique.contains(&symbol) {
                unique.push(symbol);
            }
        }
        unique
    }
}