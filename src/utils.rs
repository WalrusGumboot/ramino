//! Handy functions

use crate::card::{Card, Suit::*, CardType::*};
use rand::{thread_rng, seq::SliceRandom};
/// Generates a deck of 52 normal cards and 2 jokers in standard order.
///
/// If shuffled is true, shuffles the deck before returning it.
pub fn generate_single_deck(shuffled: bool) -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();

    for suit in [Spades, Hearts, Diamonds, Clubs] {
        deck.push(Card::new(Ace, suit));
        deck.push(Card::new(King, suit));
        deck.push(Card::new(Queen, suit));
        deck.push(Card::new(Jack, suit));
        for i in 2..11 { deck.push(Card::new(Number(i), suit)); }
    }

    for _ in 0..2 { deck.push(Card::new(Joker, JokerSuit)) }

    if shuffled {
        deck.shuffle(&mut thread_rng());
    }

    deck
}

/// Generates the full playing deck, consisting of two normal decks.
pub fn generate_deck(shuffled: bool) -> Vec<Card> {
    let mut deck = generate_single_deck(shuffled);
    deck.append(&mut generate_single_deck(shuffled));

    deck
}
