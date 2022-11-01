#![allow(missing_docs)]

#![cfg(test)]
use crate::{card::*, hand::*, utils::*, run::*};

#[test]
fn get_deck() {
    let deck = generate_single_deck(false);
    assert_eq!(deck.len(), 54);
}

#[test]
fn shuffled_deck() {
    let deck = generate_single_deck(true);
    println!("{:?}", deck[0]);

    // this test always passes, but it can be
    // individually verified that it does, in fact, function
}

#[test]
fn full_deck() {
    let deck = generate_deck(false);
    assert_eq!(deck.len(), 108);
}

#[test]
fn generate_hand() {
    let mut deck = generate_deck(true);
    let _hand = Hand::draw(&mut deck);
}

#[test]
fn hand_score() {
    let hand0 = Hand(vec![]);
    assert_eq!(hand0.score(), 0);

    let hand1 = Hand(vec![Card::from("SA")]);
    assert_eq!(hand1.score(), 1);

    let hand2 = Hand(vec![Card::from("SK"), Card::from("HQ"), Card::from("DJ")]);
    assert_eq!(hand2.score(), 30);

    let hand3 = Hand(vec![Card::from("S8"), Card::from("S9"), Card::from("J")]);
    assert_eq!(hand3.score(), 42);
}

#[test]
fn verify_runs() {
    // let mut test_cards1 = vec![Card::new(Joker, JokerSuit), Card::new(Queen, Spades), Card::new(King, Spades)];
    // test_cards1.sort();
    // assert_eq!(verify_run(test_cards1.clone()),
    //            Ok(Run::Ascending(test_cards1.clone())));

    let mut test_cards2 = vec!["S2", "S3", "SA"].iter().map(|s| Card::from(*s)).collect::<Vec<_>>();
    test_cards2.sort();
    assert_eq!(verify_run(test_cards2.clone()),
                Ok(Run::Ascending(test_cards2.clone())));

    let mut test_cards3 = vec!["S2", "D2", "H2"].iter().map(|s| Card::from(*s)).collect::<Vec<_>>();
    test_cards3.sort();
    assert_eq!(verify_run(test_cards3.clone()),
                Ok(Run::Equal(test_cards3.clone())));

    let test_cards4 = vec!["S2", "D2", "H3"].iter().map(|s| Card::from(*s)).collect::<Vec<_>>();
    assert_eq!(verify_run(test_cards4.clone()),
                Err(()));
}
