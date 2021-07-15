use crate::dice::standard::*;
use crate::rolls::*;

fn test_results_exactly(results: &RollProbabilities, symbols: &[DieSymbol], count: usize, expected: f64) {
    let target = RollTarget::exactly_n_of(count, symbols);
    let odds = results.get_odds(&vec![ target ]);
    assert_eq!(odds, expected);
}

#[test]
fn one_d4() {
    let symbols = d4().unique_symbols();
    let policy = RollCollectionPolicy::collect_all(&symbols);
    let results = RollProbabilities::new(&vec![d4()], &policy).unwrap();
    assert_eq!(results.total, 4);
    
    
    test_results_exactly(&results, &symbols, 1, 0.25);
    test_results_exactly(&results, &symbols, 2, 0.25);
    test_results_exactly(&results, &symbols, 3, 0.25);
    test_results_exactly(&results, &symbols, 4, 0.25);
}

#[test]
fn two_d4s() {
    let symbols = d4().unique_symbols();
    let policy = RollCollectionPolicy::collect_all(&symbols);
    let results = RollProbabilities::new(&vec![ d4(), d4()], &policy).unwrap();
    assert_eq!(results.total, 16);
    
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
    let symbols = d4().unique_symbols();
    let policy = RollCollectionPolicy::collect_all(&symbols);
    let results = RollProbabilities::new(&vec![ d4(), d8() ], &policy).unwrap();
    assert_eq!(results.total, 32);
    
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
    let symbols = d4().unique_symbols();
    let policy = RollCollectionPolicy::collect_all(&symbols);
    let results = RollProbabilities::new(&vec![ d4(), d4(), d4() ], &policy).unwrap();
        
    assert_eq!(results.total, 4*4*4);
    test_results_exactly(&results, &symbols, 7, 0.1875);
}

#[test]
fn four_through_ten() {
    let symbols = d4().unique_symbols();
    let policy = RollCollectionPolicy::collect_all(&symbols);
    let results = RollProbabilities::new(&vec![ d4(), d6(), d8(), d10() ], &policy).unwrap();

    assert_eq!(results.total, 4*6*8*10);
}

#[test]
// anydice.com
// output [highest 2 of 3d4]
fn three_d4s_highest_two() {
    let symbols = d4().unique_symbols();
    let policy = RollCollectionPolicy::take_highest_n_of(2, &symbols);
    let results = RollProbabilities::new(&vec![ d4(), d4(), d4() ], &policy).unwrap();

    assert_eq!(results.total, 4*4*4);
    test_results_exactly(&results, &symbols, 2, 0.015625);
    test_results_exactly(&results, &symbols, 3, 0.046875);
    test_results_exactly(&results, &symbols, 4, 0.109375);
    test_results_exactly(&results, &symbols, 5, 0.1875);
    test_results_exactly(&results, &symbols, 6, 0.25);
    test_results_exactly(&results, &symbols, 7, 0.234375);
    test_results_exactly(&results, &symbols, 8, 0.15625);
}

#[test]
// anydice.com
// output [lowest 2 of 3d4]
fn three_d4s_lowest_two() {
    let symbols = d4().unique_symbols();
    let policy = RollCollectionPolicy::take_lowest_n_of(2, &symbols);
    let results = RollProbabilities::new(&vec![ d4(), d4(), d4() ], &policy).unwrap();

    assert_eq!(results.total, 4*4*4);
    test_results_exactly(&results, &symbols, 2, 0.15625);
    test_results_exactly(&results, &symbols, 3, 0.234375);
    test_results_exactly(&results, &symbols, 4, 0.25);
    test_results_exactly(&results, &symbols, 5, 0.1875);
    test_results_exactly(&results, &symbols, 6, 0.109375);
    test_results_exactly(&results, &symbols, 7, 0.046875);
    test_results_exactly(&results, &symbols, 8, 0.015625);
}

#[test]
// anydice.com
// output [lowest 1 of 3d4]
fn three_d4s_remove_highest_two() {
    let symbols = d4().unique_symbols();
    let policy = RollCollectionPolicy::remove_highest_n_of(2, &symbols);
    let results = RollProbabilities::new(&vec![ d4(), d4(), d4() ], &policy).unwrap();

    assert_eq!(results.total, 4*4*4);
    test_results_exactly(&results, &symbols, 1, 0.578125);
    test_results_exactly(&results, &symbols, 2, 0.296875);
    test_results_exactly(&results, &symbols, 3, 0.109375);
    test_results_exactly(&results, &symbols, 4, 0.015625);
}

#[test]
// anydice.com
// output [highest 1 of 3d4]
fn three_d4s_remove_lowest_two() {
    let symbols = d4().unique_symbols();
    let policy = RollCollectionPolicy::remove_lowest_n_of(2, &symbols);
    let results = RollProbabilities::new(&vec![ d4(), d4(), d4() ], &policy).unwrap();

    assert_eq!(results.total, 4*4*4);
    test_results_exactly(&results, &symbols, 1, 0.015625);
    test_results_exactly(&results, &symbols, 2, 0.109375);
    test_results_exactly(&results, &symbols, 3, 0.296875);
    test_results_exactly(&results, &symbols, 4, 0.578125);
}


#[test]
fn one_d4_compare_two_d4() {
    let symbols = d4().unique_symbols();
    let policy = RollCollectionPolicy::collect_all(&symbols);
    let results1 = RollProbabilities::new(&vec![ d4()], &policy).unwrap();
    let results2 = RollProbabilities::new(&vec![ d4(), d4()], &policy).unwrap();

    let compare = results1.roll_against(&results2);

    assert_eq!(compare.win_odds(), 4.0/64.0);
    assert_eq!(compare.tie_odds(), 6.0/64.0);
    assert_eq!(compare.loss_odds(), 54.0/64.0);
}

#[test]
fn two_d4_compare_two_d4() {
    let symbols = d4().unique_symbols();
    let policy = RollCollectionPolicy::collect_all(&symbols);
    let results1 = RollProbabilities::new(&vec![ d4(), d4()], &policy).unwrap();
    let results2 = RollProbabilities::new(&vec![ d4(), d4()], &policy).unwrap();

    let compare = results1.roll_against(&results2);

    assert_eq!(compare.win_odds(), 106.0/256.0);
    assert_eq!(compare.tie_odds(), 44.0/256.0);
    assert_eq!(compare.loss_odds(), 106.0/256.0);
}

#[test]
fn one_d8_compare_two_d4() {
    let symbols = d4().unique_symbols();
    let policy = RollCollectionPolicy::collect_all(&symbols);
    let results1 = RollProbabilities::new(&vec![ d8()], &policy).unwrap();
    let results2 = RollProbabilities::new(&vec![ d4(), d4()], &policy).unwrap();

    let compare = results1.roll_against(&results2);

    assert_eq!(compare.win_odds(), 48.0/128.0);
    assert_eq!(compare.tie_odds(), 16.0/128.0);
    assert_eq!(compare.loss_odds(), 64.0/128.0);
}

#[test]
fn two_custom_d4_multiple_targets() {
    let a_symbol = DieSymbol::new("A").unwrap();
    let b_symbol = DieSymbol::new("B").unwrap();
    let both_symbols = vec![ a_symbol.clone(), b_symbol.clone() ];
    let sides = vec![
        DieSide::new(vec![ a_symbol.clone() ] ),
        DieSide::new(vec![ b_symbol.clone() ] ),
        DieSide::new(vec![ a_symbol.clone(), b_symbol.clone() ] ),
        DieSide::new(vec![ ] )
    ];
    let custom_d4 = Die::new(sides).unwrap();
    let policy = RollCollectionPolicy::collect_all(&both_symbols);
    let results = RollProbabilities::new(&vec![ custom_d4.clone(), custom_d4.clone() ], &policy).unwrap();

    let a_symbol_vec = vec![ a_symbol.clone() ];
    let b_symbol_vec = vec![ b_symbol.clone() ];

    let target_exactly_one_a = RollTarget::exactly_n_of(1, &a_symbol_vec);
    let target_at_least_one_b = RollTarget::at_least_n_of(1, &b_symbol_vec);

    assert_eq!(results.total, 4*4);
    let results_exactly_one_a = results.get_odds(&vec![target_exactly_one_a.clone()]);
    assert_eq!(results_exactly_one_a, 8.0/16.0);
    let results_at_least_one_b = results.get_odds(&vec![target_at_least_one_b.clone()]);
    assert_eq!(results_at_least_one_b, 12.0/16.0);
    let results_exactly_one_a_and_at_least_one_b = results.get_odds(&vec![target_exactly_one_a, target_at_least_one_b]);
    assert_eq!(results_exactly_one_a_and_at_least_one_b, 6.0/16.0);
}