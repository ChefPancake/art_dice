use crate::dice::*;

fn side_of_n_symbols(n: usize, symbol: &DieSymbol) -> DieSide {
    let vec = 
        (0..n)
        .map(|_| symbol.clone())
        .collect::<Vec<_>>();
    DieSide { symbols: vec }
}

fn n_sided_die(n: usize) -> Die {
    let pip = DieSymbol::new("Pip").unwrap();
    let sides = 
        (1..(n+1))
        .map(|i| side_of_n_symbols(i, &pip))
        .collect();
    Die { sides }
}

/// Creates an instance of the symbol used by the standard dice
pub fn pip() -> DieSymbol {
    DieSymbol::new("Pip").unwrap()
}

/// Creates a standard 4 sided die
pub fn d4() -> Die {
    n_sided_die(4)
}

/// Creates a standard 6 sided die
pub fn d6() -> Die {
    n_sided_die(6)
}

/// Creates a standard 8 sided die
pub fn d8() -> Die {
    n_sided_die(8)
}

/// Creates a standard 10 sided die
pub fn d10() -> Die {
    n_sided_die(10)
}

/// Creates a standard 12 sided die
pub fn d12() -> Die {
    n_sided_die(12)
}

/// Creates a standard 20 sided die
pub fn d20() -> Die {
    n_sided_die(20)
}
