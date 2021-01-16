use crate::dice::*;

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

pub fn d4() -> Die {
    n_sided_die(4)
}

pub fn d6() -> Die {
    n_sided_die(6)
}

pub fn d8() -> Die {
    n_sided_die(8)
}

pub fn d10() -> Die {
    n_sided_die(10)
}

pub fn d12() -> Die {
    n_sided_die(12)
}

pub fn d20() -> Die {
    n_sided_die(20)
}
