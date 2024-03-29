use crate::dice::*;
use crate::dice::standard::*;

fn assert_dice_sides(sides: &[DieSide]) {
    for i in 0..(sides.len()) {
        assert_eq!(sides[i].symbols().len(), i+1);
    }
}

#[test]
fn four_sided_die() {
    let die = d4();
    let sides = die.sides();
    let symbols = die.unique_symbols();
    assert_eq!(symbols.len(), 1);
    assert_eq!(sides.len(), 4);
    assert_dice_sides(sides);
}

#[test]
fn twelve_sided_die() {
    let die = d12();
    let sides = die.sides();
    let symbols = die.unique_symbols();
    assert_eq!(symbols.len(), 1);
    assert_eq!(sides.len(), 12);
    assert_dice_sides(sides);
}

#[test]
fn six_sided_die_average() {
    let die = d6();
    let symbol = die.unique_symbols().first().unwrap().clone();
    let average = die.average_of(&symbol);
    assert_eq!(average, 3.5);
}

#[test]
fn ten_sided_die_average() {
    let die = d10();
    let symbol = die.unique_symbols().first().unwrap().clone();
    let average = die.average_of(&symbol);
    assert_eq!(average, 5.5);
}