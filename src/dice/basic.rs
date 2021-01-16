use super::*;

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
        .map(|i| side_of_n_symbols(i, &pip))
        .collect();
    Die { sides }
}

pub fn four_sided() -> Die {
    n_sided_die(4)
}

pub fn six_sided() -> Die {
    n_sided_die(6)
}

pub fn eight_sided() -> Die {
    n_sided_die(8)
}

pub fn ten_sided() -> Die {
    n_sided_die(10)
}

pub fn twelve_sided() -> Die {
    n_sided_die(12)
}

pub fn twenty_sided() -> Die {
    n_sided_die(20)
}
