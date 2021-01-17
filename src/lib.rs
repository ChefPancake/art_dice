pub mod dice;
pub mod rolls;
pub mod item_counter;
pub mod multi_cart;

#[cfg(test)]
mod dice_tests {
    use super::dice::*;
    use super::dice::standard::*;

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
}